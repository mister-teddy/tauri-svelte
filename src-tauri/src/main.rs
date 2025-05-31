// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
struct NetworkHealth {
    status: String,
    latency: i32,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeoLocation {
    ip: String,
    city: String,
    region: String,
    country: String,
    org: String,
    timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemStatus {
    processes: i32,
    uptime: String,
    load: String,
    timestamp: String,
}

#[tauri::command]
async fn check_network_health() -> Result<NetworkHealth, String> {
    let output = Command::new("ping")
        .args(["-c", "1", "8.8.8.8"])
        .output()
        .map_err(|e| e.to_string())?;

    let status = if output.status.success() {
        "connected".to_string()
    } else {
        "disconnected".to_string()
    };

    // Extract latency from ping output
    let latency = if status == "connected" {
        let output_str = String::from_utf8_lossy(&output.stdout);
        output_str
            .lines()
            .find(|line| line.contains("time="))
            .and_then(|line| {
                line.split("time=")
                    .nth(1)?
                    .split_whitespace()
                    .next()?
                    .parse::<f32>()
                    .ok()
            })
            .map(|time| time as i32)
            .unwrap_or(0)
    } else {
        0
    };

    Ok(NetworkHealth {
        status,
        latency,
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
async fn get_geo_location() -> Result<GeoLocation, String> {
    let output = Command::new("curl")
        .args(["-s", "https://ipinfo.io/json"])
        .output()
        .map_err(|e| e.to_string())?;

    let geo_data: GeoLocation =
        serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    Ok(geo_data)
}

#[tauri::command]
async fn get_system_status() -> Result<SystemStatus, String> {
    // Get process count
    let ps_output = Command::new("ps")
        .args(["aux"])
        .output()
        .map_err(|e| e.to_string())?;

    let process_count = String::from_utf8_lossy(&ps_output.stdout).lines().count() as i32;

    // Get uptime
    let uptime_output = Command::new("uptime")
        .arg("-p")
        .output()
        .map_err(|e| e.to_string())?;

    let uptime = String::from_utf8_lossy(&uptime_output.stdout)
        .trim()
        .to_string();

    // Get load average
    let load_output = Command::new("uptime").output().map_err(|e| e.to_string())?;

    let load = String::from_utf8_lossy(&load_output.stdout)
        .split("load average:")
        .nth(1)
        .unwrap_or("")
        .trim()
        .to_string();

    Ok(SystemStatus {
        processes: process_count,
        uptime,
        load,
        timestamp: Utc::now().to_rfc3339(),
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_network_health,
            get_geo_location,
            get_system_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
