#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Data types ──

#[derive(Serialize)]
struct SystemInfo {
    cpu_temp: u32,
    gpu_temp: u32,
    fan_rpm_cpu: u32,
    fan_rpm_gpu: u32,
    fan_rpm_mid: u32,
    fps: u32,
    battery_percent: u32,
    battery_health: u32,
    battery_wh: u32,
    battery_temp: u32,
    power_input: String,
    time_remaining: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FanPoint {
    temp: u32,
    percent: u32,
}

#[derive(Deserialize)]
struct OverlayConfig {
    enabled: bool,
    game_only: bool,
    position: String,
    font_size: u32,
    bg_opacity: u32,
    elements: HashMap<String, bool>,
    edge_bar: bool,
}

// ── ACPI/WMI constants from G-Helper analysis ──
// ASUS System Control Interface \\_SB.ATKD
// DSTS = Device Status (read), DEVS = Device Set (write)
//
// Performance modes:     DEVS 0x00120075 [0=balanced, 1=turbo, 2=silent]
// CPU fan curve:         DEVS 0x00110024 [8 temp/speed pairs]
// GPU fan curve:         DEVS 0x00110025
// Mid fan curve:         DEVS 0x00110032
// PPT STAPM (APU slow):  Ryzen SMU mailbox PPT_APUA0
// PPT Fast:              Ryzen SMU mailbox PPT_CPUB0
// PPT GPU:               Ryzen SMU mailbox PPT_GPUC0
// Temp limit:            Ryzen SMU mailbox PPT_TEMP9E
// GPU mode (Eco/Std/Opt/Ult): DEVS 0x00090020
// Refresh rate:          DEVS 0x00050025 [60/120]
// Charge limit:          DEVS 0x00120057 [60/80/100]
// Keyboard backlight:    DEVS 0x00050021

// ── Commands ──

#[tauri::command]
fn get_system_info() -> SystemInfo {
    // ponytail: mock data; real impl reads ACPI DSTS + SMU + battery WMI
    let mut rng = rand::thread_rng();
    SystemInfo {
        cpu_temp: 65 + rng.gen_range(0..15),
        gpu_temp: 58 + rng.gen_range(0..12),
        fan_rpm_cpu: 2800 + rng.gen_range(0..800),
        fan_rpm_gpu: 2400 + rng.gen_range(0..600),
        fan_rpm_mid: 2000 + rng.gen_range(0..400),
        fps: 55 + rng.gen_range(0..10),
        battery_percent: 72,
        battery_health: 96,
        battery_wh: 80,
        battery_temp: 38,
        power_input: "65W USB-C".into(),
        time_remaining: "2j 15m".into(),
    }
}

#[tauri::command]
fn set_performance_mode(mode: String) -> Result<String, String> {
    // DEVS 0x00120075: 0=balanced, 1=turbo, 2=silent
    // Manual mode uses per-PPT values instead
    println!("[ACPI stub] set_performance_mode: {}", mode);
    Ok(format!("Mode set to {}", mode))
}

#[tauri::command]
fn set_tdp(stapm: u32, fast_ppt: u32, gpu_ppt: u32, temp_limit: u32) -> Result<String, String> {
    // Ryzen SMU mailbox writes:
    // PPT_APUA0 = stapm, PPT_CPUB0 = fast_ppt, PPT_GPUC0 = gpu_ppt
    // PPT_TEMP9E = temp_limit
    println!(
        "[SMU stub] set_tdp: STAPM={}W Fast={}W GPU={}W TempLimit={}C",
        stapm, fast_ppt, gpu_ppt, temp_limit
    );
    Ok("TDP updated".into())
}

#[tauri::command]
fn set_auto_tdp(enabled: bool, target_fps: u32) -> Result<String, String> {
    println!("[stub] auto_tdp: enabled={} target_fps={}", enabled, target_fps);
    Ok("Auto TDP updated".into())
}

#[tauri::command]
fn set_gpu_mode(mode: String) -> Result<String, String> {
    // DEVS 0x00090020: eco=0, standard=1, optimized=2, ultimate=3
    println!("[ACPI stub] set_gpu_mode: {}", mode);
    Ok(format!("GPU mode set to {}", mode))
}

#[tauri::command]
fn get_fan_curve(fan: String) -> Vec<FanPoint> {
    // DSTS 0x00110024 (cpu), 0x00110025 (gpu), 0x00110032 (mid)
    let base: Vec<(u32, u32)> = match fan.as_str() {
        "gpu" => vec![(30,0),(40,5),(50,20),(60,35),(70,55),(80,75),(90,90),(100,100)],
        "mid" => vec![(30,0),(40,5),(50,15),(60,30),(70,50),(80,70),(90,85),(100,100)],
        _ =>     vec![(30,0),(40,10),(50,25),(60,40),(70,60),(80,80),(90,95),(100,100)],
    };
    base.into_iter().map(|(t, p)| FanPoint { temp: t, percent: p }).collect()
}

#[tauri::command]
fn set_fan_curve(fan: String, points: Vec<FanPoint>) -> Result<String, String> {
    // DEVS 0x00110024/25/32: 8 pairs of (temp, fan%) packed into bytes
    println!("[ACPI stub] set_fan_curve {}: {} points", fan, points.len());
    Ok("Fan curve updated".into())
}

#[tauri::command]
fn set_custom_fan_enabled(enabled: bool) -> Result<String, String> {
    println!("[stub] custom_fan: {}", enabled);
    Ok("Custom fan toggled".into())
}

#[tauri::command]
fn set_refresh_rate(hz: u32) -> Result<String, String> {
    // DEVS 0x00050025: 60 or 120
    println!("[ACPI stub] refresh_rate: {}Hz", hz);
    Ok(format!("Refresh rate set to {}Hz", hz))
}

#[tauri::command]
fn set_hdr(enabled: bool) -> Result<String, String> {
    println!("[stub] HDR: {}", enabled);
    Ok("HDR toggled".into())
}

#[tauri::command]
fn set_overdrive(enabled: bool) -> Result<String, String> {
    println!("[stub] overdrive: {}", enabled);
    Ok("Overdrive toggled".into())
}

#[tauri::command]
fn set_brightness(percent: u32) -> Result<String, String> {
    println!("[stub] brightness: {}%", percent);
    Ok("Brightness updated".into())
}

#[tauri::command]
fn set_color_profile(profile: String) -> Result<String, String> {
    println!("[stub] color_profile: {}", profile);
    Ok(format!("Color profile set to {}", profile))
}

#[tauri::command]
fn set_charge_limit(percent: u32) -> Result<String, String> {
    // DEVS 0x00120057: 60, 80, or 100
    println!("[ACPI stub] charge_limit: {}%", percent);
    Ok(format!("Charge limit set to {}%", percent))
}

#[tauri::command]
fn set_power_plan(plan: String) -> Result<String, String> {
    // Windows powercfg -setactive <GUID>
    println!("[stub] power_plan: {}", plan);
    Ok(format!("Power plan set to {}", plan))
}

#[tauri::command]
fn set_usbc_charging(enabled: bool) -> Result<String, String> {
    println!("[stub] usbc_charging: {}", enabled);
    Ok("USB-C charging toggled".into())
}

#[tauri::command]
fn set_button_mapping(button: String, action: String) -> Result<String, String> {
    println!("[stub] map {} -> {}", button, action);
    Ok(format!("{} mapped to {}", button, action))
}

#[tauri::command]
fn get_button_mappings(profile: u32) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("A".into(), "A (Enter)".into());
    map.insert("B".into(), "B (Escape)".into());
    map.insert("X".into(), "X".into());
    map.insert("Y".into(), "Y".into());
    map.insert("LB".into(), "LB".into());
    map.insert("RB".into(), "RB".into());
    map.insert("LT".into(), "LT".into());
    map.insert("RT".into(), "RT".into());
    map.insert("M1".into(), "Screenshot".into());
    map.insert("M2".into(), "Overlay Toggle".into());
    let _ = profile; // ponytail: per-profile storage when needed
    map
}

#[tauri::command]
fn set_controller_profile(profile: u32) -> Result<String, String> {
    println!("[stub] controller_profile: {}", profile);
    Ok(format!("Profile {} activated", profile))
}

#[tauri::command]
fn set_overlay_config(config: OverlayConfig) -> Result<String, String> {
    println!(
        "[stub] overlay: enabled={} pos={} size={}px opacity={}%",
        config.enabled, config.position, config.font_size, config.bg_opacity
    );
    let _ = (config.game_only, config.elements, config.edge_bar);
    Ok("Overlay config updated".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            set_performance_mode,
            set_tdp,
            set_auto_tdp,
            set_gpu_mode,
            get_fan_curve,
            set_fan_curve,
            set_custom_fan_enabled,
            set_refresh_rate,
            set_hdr,
            set_overdrive,
            set_brightness,
            set_color_profile,
            set_charge_limit,
            set_power_plan,
            set_usbc_charging,
            set_button_mapping,
            get_button_mappings,
            set_controller_profile,
            set_overlay_config,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
