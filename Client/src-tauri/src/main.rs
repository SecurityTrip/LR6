// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::time::{sleep};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::time::{self, interval};
use tauri::Manager;
use reqwest;




#[tauri::command]
fn start_connection_timer1(app: tauri::AppHandle) {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(0)).await;
        let start_time = Instant::now();
        let mut elapsed_time = Duration::from_secs(0);

        let mut timer = interval(Duration::from_secs(1));

        loop {
            // Отправка времени ожидания во фронтенд
            let _ = app.emit_all("updateConnectionTime1", elapsed_time.as_secs());
            //println!("Clinet 1 time: {}", elapsed_time.as_secs());

            match TcpStream::connect("127.0.0.1:8080").await {
                Ok(_) => {
                    let total_elapsed_time = start_time.elapsed();
                    let _ = app.emit_all("connectionSuccess1", total_elapsed_time.as_secs());

                    // Отправка данных на сервер
                    let client = reqwest::Client::new();
                    let url = "http://127.0.0.1:8080"; // Замените на адрес сервера
                    let body = format!("клиент 1 ожидал {} секунд", total_elapsed_time.as_secs());
                    let _ = client.post(url).body(body).send().await;

                    break;
                }
                Err(_) => {
                    elapsed_time += Duration::from_secs(1);
                }
            }

            timer.tick().await;
        }
    });
}

#[tauri::command]
fn start_connection_timer2(app: tauri::AppHandle) {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;
        let start_time = Instant::now();
        let mut elapsed_time = Duration::from_secs(0);

        let mut timer = interval(Duration::from_secs(1));

        loop {
            // Отправка времени ожидания во фронтенд
            let _ = app.emit_all("updateConnectionTime2", elapsed_time.as_secs());
            //println!("Clinet 2 time: {}", elapsed_time.as_secs());

            match TcpStream::connect("127.0.0.1:8080").await {
                Ok(_) => {
                    let total_elapsed_time = start_time.elapsed();
                    let _ = app.emit_all("connectionSuccess2", total_elapsed_time.as_secs());

                    
                    // Отправка данных на сервер
                    let client = reqwest::Client::new();
                    let url = "http://127.0.0.1:8080"; // Замените на адрес сервера
                    let body = format!("клиент 2 ожидал {} секунд", total_elapsed_time.as_secs());
                    let _ = client.post(url).body(body).send().await;


                    break;
                }
                Err(_) => {
                    elapsed_time += Duration::from_secs(1);
                }
            }

            timer.tick().await;
        }
    });
}

#[tauri::command]
fn start_connection_timer3(app: tauri::AppHandle) {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let start_time = Instant::now();
        let mut elapsed_time = Duration::from_secs(0);

        let mut timer = interval(Duration::from_secs(1));

        loop {
            // Отправка времени ожидания во фронтенд
            let _ = app.emit_all("updateConnectionTime3", elapsed_time.as_secs());
            //println!("Clinet 3 time: {}", elapsed_time.as_secs());

            match TcpStream::connect("127.0.0.1:8080").await {
                Ok(_) => {
                    let total_elapsed_time = start_time.elapsed();
                    let _ = app.emit_all("connectionSuccess3", total_elapsed_time.as_secs());

                    
                    // Отправка данных на сервер
                    let client = reqwest::Client::new();
                    let url = "http://127.0.0.1:8080"; // Замените на адрес сервера
                    let body = format!("клиент 3 ожидал {} секунд", total_elapsed_time.as_secs());
                    let _ = client.post(url).body(body).send().await;


                    break;
                }
                Err(_) => {
                    elapsed_time += Duration::from_secs(1);
                }
            }

            timer.tick().await;
        }
    });
}

#[tauri::command]
fn start_connection_timer4(app: tauri::AppHandle) {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let start_time = Instant::now();
        let mut elapsed_time = Duration::from_secs(0);

        let mut timer = interval(Duration::from_secs(1));

        loop {
            // Отправка времени ожидания во фронтенд
            let _ = app.emit_all("updateConnectionTime4", elapsed_time.as_secs());
            //println!("Clinet 4 time: {}", elapsed_time.as_secs());

            match TcpStream::connect("127.0.0.1:8080").await {
                Ok(_) => {
                    let total_elapsed_time = start_time.elapsed();
                    let _ = app.emit_all("connectionSuccess4", total_elapsed_time.as_secs());

                    
                    // Отправка данных на сервер
                    let client = reqwest::Client::new();
                    let url = "http://127.0.0.1:8080"; // Замените на адрес сервера
                    let body = format!("клиент 4 ожидал {} секунд", total_elapsed_time.as_secs());
                    let _ = client.post(url).body(body).send().await;

                    
                    break;
                }
                Err(_) => {
                    elapsed_time += Duration::from_secs(1);
                }
            }

            timer.tick().await;
        }
    });
}

#[tauri::command]
fn initialize(app: tauri::AppHandle) {
    start_connection_timer1(app.clone()); // Явный вызов функции start_connection_timer1
    start_connection_timer2(app.clone()); // Явный вызов функции start_connection_timer2
    start_connection_timer3(app.clone()); // Явный вызов функции start_connection_timer3
    start_connection_timer4(app.clone()); // Явный вызов функции start_connection_timer4
    let _ = app.emit_all("initialize", ());
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
