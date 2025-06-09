use std::{collections::{HashMap, HashSet}, path::Path};

use ndarray_npy::read_npy;
use ort::{tensor::OrtOwnedTensor, Environment, SessionBuilder, Value};
use ndarray::{Array2, CowArray, IxDyn};

use crate::engine::{board::board::Board,
    definitions::{NrOf, Piece, Side, SQUARE_BITBOARDS},
    evaluator::evaluator::Evaluator};


struct HalfKACache {
    own_indices: HashSet<usize>,
    opp_indices: HashSet<usize>,
    own_sum: Array2<f32>,
    opp_sum: Array2<f32>,
}

impl HalfKACache {
    fn new() -> Self {
        HalfKACache {
            own_indices: HashSet::new(),
            opp_indices: HashSet::new(),
            own_sum: Array2::<f32>::zeros((1, 520)),
            opp_sum: Array2::<f32>::zeros((1, 520)),
        }
    }

    fn is_empty(&self) -> bool {
        self.own_indices.is_empty() || self.opp_indices.is_empty()
    }
}

pub struct HalfkaEvaluator {
    own_embeddings: Array2<f32>,
    opp_embeddings: Array2<f32>,

    input_session: ort::Session,
    bucket_sessions: Vec<ort::Session>,

    piece_indices: HashMap<char, usize>,
    cache: HalfKACache
}

impl HalfkaEvaluator {
    pub fn new(model_dir: &str) -> Result<Self, String> {
        let environment = std::sync::Arc::new(
            Environment::builder()
                .with_name("halfka-eval")
                .build()
                .map_err(|e| e.to_string())?
        );

        let input_path = Path::new(model_dir)
                                    .join("halfka_input_processor.onnx");
        let input_session = SessionBuilder::new(&environment)
            .map_err(|e| e.to_string())?
            .with_model_from_file(input_path)
            .map_err(|e| e.to_string())?;

        let mut bucket_sessions = Vec::with_capacity(8);
        for i in 0..8 {
            let bucket_path = Path::new(model_dir)
                                        .join(format!("halfka_bucket_evaluator_{i}.onnx"));
            let session = SessionBuilder::new(&environment)
                .map_err(|e| e.to_string())?
                .with_model_from_file(bucket_path)
                .map_err(|e| e.to_string())?;
            bucket_sessions.push(session);
        }

        let own_embeddings_path = Path::new(model_dir)
                                            .join("halfka_embeddings_own.npy");
        let opp_embeddings_path = Path::new(model_dir)
                                            .join("halfka_embeddings_opp.npy");

        let embedding_own: Array2<f32> = read_npy(&own_embeddings_path)
            .map_err(|e| format!("Failed to read {:?}: {}", own_embeddings_path, e))?;

        let embedding_opp: Array2<f32> = read_npy(&opp_embeddings_path)
            .map_err(|e| format!("Failed to read {:?}: {}", opp_embeddings_path, e))?;

        let piece_indices = HashMap::from([
            ('P', 0), ('N', 1), ('B', 2), ('R', 3), ('Q', 4),
            ('p', 5), ('n', 6), ('b', 7), ('r', 8), ('q', 9), 
            ('K', 10), ('k', 10),
        ]);

        Ok(HalfkaEvaluator {
            own_embeddings: embedding_own,
            opp_embeddings: embedding_opp,
            input_session,
            bucket_sessions,
            piece_indices,
            cache: HalfKACache::new(),
        })
    }

    fn vertical_flip(square: usize) -> usize {
        let file = square % NrOf::FILES;
        let rank = 7 - (square / NrOf::FILES);
        rank * NrOf::FILES + file
    }

    fn feature_index(
        piece_idx: usize,
        piece_sq: usize,
        king_sq: usize) -> Option<usize> {
        if piece_sq == king_sq {
            None
        } else {
            Some(piece_idx * 64 * 32 + piece_sq * 32 + king_sq)
        }
    }

    pub fn compute_halfka_indices(
        &self,
        board: &Board,
        side: Side) -> Vec<usize> {
        let mut indices = Vec::new();

        let mut king_sq = board.get_king_square(side) as usize;
        let king_rank = king_sq / NrOf::FILES;

        let flip = king_rank >= 4;
        king_sq = if flip {
            Self::vertical_flip(king_sq)
        } else {
            king_sq
        };

        let side_occupancy = board.get_side_occupancy(side);

        for i in 0..NrOf::SQUARES {
            let piece = board.piece_list[i];
            if piece == Piece::None {
                continue;
            }

            let square_bitboard = SQUARE_BITBOARDS[i as usize];
            if side_occupancy & square_bitboard != 0 && piece== Piece::King {
                continue;
            }

            let piece_char = piece.to_string().chars()
                                    .next().unwrap_or(' ');
            let piece_idx = *self.piece_indices
                                    .get(&piece_char).unwrap_or(&0);
            let piece_square = if flip {
                Self::vertical_flip(i as usize)
            } else {
                i as usize
            };

            let feature_idx = Self::feature_index(
                                            piece_idx,
                                            piece_square,
                                            king_sq);
            if let Some(idx) = feature_idx {
                indices.push(idx);
            }
        }

        indices
    }

