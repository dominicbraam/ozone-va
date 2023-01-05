use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

mod interpret;
mod capabilities;

fn main(){

    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut command_list = capabilities::create_commands();

    interpret::start_picovoice(
        2,
        &env::var("ACCESS_KEY").expect("hmm"),
        &env::var("KEYWORD_FILE_PATH").expect("hmm"),
        &env::var("CONTEXT_FILE_PATH").expect("hmm"),
        &mut command_list
        );
}
