use crate::{
    GameConfig,
    Game,
    GameStatus,
    Input,
    win,
    lose,
};

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