    fn sum_embedding(
        &self,
        embedding: &Array2<f32>,
        indices: &[usize]) -> Array2<f32> {
        let mut sum = Array2::<f32>::zeros((1, 520));
        for &i in indices {
            let result = &sum.row(0) + &embedding.row(i);
            sum.row_mut(0).assign(&result);
        }
        sum
    }

}


impl Evaluator for HalfkaEvaluator {
    fn evaluate_board(&mut self, board: &Board) -> f32 {

        let active_side = board.get_active_side();
        let opp_side = board.get_opponent();

        let own_indices = self.compute_halfka_indices(board, active_side);
        let opp_indices = self.compute_halfka_indices(board, opp_side);

        let new_own_set: HashSet<usize> = HashSet::from_iter(
                                            own_indices.iter().cloned());
        let new_opp_set: HashSet<usize> = HashSet::from_iter(
                                            opp_indices.iter().cloned());

        if self.cache.is_empty() {
            for idx in &new_own_set {
                self.cache.own_indices.insert(*idx);
            }

            for idx in &new_opp_set {
                self.cache.opp_indices.insert(*idx);
            }

            self.cache.own_sum = self.sum_embedding(&self.own_embeddings, &own_indices);
            self.cache.opp_sum = self.sum_embedding(&self.opp_embeddings, &opp_indices);
        } else {
            let added: Vec<usize> = new_own_set.difference(&self.cache.own_indices)
                                    .cloned().collect();
            let removed: Vec<usize> = self.cache.own_indices.difference(&new_own_set)
                                    .cloned().collect();

            for &i in &added {
                self.cache.own_sum += &self.own_embeddings.row(i);
            }
            for &i in &removed {
                self.cache.own_sum -= &self.opp_embeddings.row(i);
            }
            self.cache.own_indices = new_own_set;


            let added: Vec<usize> = new_opp_set.difference(&self.cache.opp_indices)
                                        .cloned().collect();
            let removed: Vec<usize> = self.cache.opp_indices.difference(&new_opp_set)
                                        .cloned().collect();

            for &i in &added {
                self.cache.opp_sum += &self.opp_embeddings.row(i);
            }
            for &i in &removed {
                self.cache.opp_sum -= &self.opp_embeddings.row(i);
            }
            self.cache.opp_indices = new_opp_set;
        }

        let own_sum = &self.cache.own_sum;
        let opp_sum = &self.cache.opp_sum;

        let input_own = CowArray::from(own_sum.clone().into_dyn());
        let input_opp = CowArray::from(opp_sum.clone().into_dyn());

        let value_own = Value::from_array(
                self.input_session.allocator(),
                        &input_own).unwrap();
        let value_opp = Value::from_array(
                self.input_session.allocator(),
                        &input_opp).unwrap();

        let outputs = self.input_session
                                            .run(vec![value_own, value_opp])
                                            .unwrap();

        let x_1024_dyn: OrtOwnedTensor<f32, IxDyn> = outputs[0].try_extract().unwrap();
        let avg_score_dyn: OrtOwnedTensor<f32, IxDyn> = outputs[1].try_extract().unwrap();

        let x_1024 = x_1024_dyn.view().to_owned()
                                                    .into_dimensionality::<ndarray::Ix2>()
            .expect("Expected (1, 1024) array");

        let avg_score = avg_score_dyn.view().to_owned()
                                                    .into_dimensionality::<ndarray::Ix2>()
            .expect("Expected (1, 1) array");


        let piece_count = own_indices.len() + 1;
        let bucket_index = ((piece_count - 1) / 4).min(7);

        let input_x_val = CowArray::from(x_1024.into_dyn());
        let input_avg_val = CowArray::from(avg_score.into_dyn());
        let x_val = Value::from_array(self.bucket_sessions[bucket_index]
                                    .allocator(),&input_x_val).unwrap();
        let avg_val = Value::from_array(self.bucket_sessions[bucket_index]
                                    .allocator(),&input_avg_val).unwrap();

        let result = self.bucket_sessions[bucket_index]
                                            .run(vec![x_val, avg_val]).unwrap();
        let output_tensor: OrtOwnedTensor<f32, IxDyn> = result[0].try_extract().unwrap();

        *output_tensor.view().iter().next().unwrap()
    }
}