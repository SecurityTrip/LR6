// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tauri::Manager;


#[tauri::command]
async fn initialize(app: tauri::AppHandle) {
    println!("Initializing app");
    start(app.clone()).await; // Вызываем функцию start и ожидаем ее завершения
    let _ = app.emit_all("initialize", ());
}


#[tauri::command]
async fn start(app: tauri::AppHandle){
    tokio::spawn(async move {
        // Создаем TCP слушателя для адреса 127.0.0.1:8080
        let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind address");

        println!("Server listening on 127.0.0.1:8080");

        // Принимаем входящие соединения в бесконечном цикле
        loop {
            // Ожидаем новое входящее соединение
            let (socket, _) = listener.accept().await.expect("Failed to accept connection");

            // Запускаем задачу для каждого входящего соединения
            let app_handle = app.clone(); // Создаем копию AppHandle для передачи внутрь closure
            tokio::spawn(async move {
                handle_client(socket, app_handle).await;
            });
        }
    });
}
async fn handle_client(mut socket: tokio::net::TcpStream, app: tauri::AppHandle) {
    let mut buf = [0; 1024];

    // Читаем данные из входящего соединения
    while let Ok(n) = socket.read(&mut buf).await {
        if n == 0 {
            // Соединение закрыто
            break;
        }

        // Обрабатываем полученные данные
        let received_data = String::from_utf8_lossy(&buf[..n]);
        let message = extract_message(&received_data);
        if !message.is_empty() {
            println!("Received message: {}", message);
            let message_copy = message.to_string(); // Создаем копию сообщения
            let _ = app.emit_all("msgs", message_copy);
        }
    }
}

fn extract_message(data: &str) -> &str {
    if let Some(index) = data.find("\r\n\r\n") {
        &data[index + 4..]
    } else {
        data
    }
}


fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![initialize])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        });
}