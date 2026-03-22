use tauri::Emitter;
use tokio::sync::{RwLock, watch};

#[tauri::command(rename_all = "snake_case")]
pub async fn cancel_populate(
    state: tauri::State<'_, RwLock<raesan::AppState>>,
) -> Result<(), error::Error> {
    let state = state.read().await;
    if !state.db_state.is_populating() {
        return Err(error::Error::NotFoundError(
            "No current populate process running".to_string(),
        ));
    }
    state.db_state.cancel_populate()?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn populate_database(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, RwLock<raesan::AppState>>,
    input_data: String,
) -> Result<(), error::Error> {
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    {
        let mut state = state.write().await;
        if state.db_state.is_populating() {
            // NOTE: the reason the gaurd is not created before this line is because when the
            // program reaches this line, the "state.db_state.populating" is true, and we want it to stay
            // true even when we return Err() here
            return Err(error::Error::AlreadyRunningError(
                "You can only run 1 populate process at a time".to_string(),
            ));
        } else {
            state.db_state.begin_populate(shutdown_tx)?;
        }
    }

    // NOTE: this is called a "Guard Pattern", which basically is used to clean or mutate some
    // value whenever that value goes out of scope, so here when the RwLock<AppState> goes out of
    // scope the gaurd will automatically set the "state.db_state.populating = false", this will happen no
    // matter what is the reason behind a return i.e Err(), Ok(), whatever
    let _guard = raesan::AppStateGuard(state.inner());

    for i in 0..=10 {
        if *shutdown_rx.borrow() {
            app_handle.emit(
                "populate_event",
                serde_json::json!({
                    "name": "cancelled",
                    "data": "the process was cancelled",
                }),
            )?;
        }
        app_handle.emit(
            "populate_event",
            serde_json::json!({
                "name": format!("event_{}", i),
                "data": format!("The data that I got from js was: {}", input_data),
            }),
        )?;
        tokio::select! {
            _ = shutdown_rx.changed() => {
                app_handle.emit(
                    "populate_event",
                    serde_json::json!({
                        "name": "cancelled",
                        "data": "the process was cancelled",
                    }),
                )?;
                return Ok(())
            },
            _ = tokio::time::sleep(std::time::Duration::from_millis(1500)) => {}
        }
    }

    Ok(())
}
