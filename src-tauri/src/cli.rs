use tauri::Manager;

use crate::{
    GameConfig,
    Game,
    GameStatus,
    Input,
    win,
    lose,
};

#[derive(Clone, serde::Serialize)]
pub struct Parameters {
    content: String,
}
impl Parameters {
    pub fn new(content: String) -> Self {
        Self {
            content: content,
        }
    }
}
#[tauri::command]
pub fn start(width: String, height: String, mines: String) -> Result<(), &'static str> {
    let args: Vec<String> = vec![width, height, mines];
    let config = GameConfig::init(&args)?;
    let mut game = Game::init(config)?;
    while game.status == GameStatus::InProgress {
        /*
            Here an async method where each move is waiting to be
            triggered by the frontend side.
         */
    }
    Ok(())
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn display(n: usize) -> Vec<String> {
    vec![String::from("&#128681;"); n]
}

pub fn middle_handler(app: tauri::AppHandle, content: String) -> Result<(), tauri::Error> {
    let payload = Parameters::new(content);
    app.emit_to("playground", "parameter-init", payload).unwrap();
    Ok(())
}