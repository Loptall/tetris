/// 各マスの状態のマーカー
#[derive(Copy, Clone, PartialEq)]
pub enum BlockState {
    Static(u16),
    Dynamic(u16),
    Ghost,
    Nothing,
}

/// 現在の操作状況
#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    Operating(u16),
    Waiting,
    GameOver,
}

/// 盤面と操作の情報を保持する構造体
#[derive(Copy, Clone)]
pub struct Game {
    pub screen: [[BlockState; 10]; 20],
    pub state: GameState,
    pub score: usize,
    pub frame: usize,
    pub level: usize,
    pub duration: u64,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
