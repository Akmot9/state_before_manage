use std::{collections::HashMap, sync::{Arc, Mutex}, thread, time::Duration};

use rand::Rng;

pub struct SonarState {
    matrice: HashMap<String, HashMap<u32, u32>>, // Thread -> (valeur -> occurrences)
    recording: bool,
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
            .map(|(thread, values)| {
                let values_str: String = values
                    .iter()
                    .map(|(value, occurrences)| format!("{}: {}", value, occurrences))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{} -> [{}]", thread, values_str)
            })
            .collect::<Vec<String>>()
            .join("; ");
        Ok(formatted_data)
    }

    pub fn set_recording(&mut self, recording: bool) {
        self.recording = recording;
    }

    pub fn update_matrice(&mut self, thread_name: String, value: u32) {
        let thread_entry = self.matrice
            .entry(thread_name)
            .or_insert_with(HashMap::new);
        let counter = thread_entry
            .entry(value)
            .or_insert(0);
        *counter += 1;
    }
}

fn start_thread(state: Arc<Mutex<SonarState>>, thread_name: String) {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            {
                //1. lock the state
                let mut locked_state = state.lock().unwrap();
                if locked_state.recording {
                    let value = rng.gen_range(1..100);
                    //2. update the state.matrice using the new method
                    locked_state.update_matrice(thread_name.clone(), value);
                    println!("{} added value: {} (occurrences: {})", thread_name, value, locked_state.matrice[&thread_name][&value]);
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
