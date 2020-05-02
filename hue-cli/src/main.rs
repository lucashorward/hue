extern crate hue_core;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let base_uri: String = hue_core::generate_base_uri();
    let command = std::env::args().nth(1).expect("no command given");
    if command == "lights" {
        show_light_names(&base_uri).await;
    }
    else if command == "light" {
        let subcommand = std::env::args().nth(2).expect("no sub command given");
        let id = std::env::args().nth(3).expect("no id given");
        if subcommand == "on" {
            turn_on_light(&base_uri, &id).await;
        }
        if subcommand == "off" {
            turn_off_light(&base_uri, &id).await;
        }
    } else if command == "help" {
        println!("The following commands are available:");
        println!("`lights` - prints all lights with id, name and status (on/off)");
        println!("`light on {{id}} - turns light {{id}} on");
        println!("`light off {{id}} - turns light {{id}} off");
    }
    else {
        println!("Unknown command: {}", command);
    }
}

async fn show_light_names(uri: &String) {
    let mut result: HashMap<i16, hue_core::Light> = HashMap::new();
    match hue_core::get_lights(uri).await {
        Err(why) => println!("{}", why),
        Ok(data) => result = data,
    };
    for (key, value) in &result {
        println!("id: {} - name: {} - on: {}", key, value.name, value.state.on);
    }
}

async fn turn_on_light(uri: &String, id: &String) {
    match hue_core::turn_on_light(uri, id.to_string()).await {
        Err(why) => println!("{}", why),
        Ok(data) => data
    };
}

async fn turn_off_light(uri: &String, id: &String) {
    match hue_core::turn_off_light(uri, id.to_string()).await {
        Err(why) => println!("{}", why),
        Ok(data) => data
    };
}