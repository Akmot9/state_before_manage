use std::{collections::HashMap, sync::{Arc, Mutex}, thread, time::Duration};

use rand::Rng;

pub struct SonarState {
    matrice: HashMap<String,u32>,
    recording: bool
}

impl SonarState {
    pub fn new() -> Arc<Mutex<Self>> {
        let state = Arc::new(Mutex::new(SonarState {
            matrice: HashMap::new(),
            recording: true,
        }));

        let state_clone1 = Arc::clone(&state);
        let state_clone2 = Arc::clone(&state);
        let state_clone3 = Arc::clone(&state);

        start_thread(state_clone1, "Thread1".to_string());
        start_thread(state_clone2, "Thread2".to_string());
        start_thread(state_clone3, "Thread3".to_string());

        state
    }

    pub fn get_matrice_data(&self) -> Result<String, String> {
        let formatted_data: String = self
            .matrice
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join(", ");
        Ok(formatted_data)
    }

    pub fn set_recording(&mut self, recording: bool) {
        self.recording = recording;
    }
}

fn start_thread(state: Arc<Mutex<SonarState>>, thread_name: String) {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            {
                let mut locked_state = state.lock().unwrap();
                if locked_state.recording {
                    let value = rng.gen_range(1..100);
                    locked_state.matrice.insert(thread_name.clone(), value);
                    println!("{} added value: {}", thread_name, value);
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}