extern crate config;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use reqwest::Error;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct LightState {
    pub on: Option<bool>,
    pub bri: Option<i16>,
    pub hue: Option<i16>,
    pub sat: Option<i16>,
}

#[wasm_bindgen]
impl LightState {
    pub fn new(
        on: Option<bool>,
        bri: Option<i16>,
        hue: Option<i16>,
        sat: Option<i16>,
    ) -> LightState {
        LightState { on, bri, hue, sat }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub state: LightState,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    /// These are only the id's
    pub lights: Vec<String>,
    pub action: LightState,
}

pub fn generate_base_uri() -> String {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();
    let data = settings.try_into::<HashMap<String, String>>().unwrap();
    let ip = data.get("ip").unwrap();
    let key = data.get("apikey").unwrap();
    create_base_uri(&ip, &key)
}

pub async fn get_lights(base_uri: &String) -> Result<HashMap<i16, Light>, Error> {
    let api_string = create_api_url(&base_uri, &"lights".to_owned());
    let resp = reqwest::get(&api_string)
        .await?
        .json::<HashMap<i16, Light>>()
        .await?;
    Ok(resp)
}

pub async fn get_groups(base_uri: &String) -> Result<HashMap<i16, Group>, Error> {
    let api_string = create_api_url(&base_uri, &"groups".to_owned());
    let resp = reqwest::get(&api_string)
        .await?
        .json::<HashMap<i16, Group>>()
        .await?;
    Ok(resp)
}

/// Sets status of a light or group
/// Postfix is due to the API using the same object with different names (status for lights and action for groups) in a PUT call
pub async fn set_object_status(
    base_uri: &String,
    id: String,
    prefix: String,
    postfix: String,
    status: &LightState,
) -> Result<Response, Error> {
    let post_string = format!("{}/{}/{}", prefix, id, postfix);
    let api_string = create_api_url(&base_uri, &post_string);
    let client = reqwest::Client::new();
    client.put(&api_string).json(status).send().await
}

pub async fn set_light_status(
    base_uri: &String,
    id: String,
    status: &LightState,
) -> Result<Response, Error> {
    set_object_status(
        base_uri,
        id,
        "lights".to_owned(),
        "state".to_owned(),
        status,
    )
    .await
}

pub async fn set_group_action(
    base_uri: &String,
    id: String,
    status: &LightState,
) -> Result<Response, Error> {
    set_object_status(
        base_uri,
        id,
        "groups".to_owned(),
        "action".to_owned(),
        status,
    )
    .await
}

pub async fn turn_on_light(base_uri: &String, id: String) -> Result<Response, Error> {
    set_light_status(
        base_uri,
        id,
        &LightState {
            on: Some(true),
            bri: None,
            hue: None,
            sat: None,
        },
    )
    .await
}

pub async fn turn_off_light(base_uri: &String, id: String) -> Result<Response, Error> {
    set_light_status(
        base_uri,
        id,
        &LightState {
            on: Some(false),
            bri: None,
            hue: None,
            sat: None,
        },
    )
    .await
}

pub async fn turn_on_group(base_uri: &String, id: String) -> Result<Response, Error> {
    set_group_action(
        base_uri,
        id,
        &LightState {
            on: Some(true),
            bri: None,
            hue: None,
            sat: None,
        },
    )
    .await
}

pub async fn turn_off_group(base_uri: &String, id: String) -> Result<Response, Error> {
    set_group_action(
        base_uri,
        id,
        &LightState {
            on: Some(false),
            bri: None,
            hue: None,
            sat: None,
        },
    )
    .await
}

pub async fn set_light_brightness(
    base_uri: &String,
    id: String,
    brightness: i16,
) -> Result<Response, Error> {
    set_light_status(
        base_uri,
        id,
        &LightState {
            on: None,
            bri: Some(brightness),
            hue: None,
            sat: None,
        },
    )
    .await
}

fn create_base_uri(uri: &String, api_key: &String) -> String {
    format!("http://{}/api/{}", uri, api_key)
}

pub fn create_api_url(base_uri: &String, post_fix: &String) -> String {
    format!("{}/{}", base_uri, post_fix)
}
