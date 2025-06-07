use std::collections::HashMap;

use ort::{tensor::OrtOwnedTensor, Environment, SessionBuilder, Value};
use ndarray::{Array3, Axis, CowArray, IxDyn};
use crate::engine::{board::board::Board, definitions::{Castling, NrOf, Piece, Side, SQUARE_BITBOARDS}, evaluator::evaluator::Evaluator};

pub struct CNNEvaluator {
    session: ort::Session,
    piece_channels: HashMap<char, usize>,
}

impl CNNEvaluator {
    pub fn new(onnx_model_path: &str) -> Result<Self, String> {
        let environment = std::sync::Arc::new(
            Environment::builder()
                .with_name("depthwise-cnn-eval")
                .build()
                .map_err(|e| e.to_string())?
        );

        let session = SessionBuilder::new(&environment)
            .map_err(|e| e.to_string())?
            .with_model_from_file(onnx_model_path)
            .map_err(|e| e.to_string())?;

        let piece_channels = HashMap::from([
            ('P', 0),
            ('N', 1),
            ('B', 2),
            ('R', 3),
            ('Q', 4),
            ('K', 5),
            ('p', 6),
            ('n', 7),
            ('b', 8),
            ('r', 9),
            ('q', 10),
            ('k', 11),
        ]);

        Ok(CNNEvaluator {
            session,
            piece_channels,
        })
    }


    fn encode_board(&self, board: &Board) -> Array3<f32> {
        let mut planes = Array3::<f32>::zeros((14, 8, 8));

        for i in 0..NrOf::SQUARES {
            let piece = board.piece_list[i];

            if piece != Piece::None {
                let mut piece_char = piece.to_string()
                    .chars()
                    .next()
                    .unwrap_or(' ');

                let square_bitboard = SQUARE_BITBOARDS[i as usize];
                let black_occupancy = board.get_side_occupancy(Side::Black);

                if black_occupancy & square_bitboard != 0 {
                    piece_char = piece_char.to_ascii_lowercase();
                }

                let channel = self.piece_channels.get(&piece_char).unwrap();

                let rank = i / NrOf::FILES;
                let file = i % NrOf::FILES;
                planes[[*channel, rank, file]] = 1.0;
            }
        }

        let castling_rights = board.game_state.castling;
        if castling_rights & Castling::WhiteKing as u8 > 0 {
            planes[[12, 0, 0]] = 1.0;
        }
        if castling_rights & Castling::WhiteQueen as u8 > 0 {
            planes[[12, 0, 1]] = 1.0;
        }
        if castling_rights & Castling::BlackKing as u8 > 0 {
            planes[[12, 1, 0]] = 1.0;
        }
        if castling_rights & Castling::BlackQueen as u8 > 0 {
            planes[[12, 1, 1]] = 1.0;
        }

        if let Some(ep_square) = board.game_state.en_passant {
            let rank = (ep_square as usize) / NrOf::FILES;
            let file = (ep_square as usize) % NrOf::FILES;
            planes[[13, rank, file]] = 1.0;
        }

        planes
    }

}

impl Evaluator for CNNEvaluator {
    fn evaluate_board(&self, board: &Board) -> f32 {
        let input_tensor: Array3<f32> = self.encode_board(board);

        let batched = input_tensor.insert_axis(Axis(0));
        let cow_input: CowArray<f32, IxDyn> = CowArray::from(batched.into_dyn());

        let input = Value::from_array(self.session.allocator(), &cow_input)
                                                    .unwrap();

        let outputs = self
            .session
            .run(vec![input]).unwrap();


        let output_tensor: OrtOwnedTensor<f32, IxDyn> = outputs[0]
        .try_extract()
        .map_err(|e| format!("Failed to extract output tensor: {e}")).unwrap();

        let view = output_tensor.view();
        let value = *view
            .iter()
            .next().unwrap();

        value
    }
}