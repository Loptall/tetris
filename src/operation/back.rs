use crate::r#struct::{
    BlockState::{Dynamic, Nothing, Static},
    Game,
    GameState::{GameOver, Operating, Waiting},
};
use rand::random;

impl Game {
    /// 新しいインスタンスを生成
    pub fn new() -> Game {
        Game {
            screen: [[Nothing; 10]; 20],
            state: Waiting,
            score: 0usize,
            frame: 0usize,
            level: 1usize,
            duration: 100u64,
        }
    }

    /// テトロミノを生成する
    pub fn generate_tetromino(&mut self) {
        let random_number = random::<u16>() % 7;
        let mut place = Vec::new();
        match random_number {
            0 => {
                place = vec![(0, 4), (0, 5), (1, 4), (1, 5)];
            }
            1 => {
                place = vec![(0, 3), (0, 4), (0, 5), (0, 6)];
            }
            2 => {
                place = vec![(0, 4), (0, 5), (1, 5), (1, 6)];
            }
            3 => {
                place = vec![(0, 5), (0, 6), (1, 4), (1, 5)];
            }
            4 => {
                place = vec![(0, 4), (1, 4), (1, 5), (1, 6)];
            }
            5 => {
                place = vec![(0, 6), (1, 4), (1, 5), (1, 6)];
            }
            6 => {
                place = vec![(0, 4), (1, 3), (1, 4), (1, 5)];
            }
            _ => {}
        }
        self.checked_generate(&place, random_number);
        if self.state == Waiting {
            self.state = Operating(random_number);
        }
    }

    /// #private
    /// 生成時にゲームオーバーになっていないか
    /// 確認した上でテトロミノを生成
    fn checked_generate(&mut self, arg: &[(usize, usize)], rand: u16) {
        for (i, j) in arg {
            let t = self.get_mino_id(*i, *j);
            match t {
                Some(_) => {
                    self.state = GameOver;
                    break;
                }
                None => {
                    self.screen[*i][*j] = Dynamic(rand);
                }
            }
        }
    }

    /// どのミノを操作しているかをOptionで包んで返す
    pub fn get_mino_id(&self, i: usize, j: usize) -> Option<u16> {
        match self.screen[i][j] {
            Dynamic(n) => Some(n),
            Static(n) => Some(n),
            _ => None,
        }
    }

    pub fn will_fall(&self) -> bool {
        let mut will_fall = true;
        'a: for line in 0..19 {
            for e in 0..=9 {
                if let Dynamic(_) = self.screen[line][e] {
                    match self.screen[line + 1][e] {
                        Static(_) => {
                            will_fall = false;
                            break 'a;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }
        for last_line in self.screen.last().unwrap() {
            match *last_line {
                Dynamic(_) => {
                    will_fall = false;
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
        will_fall
    }

    /// 毎フレームブロックを下げる
    pub fn fall_blocks(&mut self) {
        if let Operating(_) = self.state {
            if self.will_fall() {
                let static_clone = self.screen;
                self.remove_all_dynamic();
                for i in (0..19).rev() {
                    for j in 0..10 {
                        if let Dynamic(id) = static_clone[i][j] {
                            self.screen[i + 1][j] = Dynamic(id);
                        }
                    }
                }
            } else {
                for line in 0..20 {
                    for e in 0..10 {
                        if let Dynamic(id) = self.screen[line][e] {
                            self.screen[line][e] = Static(id);
                        }
                    }
                }
                self.state = Waiting;
            }
        }
    }
}
