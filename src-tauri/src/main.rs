#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;

use std::{sync::Mutex, thread};

use audio::AudioEditor;
use tauri::{
    Menu,
    CustomMenuItem,
    Submenu,
    api::dialog::{
        blocking::{FileDialogBuilder, MessageDialogBuilder}, MessageDialogKind,
    },
};

fn main() {
    let open_file = CustomMenuItem::new("open_file".to_owned(), "Open File...");
    let submenu = Submenu::new("File", Menu::new().add_item(open_file));
    let menu = Menu::new()
        .add_submenu(submenu);

    let audio_editor = Mutex::new(AudioEditor::default());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .menu(menu)
        .on_menu_event(move |event| {
            match event.menu_item_id() {
                "open_file" => {
                    //ファイル選択ダイアログ表示
                    let Some(file_path) = FileDialogBuilder::new().pick_file() else { return; };

                    //ウィンドウにファイルパスを通知
                    let window = event.window();
                    window.emit("open_file", &file_path).unwrap();

                    //デコード
                    thread::scope(|s| {
                        s.spawn(|| {
                            if let Err(err) = audio_editor.lock().unwrap().decode(file_path) {
                                MessageDialogBuilder::new("デコーダーエラー", err).kind(MessageDialogKind::Error).show();
                                return;
                            };
                        });
                    });
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
