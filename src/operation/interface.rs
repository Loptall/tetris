use crate::r#struct::{BlockState::*, Game, GameState::*};

/// 実装
impl Game {
    /// 内部関数
    pub fn remove_all_dynamic(&mut self) {
        for line in 0..20 {
            for e in 0..10 {
                if let Dynamic(_) = self.screen[line][e] {
                    self.screen[line][e] = Nothing;
                }
            }
        }
    }

    pub fn rotate_std(&mut self) -> Self {
        let mut rectangele = Vec::<(usize, usize)>::new();
        let mut circle = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (2, 1),
            (2, 0),
            (1, 0),
        ]
        .repeat(2);
        for line in 0..20 {
            for e in 0..10 {
                if let Dynamic(_) = self.screen[line][e] {
                    rectangele.push((line, e));
                }
            }
        }
        let (mut left, mut up) = Game::find_max_rectangle(&rectangele);
        if left >= 8 {
            left = 7;
        }
        if up >= 18 {
            up = 17;
        }
        for (i, j) in &mut circle {
            *i += up;
            *j += left;
        }
        let mut pre_changes = *self;
        pre_changes.remove_all_dynamic();
        for i in 0..8 {
            let (x1, y1) = (circle[i].0, circle[i].1);
            let (x2, y2) = (circle[i + 2].0, circle[i + 2].1);
            match self.screen[x1][y1] {
                Dynamic(id) => {
                    if let Static(_) = self.screen[x2][y2] {
                        return *self;
                    }
                    pre_changes.screen[x2][y2] = Dynamic(id);
                }
                Static(_) => {}
                _ => {
                    if let Dynamic(_) = self.screen[x2][y2] {
                        pre_changes.screen[x2][y2] = Nothing;
                    }
                }
            }
        }
        pre_changes.screen[circle[0].0 + 1][circle[0].1 + 1] =
            self.screen[circle[0].0 + 1][circle[0].1 + 1];

        pre_changes
    }

    pub fn ratate_i(&mut self, id: u16) {
        let mut count = 0;
        let mut last = (0, 0);
        for line in 0..20 {
            let mut a = 0;
            for e in 0..10 {
                if let Dynamic(_) = self.screen[line][e] {
                    a += 1;
                    last.0 = line;
                    last.1 = e;
                }
            }
            count = count.max(a);
        }
        match count {
            1 => {
                if last.1 > 2 && last.1 < 9 {
                    self.remove_all_dynamic();
                    for i in 0..4 {
                        self.screen[last.0][last.1 - i + 1] = Dynamic(id);
                    }
                } else if last.1 <= 2 {
                    self.remove_all_dynamic();
                    for i in 0..4 {
                        self.screen[last.0][i] = Dynamic(id);
                    }
                } else {
                    self.remove_all_dynamic();
                    for i in 6..10 {
                        self.screen[last.0][i] = Dynamic(id);
                    }
                }
            }
            4 => {
                if last.0 < 3 {
                    return;
                }
                self.remove_all_dynamic();
                for i in 0..4 {
                    self.screen[last.0 - i][last.1 - 1] = Dynamic(id);
                }
            }
            _ => {}
        }
    }

    pub fn right(&mut self, id: u16) {
        for i in 0..20 {
            for j in 0..9 {
                if let Dynamic(_) = self.screen[i][j] {
                    if let Static(_) = self.screen[i][j + 1] {
                        return;
                    }
                }
            }
        }
        if self.screen.iter().filter(|l| l[9] == Dynamic(id)).count() == 0 {
            for line in self.screen.iter_mut() {
                let static_clone_line = *line;
                for i in (0..=9).rev() {
                    if static_clone_line[i] == Dynamic(id) {
                        line[i] = Nothing;
                        line[i + 1] = Dynamic(id);
                    }
                }
            }
        }
    }

    pub fn left(&mut self, id: u16) {
        for i in 0..20 {
            for j in 1..10 {
                if let Dynamic(_) = self.screen[i][j] {
                    if let Static(_) = self.screen[i][j - 1] {
                        return;
                    }
                }
            }
        }
        if self.screen.iter().filter(|l| l[0] == Dynamic(id)).count() == 0
        // todo! 左がStaticだったときにブロック
        {
            for line in self.screen.iter_mut() {
                let static_clone_line = *line;
                for i in 0..10 {
                    if static_clone_line[i] == Dynamic(id) {
                        line[i] = Nothing;
                        line[i - 1] = Dynamic(id);
                    }
                }
            }
        }
    }

    pub fn head_drop(&mut self) {
        while self.will_fall() {
            self.fall_blocks();
        }
    }

    /// 1列揃ったら消す
    pub fn clear_line(&mut self) {
        if let Waiting = self.state {
            let point = vec![40, 100, 300, 1200];
            let mut line_to_clear = Vec::<usize>::new();
            for line in 0..20 {
                for e in 0..10 {
                    if let Static(_) = self.screen[line][e] {
                    } else {
                        break;
                    }
                    if e == 9 {
                        line_to_clear.push(line);
                    }
                }
            }
            if line_to_clear.is_empty() {
                return;
            }
            line_to_clear.sort();
            self.score += point[line_to_clear.len() - 1];
            self.lvl_update();
            if !line_to_clear.is_empty() {
                for line in line_to_clear {
                    for upper in (0..line).rev() {
                        self.screen[upper + 1] = self.screen[upper];
                    }
                }
            }
        }
    }

    fn find_max_rectangle(v: &[(usize, usize)]) -> (usize, usize) {
        let up = v.iter().map(|(x, _)| x).min().unwrap();
        let left = v.iter().map(|(_, y)| y).min().unwrap();
        (*left, *up)
    }

    fn lvl_update(&mut self) {
        self.level = self.score / 500 + 1;
        self.duration = 100 + self.level as u64 * 10;
    }
}
