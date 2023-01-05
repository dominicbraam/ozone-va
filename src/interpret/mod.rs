use picovoice::{rhino::RhinoInference, PicovoiceBuilder};
use ctrlc;
use pv_recorder::RecorderBuilder;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::HashMap;
use crate::capabilities::Response;

static LISTENING: AtomicBool = AtomicBool::new(false);

pub fn start_picovoice(
        audio_device_index: i32,
        access_key: &str,
        keyword_path: &str,
        context_path: &str,
        commands: &mut HashMap<&str, Box<Response>>
    ){

    let wake_word_callback = || {
        // wake word detected
        println!("Heard you bro");
    };

    let inference_callback = |inference: RhinoInference| {
        if inference.is_understood {
            let intent = inference.intent.unwrap();
            let mut slots = inference.slots;

            // take action based on inferred intent and slot values
            println!("{}",commands.get(&intent as &str).unwrap()(&mut slots));
        } else {
            // handle unsupported commands
            println!("Did not understand");
        }
    };

    let mut picovoice = PicovoiceBuilder::new(
        access_key,
        keyword_path,
        wake_word_callback,
        context_path,
        inference_callback,
    )
    .init()
    .expect("Failed to create picovoice");


    // live mic

    let recorder = RecorderBuilder::new()
        .device_index(audio_device_index)
        .frame_length(picovoice.frame_length() as i32)
        .init()
        .expect("Failed to initialize pvrecorder");
    recorder.start().expect("Failed to start audio recording");

    LISTENING.store(true, Ordering::SeqCst);
    ctrlc::set_handler(|| {
        LISTENING.store(false, Ordering::SeqCst);
    })
    .expect("Unable to setup signal handler");

    println!("Listening for commands...");

    while LISTENING.load(Ordering::SeqCst) {
        let mut pcm = vec![0; recorder.frame_length()];
        recorder.read(&mut pcm).expect("Failed to read audio frame");

        picovoice.process(&pcm).unwrap();
    }

    println!("\nStopping...");
    recorder.stop().expect("Failed to stop audio recording");
}
