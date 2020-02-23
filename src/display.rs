use crate::r#struct::Game;

use colored::Colorize;
use std::fmt;

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s = format!("{}\n{}", "Tetris", s);
        for i in 0..20 {
            s = format!("{}|", s);
            for j in 0..10 {
                match self.get_mino_id(i, j) {
                    None => {
                        s = format!("{} ", s);
                    }
                    Some(id) => {
                        // 0..=6 ミノごとに色付け
                        match id {
                            0 => {
                                s = format!("{}{}", s, Colorize::cyan("■"));
                            }
                            1 => {
                                s = format!("{}{}", s, Colorize::yellow("■"));
                            }
                            2 => {
                                s = format!("{}{}", s, Colorize::red("■"));
                            }
                            3 => {
                                s = format!("{}{}", s, Colorize::green("■"));
                            }
                            4 => {
                                s = format!("{}{}", s, Colorize::blue("■"));
                            }
                            5 => {
                                s = format!("{}{}", s, Colorize::magenta("■"));
                            }
                            6 => {
                                s = format!("{}{}", s, Colorize::bright_purple("■"));
                            }
                            _ => {}
                        }
                    }
                }
            }
            s = format!("{}|\n", s);
        }
        s = format!(
            "{}\nSCORE = {} TIME = {} LVL = {}",
            s, self.score, self.frame, self.level
        );
        write!(f, "{}", s)
    }
}
