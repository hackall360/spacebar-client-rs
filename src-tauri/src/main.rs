// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controllers;
mod ui;

fn main() {
    use dioxus::prelude::VirtualDom;
    use tauri::WebviewUrl;

    let mut dom = VirtualDom::new(ui::app::App);
    let html = dioxus_ssr::render(&dom);
    let data_url = format!("data:text/html,{}", urlencoding::encode(&html));

    tauri::Builder::default()
        .setup(move |app| {
            tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::External(data_url.parse().unwrap()))
                .build()
                .unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
