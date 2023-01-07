use reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::ACCEPT;

use crate::HashMap;
use serde::{Deserialize, Serialize};

pub fn change_light_state_handler(s: &mut HashMap<String,String>) -> String {
    let res = change_light_state(s);
    "hello".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
struct Entity {
    entity_id: String,
}

#[tokio::main]
async fn change_light_state(s: &mut HashMap<String,String>) -> Result<(), Box<dyn std::error::Error>> {

    if s["state"] == "on" {
        println!("turning lights on");
    } else {
        println!("turning lights off");
    }

    let light_bedroom_all = Entity {
        entity_id: "light.bedroom_ceiling_lights".into(),
    };

    let url: String = "https://home.dominicbraam.com/api/services/light/turn_".to_owned();

    let url = url + &s["state"];

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header(AUTHORIZATION, "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIxYzZhNGY3NjRkZmQ0NjQyOGQ1NWRhM2E3ODAxOWUxZCIsImlhdCI6MTY2MzUzNTA1NywiZXhwIjoxOTc4ODk1MDU3fQ.XKGMb252-sZXEvC0K8gIM9gyKEsqluz1KZYAileLTV8")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&light_bedroom_all)
        .send()
        .await?;

    let res = response.text().await?;

    println!("{}",res);

    Ok(())
}


// the following code was mostly from ozone-api - will have to adapt to ozone-va
//
// pub async fn set_fan_state(s: &mut HashMap<String,String>) -> Result<(), Box<dyn std::error::Error>> {
// 
//     let fan_switch = Entity {
//         entity_id: "switch.lumi_lumi_plug_maus01_on_off".into(),
//     };
// 
//     let url: String = "https://home.dominicbraam.com/api/services/switch/turn_".to_owned();
// 
//     let state: &str = if state {
//         "on"
//     } else {
//         "off"
//     };
// 
//     let url = url + state;
// 
//     let client = reqwest::Client::new();
// 
//     let response = client
//         .post(url)
//         .header(AUTHORIZATION, "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIxYzZhNGY3NjRkZmQ0NjQyOGQ1NWRhM2E3ODAxOWUxZCIsImlhdCI6MTY2MzUzNTA1NywiZXhwIjoxOTc4ODk1MDU3fQ.XKGMb252-sZXEvC0K8gIM9gyKEsqluz1KZYAileLTV8")
//         .header(CONTENT_TYPE, "application/json")
//         .header(ACCEPT, "application/json")
//         .json(&fan_switch)
//         .send()
//         .await?;
// 
//     let res = response.text().await?;
// 
//     println!("{}",res);
//     
//     Ok(())
// }
// 
// pub async fn get_fan_state() -> Result<(), Box<dyn std::error::Error>> {
//    let fan_switch = Entity {
//         entity_id: "switch.lumi_lumi_plug_maus01_on_off".into(),
//     };
// 
//     let url: String = "https://home.dominicbraam.com/api/states".to_owned();
// 
//     let client = reqwest::Client::new();
// 
//     let response = client
//         .get(url)
//         .header(AUTHORIZATION, "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIxYzZhNGY3NjRkZmQ0NjQyOGQ1NWRhM2E3ODAxOWUxZCIsImlhdCI6MTY2MzUzNTA1NywiZXhwIjoxOTc4ODk1MDU3fQ.XKGMb252-sZXEvC0K8gIM9gyKEsqluz1KZYAileLTV8")
//         .header(CONTENT_TYPE, "application/json")
//         .header(ACCEPT, "application/json")
//         .json(&fan_switch)
//         .send()
//         .await?;
// 
//     let res = response.text().await?;
// 
//     println!("{}",res);
//     
//     Ok(())
// }
