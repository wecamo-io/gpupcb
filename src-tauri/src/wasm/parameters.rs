// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct SParameters {
    pub s11: (f64, f64),
    pub s21: (f64, f64),
    pub s12: (f64, f64),
    pub s22: (f64, f64),
}

// TRANSMISSION LINE PARAMETERS

#[wasm_bindgen]
pub fn get_wave_impedance_with_wasm() -> Result<f64, JsValue> {
    Ok(1.1)
}

#[wasm_bindgen]
pub fn get_differential_impedance_with_wasm() -> Result<f64, JsValue> {
    Ok(1.2)
}

#[wasm_bindgen]
pub fn get_effective_dielectric_constant_with_wasm() -> Result<f64, JsValue> {
    Ok(1.3)
}

// ELECTROMAGNETIC COMPATIBILITY PARAMETERS

#[wasm_bindgen]
pub fn get_crosstalk_with_wasm() -> Result<f64, JsValue> {
    Ok(1.4)
}

// HIGH-SPEED SIGNAL INTEGRITY PARAMETERS

#[wasm_bindgen]
pub fn get_skew_with_wasm() -> Result<f64, JsValue> {
    Ok(1.5)
}

#[wasm_bindgen]
pub fn get_standing_wave_ratio_with_wasm() -> Result<f64, JsValue> {
    Ok(1.6)
}

#[wasm_bindgen]
pub fn get_s_parameters_with_wasm() -> Result<JsValue, JsValue> {
    let parameters = SParameters {
        s11: (1.71, -1.71),
        s21: (1.72, -1.72),
        s12: (1.73, -1.73),
        s22: (1.74, -1.74),
    };

    serde_wasm_bindgen::to_value(&parameters).map_err(|e| JsValue::from_str(&e.to_string()))
}
