use device_query::{DeviceQuery, DeviceState, Keycode};
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time;
use tetris::r#struct::Game;
use tetris::get_keys;
use tetris::r#struct::GameState::{GameOver, Operating, Waiting};

fn main() {
    let mut current_game = Game::new();
    let device_state = DeviceState::new();

    loop {
        let (tx, rx) = mpsc::channel::<bool>();
        println!("{}", current_game);
        thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(current_game.duration));
            tx.send(true).unwrap();
        });
        // let mut keys = device_state.get_keys();
        let mut keys;
        loop {
            keys = device_state.get_keys();
            match rx.try_recv() {
                Ok(true) => {
                    break;
                }
                _ => continue,
            }
        }
        match current_game.state {
            Operating(id) => {
                if keys.is_empty() {
                } else {
                    match keys[keys.len() - 1] {
                        Keycode::Right => {
                            current_game.right(id);
                        }
                        Keycode::Left => {
                            current_game.left(id);
                        }
                        Keycode::Up => match id {
                            0 => {}
                            1 => current_game.ratate_i(id),
                            _ => current_game = current_game.rotate_std(),
                        },
                        Keycode::Down => {
                            current_game.head_drop();
                        }
                        _ => {}
                    }
                }
                current_game.frame += 1;
            }
            // Delaying(id) => {
            //     // ^TODO 遊び時間の実装
            // }
            Waiting => {
                current_game.generate_tetromino();
                current_game.frame += 1;
            }
            GameOver => {
                println!("GAME OVER");
                process::exit(0);
            }
        }
        if current_game.frame % 6 == 0 {
            current_game.fall_blocks();
        }
        current_game.clear_line();
    }
}
