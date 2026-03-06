use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Emotion {
    Idle,
    Angry,
    Laugh,
    Embarrassed,
    Scared,
    Oops,
}

pub fn start_monitor(emotion_state: Arc<Mutex<Emotion>>) {
    thread::spawn(move || {
        let mut last_random = Instant::now();

        loop {
            let now = Instant::now();

            if now.duration_since(last_random).as_secs() > 10 {
                last_random = now;

                let mut rng = rand::thread_rng();
                let roll: u8 = rng.gen_range(0..100);

                let new_emotion = if roll < 50 {
                    Emotion::Idle
                } else if roll < 75 {
                    Emotion::Laugh
                } else {
                    Emotion::Oops
                };

                *emotion_state.lock().unwrap() = new_emotion;
            }

            thread::sleep(Duration::from_millis(300));
        }
    });
}
