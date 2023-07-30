use std::{env, io::Write};
use MS::*;

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
    
    // let args: Vec<String> = env::args().collect();
    // let config = GameConfig::init(&args)
    //         .unwrap_or_else(|err| {
    //             println!("Problem parsing arguments: {}", err);
    //             std::process::exit(2);
    //         });
    // let mut game = Game::init(config)
    //         .unwrap_or_else(|err:&str | {
    //             println!("Problem initializing game: {}", err);
    //             std::process::exit(2);
    //         });
    // while game.status == GameStatus::InProgress {
    //     let mut buffer = std::io::stdout();
    //     game.show(&mut buffer);
    //     buffer.flush().unwrap();
    //     let input: Input = match input(std::io::stdin().lock()) {
    //         Ok(input) => input,
    //         Err(s) => {
    //             println!("Problem reading input: {}", s);
    //             println!("Please try again.");
    //             continue;
    //         },
    //     };
    //     match game.try_update(&input) {
    //         Ok(_) => (),
    //         Err(err) => {
    //             println!("Problem trying move: {}", err);
    //             println!("Please try again.");
    //         },
    //     }
    // }
    // if game.status == GameStatus::Won {
    //     win();
    // } else {
    //     lose();
    // }
}

#[tauri::command]
fn greet(name: &str) -> String {
    let mut greet_line: String = "Hello, !".to_string();
    greet_line.insert_str(6, name);
    greet_line
}