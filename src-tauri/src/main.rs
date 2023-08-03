use MS::*;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    // 'api' is short for 'async proc input'
    // 'apo' is short for 'async proc output'
    let (api_tx, api_rx)
        : (mpsc::Sender<cli::AsyncInput>, mpsc::Receiver<cli::AsyncInput>)
        = mpsc::channel(1);
    let (apo_tx, mut apo_rx)
        : (mpsc::Sender<cli::AsyncOutput>, mpsc::Receiver<cli::AsyncOutput>)
        = mpsc::channel(1);

    tauri::Builder::default()
    .manage(cli::AsyncProcInputTx{
        tx: Mutex::new(api_tx)
    })
    .setup(|app| {
        tauri::async_runtime::spawn( async move {
            cli::main_thread(api_rx, apo_tx).await;
        });
        let app_handle = app.handle();
        tauri::async_runtime::spawn(async move {
            loop {
                if let Some(output) = apo_rx.recv().await {
                    cli::output_handler(output, &app_handle).await;
                }
            }
        });
        return Ok(());
    })
    .invoke_handler(tauri::generate_handler![
        cli::start_game,
        cli::display
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

