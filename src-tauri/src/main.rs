use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, State,
};
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimerState {
    Work,
    ShortBreak,
    LongBreak,
    Idle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NextAction {
    Work,
    Break,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerStatus {
    pub state: TimerState,
    pub remaining_secs: u32,
    pub total_secs: u32,
    pub session: u32,
    pub sessions_until_long: u32,
    pub is_paused: bool,
    pub next_action: NextAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerSettings {
    pub work_duration: u32,
    pub short_break: u32,
    pub long_break: u32,
    pub sessions_before_long: u32,
    pub sound_enabled: bool,
}

impl Default for TimerSettings {
    fn default() -> Self {
        Self {
            work_duration: 25 * 60,
            short_break: 5 * 60,
            long_break: 15 * 60,
            sessions_before_long: 4,
            sound_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroRecord {
    pub start_time: String,
    pub duration_secs: u32,
    pub state: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub settings: TimerSettings,
    pub records: Vec<PomodoroRecord>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            settings: TimerSettings::default(),
            records: Vec::new(),
        }
    }
}

pub struct AppState {
    pub settings: Mutex<TimerSettings>,
    pub status: Mutex<TimerStatus>,
    pub cancel_tx: Mutex<Option<mpsc::Sender<()>>>,
    pub pause_tx: Mutex<Option<watch::Sender<bool>>>,
    pub records: Mutex<Vec<PomodoroRecord>>,
    pub data_path: Mutex<String>,
}

impl AppState {
    fn save_data(&self) {
        let data = AppData {
            settings: self.settings.lock().unwrap().clone(),
            records: self.records.lock().unwrap().clone(),
        };
        let path = self.data_path.lock().unwrap();
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = fs::write(&*path, json);
        }
    }

    fn load_data(path: &str) -> AppData {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
}

fn emit_status(app: &AppHandle, status: &TimerStatus) {
    app.emit("timer-status", status.clone()).ok();
}

#[tauri::command]
fn get_status(state: State<'_, AppState>) -> TimerStatus {
    state.status.lock().unwrap().clone()
}

#[tauri::command]
fn get_settings(state: State<'_, AppState>) -> TimerSettings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(state: State<'_, AppState>, settings: TimerSettings) {
    *state.settings.lock().unwrap() = settings;
    state.save_data();
}

#[tauri::command]
fn get_records(state: State<'_, AppState>) -> Vec<PomodoroRecord> {
    state.records.lock().unwrap().clone()
}

#[tauri::command]
fn get_today_stats(state: State<'_, AppState>) -> serde_json::Value {
    let records = state.records.lock().unwrap();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let today_records: Vec<&PomodoroRecord> = records
        .iter()
        .filter(|r| r.start_time.starts_with(&today) && r.state == "Work" && r.completed)
        .collect();
    serde_json::json!({
        "count": today_records.len(),
        "total_minutes": today_records.iter().map(|r| r.duration_secs / 60).sum::<u32>(),
    })
}

#[tauri::command]
async fn start_timer(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // Cancel any existing timer first
    {
        let tx = state.cancel_tx.lock().unwrap().take();
        if let Some(tx) = tx {
            let _ = tx.try_send(());
        }
        *state.pause_tx.lock().unwrap() = None;
    }

    let (cancel_tx, mut cancel_rx) = mpsc::channel::<()>(1);
    let (pause_tx, pause_rx) = watch::channel(false);
    {
        *state.cancel_tx.lock().unwrap() = Some(cancel_tx);
        *state.pause_tx.lock().unwrap() = Some(pause_tx);
    }

    let settings = state.settings.lock().unwrap().clone();

    let session;
    {
        let mut status = state.status.lock().unwrap();
        status.state = TimerState::Work;
        status.remaining_secs = settings.work_duration;
        status.total_secs = settings.work_duration;
        status.session += 1;
        status.is_paused = false;
        status.next_action = NextAction::None;
        session = status.session;
    }

    {
        let status = state.status.lock().unwrap();
        state.records.lock().unwrap().push(PomodoroRecord {
            start_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            duration_secs: status.total_secs,
            state: "Work".to_string(),
            completed: false,
        });
    }

    emit_status(&app, &state.status.lock().unwrap());

    let app2 = app.clone();

    tokio::spawn(async move {
        let mut remaining = settings.work_duration;
        let mut paused_rx = pause_rx;

        loop {
            let is_paused = *paused_rx.borrow();

            if is_paused {
                tokio::select! {
                    result = paused_rx.changed() => {
                        if result.is_err() { break; }
                    }
                    _ = cancel_rx.recv() => { break; }
                }
            } else {
                tokio::select! {
                    _ = sleep(Duration::from_secs(1)) => {
                        if remaining > 0 {
                            remaining -= 1;
                            let mins = remaining / 60;
                            let secs = remaining % 60;
                            if let Some(w) = app2.get_webview_window("main") {
                                let _ = w.set_title(&format!("专注中 {:02}:{:02}", mins, secs));
                            }
                            emit_status(&app2, &TimerStatus {
                                state: TimerState::Work,
                                remaining_secs: remaining,
                                total_secs: settings.work_duration,
                                session,
                                sessions_until_long: settings.sessions_before_long,
                                is_paused: false,
                                next_action: NextAction::None,
                            });
                        } else {
                            // Mark the work record as completed via AppHandle
                            {
                                let s = app2.state::<AppState>();
                                let mut records = s.records.lock().unwrap();
                                if let Some(last) = records.last_mut() {
                                    if last.state == "Work" && !last.completed {
                                        last.completed = true;
                                    }
                                }
                                drop(records);
                                s.save_data();
                            }
                            app2.emit("timer-alert", serde_json::json!({"label": "专注结束"})).ok();
                            if let Some(w) = app2.get_webview_window("main") {
                                let _ = w.set_title("番茄钟");
                            }
                            emit_status(&app2, &TimerStatus {
                                state: TimerState::Idle,
                                remaining_secs: 0,
                                total_secs: settings.work_duration,
                                session,
                                sessions_until_long: settings.sessions_before_long,
                                is_paused: false,
                                next_action: NextAction::Break,
                            });
                            break;
                        }
                    }
                    result = paused_rx.changed() => {
                        if result.is_err() { break; }
                        let new_paused = *paused_rx.borrow();
                        if new_paused {
                            emit_status(&app2, &TimerStatus {
                                state: TimerState::Work,
                                remaining_secs: remaining,
                                total_secs: settings.work_duration,
                                session,
                                sessions_until_long: settings.sessions_before_long,
                                is_paused: true,
                                next_action: NextAction::None,
                            });
                        }
                    }
                    _ = cancel_rx.recv() => { break; }
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn start_break(app: AppHandle, state: State<'_, AppState>, is_long: bool) -> Result<(), String> {
    // Cancel any existing timer first
    {
        let tx = state.cancel_tx.lock().unwrap().take();
        if let Some(tx) = tx {
            let _ = tx.try_send(());
        }
        *state.pause_tx.lock().unwrap() = None;
    }

    let (cancel_tx, mut cancel_rx) = mpsc::channel::<()>(1);
    let (pause_tx, pause_rx) = watch::channel(false);
    {
        *state.cancel_tx.lock().unwrap() = Some(cancel_tx);
        *state.pause_tx.lock().unwrap() = Some(pause_tx);
    }

    let settings = state.settings.lock().unwrap().clone();
    let break_duration = if is_long { settings.long_break } else { settings.short_break };
    let state_value = if is_long { TimerState::LongBreak } else { TimerState::ShortBreak };
    let break_label = if is_long { "长休息" } else { "短休息" };

    {
        let mut status = state.status.lock().unwrap();
        status.state = state_value.clone();
        status.remaining_secs = break_duration;
        status.total_secs = break_duration;
        status.is_paused = false;
        status.next_action = NextAction::None;
    }

    emit_status(&app, &state.status.lock().unwrap());

    let app2 = app.clone();

    tokio::spawn(async move {
        let mut remaining = break_duration;
        let mut paused_rx = pause_rx;

        loop {
            let is_paused = *paused_rx.borrow();

            if is_paused {
                tokio::select! {
                    result = paused_rx.changed() => {
                        if result.is_err() { break; }
                    }
                    _ = cancel_rx.recv() => { break; }
                }
            } else {
                tokio::select! {
                    _ = sleep(Duration::from_secs(1)) => {
                        if remaining > 0 {
                            remaining -= 1;
                            let mins = remaining / 60;
                            let secs = remaining % 60;
                            if let Some(w) = app2.get_webview_window("main") {
                                let _ = w.set_title(&format!("{} {:02}:{:02}", break_label, mins, secs));
                            }
                            emit_status(&app2, &TimerStatus {
                                state: state_value.clone(),
                                remaining_secs: remaining,
                                total_secs: break_duration,
                                session: 0,
                                sessions_until_long: settings.sessions_before_long,
                                is_paused: false,
                                next_action: NextAction::None,
                            });
                        } else {
                            app2.emit("timer-alert", serde_json::json!({"label": "休息结束"})).ok();
                            if let Some(w) = app2.get_webview_window("main") {
                                let _ = w.set_title("番茄钟");
                            }
                            emit_status(&app2, &TimerStatus {
                                state: TimerState::Idle,
                                remaining_secs: 0,
                                total_secs: break_duration,
                                session: 0,
                                sessions_until_long: settings.sessions_before_long,
                                is_paused: false,
                                next_action: NextAction::Work,
                            });
                            break;
                        }
                    }
                    result = paused_rx.changed() => {
                        if result.is_err() { break; }
                        let new_paused = *paused_rx.borrow();
                        if new_paused {
                            emit_status(&app2, &TimerStatus {
                                state: state_value.clone(),
                                remaining_secs: remaining,
                                total_secs: break_duration,
                                session: 0,
                                sessions_until_long: settings.sessions_before_long,
                                is_paused: true,
                                next_action: NextAction::None,
                            });
                        }
                    }
                    _ = cancel_rx.recv() => { break; }
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn pause_timer(state: State<'_, AppState>) -> Result<(), String> {
    let new_paused;
    {
        let mut status = state.status.lock().unwrap();
        status.is_paused = !status.is_paused;
        new_paused = status.is_paused;
    }

    let pause_tx = state.pause_tx.lock().unwrap().clone();
    if let Some(tx) = pause_tx {
        let _ = tx.send(new_paused);
    }

    Ok(())
}

#[tauri::command]
async fn skip_timer(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let tx = state.cancel_tx.lock().unwrap().take();
    if let Some(tx) = tx {
        let _ = tx.try_send(());
    }
    *state.pause_tx.lock().unwrap() = None;

    {
        let mut status = state.status.lock().unwrap();
        status.is_paused = false;
        status.remaining_secs = 0;
        match status.state {
            TimerState::Work => {
                status.state = TimerState::Idle;
                status.next_action = NextAction::Break;
            }
            TimerState::ShortBreak | TimerState::LongBreak => {
                status.state = TimerState::Idle;
                status.next_action = NextAction::Work;
            }
            TimerState::Idle => {}
        }
    }
    emit_status(&app, &state.status.lock().unwrap());

    if let Some(w) = app.get_webview_window("main") {
        let _ = w.set_title("番茄钟");
    }

    Ok(())
}

#[tauri::command]
async fn reset_timer(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let tx = state.cancel_tx.lock().unwrap().take();
    if let Some(tx) = tx {
        let _ = tx.try_send(());
    }
    *state.pause_tx.lock().unwrap() = None;

    let settings = state.settings.lock().unwrap().clone();
    {
        let mut status = state.status.lock().unwrap();
        status.state = TimerState::Idle;
        status.remaining_secs = settings.work_duration;
        status.total_secs = settings.work_duration;
        status.session = 0;
        status.is_paused = false;
        status.next_action = NextAction::None;
    }

    emit_status(&app, &state.status.lock().unwrap());
    state.save_data();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("pomodoro-timer")
        .join("pomodoro-data.json");

    fs::create_dir_all(data_path.parent().unwrap()).ok();

    let app_data = AppState::load_data(data_path.to_str().unwrap_or("pomodoro-data.json"));

    let state = AppState {
        settings: Mutex::new(app_data.settings),
        status: Mutex::new(TimerStatus {
            state: TimerState::Idle,
            remaining_secs: 25 * 60,
            total_secs: 25 * 60,
            session: 0,
            sessions_until_long: 4,
            is_paused: false,
            next_action: NextAction::None,
        }),
        cancel_tx: Mutex::new(None),
        pause_tx: Mutex::new(None),
        records: Mutex::new(app_data.records),
        data_path: Mutex::new(data_path.to_str().unwrap_or("pomodoro-data.json").to_string()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(|app| {
            let show = MenuItemBuilder::with_id("show", "显示窗口").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

            let icon = Image::from_bytes(include_bytes!("../icons/icon.png"))
                .unwrap_or_else(|_| app.default_window_icon().unwrap().clone());

            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .tooltip("番茄钟")
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            get_settings,
            update_settings,
            get_records,
            get_today_stats,
            start_timer,
            start_break,
            pause_timer,
            skip_timer,
            reset_timer,
        ])
        .run(tauri::generate_context!())
        .expect("启动番茄钟失败");
}

fn main() {
    run()
}
