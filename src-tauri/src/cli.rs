use std::f32::consts::E;

use crate::{
    GameConfig,
    Game,
    GameStatus,
    Input,
    win,
    lose,
};
use tokio::sync::{Mutex, mpsc};

pub struct AsyncProcInputTx {
    pub tx: Mutex<mpsc::Sender<AsyncInput>>,
}
pub enum AsyncInput {
    StartGame(GameConfig),
    LeftClick(usize, usize),
    RightClick(usize, usize),
    EndGame,
    EndSession,
    EndProgram,
}
pub enum AsyncOutput {
    GameStatus(GameStatus),
    Display(Vec<String>),
    Error(String),
    Window(usize, usize),
}
pub async fn main_thread(
    mut input: mpsc::Receiver<AsyncInput>,
    output: mpsc::Sender<AsyncOutput>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut game = Game::placehold();
    while let Some(instruction) = input.recv().await {
        match instruction {
            AsyncInput::StartGame(game_config) => {
                // this arem is for the game to start run
                let start_receit = Game::init_ref(game_config, &mut game);
                if start_receit.is_err() {
                    output
                        .send(AsyncOutput::Error(start_receit.unwrap_err().to_string()))
                        .await?;
                }
                else {
                    // Send a game status update to the frontend
                    output
                        .send(AsyncOutput::GameStatus(GameStatus::InProgress))
                        .await?;
                    // Send a display update to the frontend
                    // output.send(AsyncOutput::Display(game.display())).await;
                    // Send a Window to initialize the window and display the game
                    output.send(AsyncOutput::Window(game_config.width, game_config.height)).await?;
                }

            },
            AsyncInput::LeftClick(x, y) => {

            },
            AsyncInput::RightClick(x, y) => {
            },
            _ => (),
        }
    }
    Ok(())
}



/// Start a new game
/// Write into the api_tx channel
#[tauri::command]
pub async fn start_game(
    width: String,
    height: String,
    mines: String,
    input: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
    let input_tx = input.tx.lock().await;
    let args = vec![width, height, mines];
    let game_config: GameConfig = GameConfig::init(&args)?;
    let start_game_config = AsyncInput::StartGame(game_config);
    input_tx
        .send(start_game_config)
        .await
        .map_err(|e|e.to_string())
}

pub async fn output_handler<R: tauri::Runtime>(
    output: AsyncOutput,
    manager: &impl tauri::Manager<R>,
) {
    match output {
        AsyncOutput::Error(e) => {
            manager.emit_all("error", e).unwrap();
        }
        AsyncOutput::Window(w, h) => {
            create_window(w, h, manager).await;
        }
        _ => (),
    };
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn display(n: usize) -> Vec<String> {
    vec![String::from("&#128681;"); n]
}

async fn create_window<R: tauri::Runtime>(
    w: usize, h: usize,
    manager: &impl tauri::Manager<R>
) {
    let playboard = tauri::WindowBuilder::new(
        manager,
        "playboard",
        tauri::WindowUrl::App("../../dist/playboard.html".into()),
    );
    manager
        .emit_to(
            "playboard",
            "parameter-init",
            format!("{{width: {}, height: {}}}", w, h)
        ).unwrap();
}