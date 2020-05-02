extern crate hue_core;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let base_uri: String = hue_core::generate_base_uri();
    let command = std::env::args().nth(1).expect("no command given");
    if command == "lights" {
        show_light_names(&base_uri).await;
    } else {
        println!("Unknown command");
    }
}

async fn show_light_names(uri: &String) {
    let mut result: HashMap<i16, hue_core::Light> = HashMap::new();
    match hue_core::get_lights(uri).await {
        Err(why) => println!("{}", why),
        Ok(data) => result = data,
    };
    for (_, value) in &result {
        println!("Name: {}", value.name);
    }
}
