// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

// TRANSMISSION LINES PARAMS

use serde::Serialize;

#[derive(Serialize)]
pub struct SParameters {
    pub s11: (f64, f64), // (f64, f64) represents complex number
    pub s21: (f64, f64),
    pub s12: (f64, f64),
    pub s22: (f64, f64),
}

#[tauri::command]
pub fn get_wave_impedance() -> Result<f64, String> {
    Ok(1.1)
}

#[tauri::command]
pub fn get_differential_impedance() -> Result<f64, String> {
    Ok(1.2)
}

#[tauri::command]
pub fn get_effective_dielectric_constant() -> Result<f64, String> {
    Ok(1.3)
}

// ELECTROMAGNETIC COMPATIBILITY PARAMS

#[tauri::command]
pub fn get_crosstalk() -> Result<f64, String> {
    Ok(1.4)
}

// HIGH SPEED SIGNAL INTEGRITY PARAMS

#[tauri::command]
pub fn get_skew() -> Result<f64, String> {
    Ok(1.5)
}

#[tauri::command]
pub fn get_standing_wave_ratio() -> Result<f64, String> {
    Ok(1.6)
}

#[tauri::command]
pub fn get_s_parameters() -> Result<SParameters, String> {
    Ok(SParameters {
        s11: (1.71, -1.71),
        s21: (1.72, -1.72),
        s12: (1.73, -1.73),
        s22: (1.74, -1.74),
    })
}
