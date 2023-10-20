// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::{str::FromStr, default};
mod httpclient;
use httpclient::{ 
    client,
    err,
    method 
};
use client::{ ResponseData, HttpRequestBuilder, ClientBuilder };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(mut name: &str) -> String {
    match name {
        "" => name = "NULL",
        "NULL" => name = "NaN",
        "NaN" => name = "NULL",
        _ => (),
    }
    format!("Hello, {}! You've been greeted from Rust!", name)
}

type ClientId = u32;

#[tauri::command]
async fn create_client(options: Option<ClientBuilder>)->err::Result<ClientId>{
    println!("create_client");
    return method::Cmd::create_client(options).await;
}

#[tauri::command]
async fn drop_client(client_id: ClientId)->err::Result<()>{
    println!("drop_client:{:?}",client_id);
    return method::Cmd::drop_client(client_id).await;
}

#[tauri::command]
async fn http_request(client_id: ClientId,options: Box<HttpRequestBuilder>,)
    -> err::Result<ResponseData>
{
    println!("http_request:{:?}",client_id);
    println!("options:{:?}",options);
    return method::Cmd::http_request(client_id,options).await;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_client,
            drop_client,
            http_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
