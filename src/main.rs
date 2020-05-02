extern crate config;
use std::collections::HashMap;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap();
    let data = settings.try_into::<HashMap<String, String>>().unwrap();
    let ip = data.get("ip").unwrap();
    let key = data.get("apikey").unwrap();
    let base_uri = create_base_uri(&ip, &key);
    let return_data = get_lights(&base_uri);
    println!("{}", return_data);
}

fn get_lights(base_uri: &String) -> String {
    let api_string = create_api_url(&base_uri, &"lights".to_owned());
    println!("{}", api_string);
    let response = ureq::get(&api_string).call();
    println!("{}", &response.status_line());
    response.into_string().unwrap()
}

fn create_base_uri(uri: &String, api_key: &String) -> String {
   format!("http://{}/api/{}", uri, api_key)
}

fn create_api_url(base_uri: &String, post_fix: &String) -> String {
    format!("{}/{}", base_uri, post_fix)
}
