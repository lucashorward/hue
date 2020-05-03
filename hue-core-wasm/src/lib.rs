mod utils;
extern crate config;
extern crate serde_json;
extern crate reqwest;
extern crate hue_core;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn get_lights(base_uri: String) -> Result<JsValue, JsValue> {
    let api_string = hue_core::create_api_url(&base_uri, &"lights".to_owned());
    let res = reqwest::Client::new()
        .get(&api_string)
        .send()
        .await?;

    let text = res.text().await?;
    let branch_info: HashMap<i16, hue_core::Light> = serde_json::from_str(&text).unwrap();

    Ok(JsValue::from_serde(&branch_info).unwrap())
}

#[wasm_bindgen]
pub async fn set_light_state(base_uri: String, id: String, state: hue_core::LightState) {
    hue_core::set_light_status(&base_uri, id, &state).await;
}
