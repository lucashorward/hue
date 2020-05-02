extern crate config;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Serialize, Deserialize)]
pub struct LightState {
    pub on: bool,
    pub bri: i16,
    pub hue: i16,
    pub sat: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnOff {
    pub on: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub state: LightState,
    pub name: String
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

pub async fn set_light_status(base_uri: &String, id: String, light_on: bool) -> Result<(), Error> {
    let post_string = format!("lights/{}/state", id);
    let api_string = create_api_url(&base_uri, &post_string);
    let client = reqwest::Client::new();
    let status = OnOff { on: light_on};
    let result = client.put(&api_string)
        .json(&status)
        .send()
        .await;
    Ok(())
}

pub async fn turn_on_light(base_uri: &String, id: String) -> Result<(), Error> {
    set_light_status(base_uri, id, true).await
}

pub async fn turn_off_light(base_uri: &String, id: String) -> Result<(), Error> {
    set_light_status(base_uri, id, false).await
}

fn create_base_uri(uri: &String, api_key: &String) -> String {
    format!("http://{}/api/{}", uri, api_key)
}

fn create_api_url(base_uri: &String, post_fix: &String) -> String {
    format!("{}/{}", base_uri, post_fix)
}