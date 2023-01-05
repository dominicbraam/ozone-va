use crate::HashMap;

pub fn timer(s: &mut HashMap<String,String>) -> String {
    println!("\tslots : {{");
            for (slot, value) in s.iter() {
                println!("\t\t{} : {}", slot, value);
            }
            println!("\t}}");
            println!("}}\n");
    "hello".to_string()
}

pub fn set_timer(s: &mut HashMap<String,String>) -> String {
    println!("\tslots : {{");
            for (slot, value) in s.iter() {
                println!("\t\t{} : {}", slot, value);
            }
            println!("\t}}");
            println!("}}\n");
    "hello".to_string()
}

pub fn start_stopwatch(s: &mut HashMap<String,String>) -> String {
    println!("\tslots : {{");
            for (slot, value) in s.iter() {
                println!("\t\t{} : {}", slot, value);
            }
            println!("\t}}");
            println!("}}\n");
    "hello".to_string()
}
