#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use tauri::ipc::{InvokeBody, Request, Response};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::Emitter;

// File paths travel in a percent-encoded header so file bytes can go as a raw binary body.
fn percent_decode(s: &str) -> Result<String, String> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            let hex = std::str::from_utf8(&b[i + 1..i + 3]).map_err(|e| e.to_string())?;
            out.push(u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8(out).map_err(|e| e.to_string())
}

fn header_path(request: &Request<'_>) -> Result<PathBuf, String> {
    let raw = request
        .headers()
        .get("x-path")
        .ok_or("missing file path")?
        .to_str()
        .map_err(|e| e.to_string())?;
    Ok(PathBuf::from(percent_decode(raw)?))
}

#[tauri::command]
fn save_file(request: Request<'_>) -> Result<(), String> {
    let path = header_path(&request)?;
    match request.body() {
        InvokeBody::Raw(data) => std::fs::write(&path, data).map_err(|e| e.to_string()),
        _ => Err("expected binary data".into()),
    }
}

#[tauri::command]
fn read_file(request: Request<'_>) -> Result<Response, String> {
    let path = header_path(&request)?;
    let data = std::fs::read(&path).map_err(|e| e.to_string())?;
    Ok(Response::new(data))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_file, read_file])
        .setup(|app| {
            let handle = app.handle();
            let open = MenuItem::with_id(handle, "open", "Open Image…", true, Some("CmdOrCtrl+O"))?;
            let export = MenuItem::with_id(handle, "export", "Export", true, Some("CmdOrCtrl+E"))?;
            let updates = MenuItem::with_id(handle, "updates", "Check for Updates…", true, None::<&str>)?;
            let app_menu = Submenu::with_items(
                handle,
                "Any Angle Mockup",
                true,
                &[
                    &PredefinedMenuItem::about(handle, None, None)?,
                    &updates,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::hide(handle, None)?,
                    &PredefinedMenuItem::hide_others(handle, None)?,
                    &PredefinedMenuItem::show_all(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::quit(handle, None)?,
                ],
            )?;
            let file = Submenu::with_items(
                handle,
                "File",
                true,
                &[
                    &open,
                    &export,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::close_window(handle, None)?,
                ],
            )?;
            let edit = Submenu::with_items(
                handle,
                "Edit",
                true,
                &[
                    &PredefinedMenuItem::undo(handle, None)?,
                    &PredefinedMenuItem::redo(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::cut(handle, None)?,
                    &PredefinedMenuItem::copy(handle, None)?,
                    &PredefinedMenuItem::paste(handle, None)?,
                    &PredefinedMenuItem::select_all(handle, None)?,
                ],
            )?;
            let view = Submenu::with_items(handle, "View", true, &[&PredefinedMenuItem::fullscreen(handle, None)?])?;
            let window = Submenu::with_items(
                handle,
                "Window",
                true,
                &[
                    &PredefinedMenuItem::minimize(handle, None)?,
                    &PredefinedMenuItem::maximize(handle, None)?,
                ],
            )?;
            let menu = Menu::with_items(handle, &[&app_menu, &file, &edit, &view, &window])?;
            app.set_menu(menu)?;
            app.on_menu_event(|app, event| {
                let _ = app.emit("menu", event.id().as_ref().to_string());
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Any Angle Mockup");
}
