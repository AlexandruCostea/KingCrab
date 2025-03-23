use super::{definitions::MAX_GAME_MOVES, game_state::GameState};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GameHistory {
    list: [GameState; MAX_GAME_MOVES],
    count: usize,
}


impl GameHistory {
    pub fn new() -> Self {
        GameHistory {
            list: [GameState::new(); MAX_GAME_MOVES],
            count: 0,
        }
    }


    pub fn push(&mut self, new_state: GameState) {
        self.list[self.count] = new_state;
        self.count += 1;
    }


    pub fn pop(&mut self) -> Option<GameState> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.list[self.count])
        } else {
            None
        }
    }


    pub fn get_ref(&self, index: usize) -> &GameState {
        &self.list[index]
    }


    pub fn len(&self) -> usize {
        self.count
    }


    pub fn clear(&mut self) {
        self.count = 0;
    }
}