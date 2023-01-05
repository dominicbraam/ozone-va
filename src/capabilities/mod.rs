use crate::HashMap;

pub mod clock;
pub mod homeassistant;

pub type Response = dyn Fn(&mut HashMap<String,String>) -> String;

pub fn create_commands() -> HashMap<&'static str, Box<Response>> {

    let mut command_map: HashMap<&str, Box<Response>> = HashMap::new();
    
    // context: CLOCK
    command_map.insert("timer", Box::new(clock::timer));
    command_map.insert("setTimer", Box::new(clock::set_timer));
    command_map.insert("stopwatch", Box::new(clock::start_stopwatch));

    // context: SMART LIGHTING
    command_map.insert("changeLightState", Box::new(homeassistant::change_light_state_handler));

    command_map
}



