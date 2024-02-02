// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;
// use std::time::{Duration, Instant};

// use nito::Simulation;

// pub fn simulation_logic(
//     simulation: Arc<Mutex<Simulation>>,
//     dump: Arc<Mutex<Vec<u8>>>,
//     sender: mpsc::Sender<bool>,
//     frequency: f64,
// ) {
//     println!("Simulation thread started!");
//     let d = Duration::from_secs_f64(frequency);
//     let mut last_frame_time = Instant::now();
//     loop {
//         {
//             let mut dump = dump.lock().unwrap();
//             *dump = simulation.lock().unwrap().dump(false);
//             sender.send(true).unwrap();
//         }

//         {
//             let mut simulation = simulation.lock().unwrap();
//             simulation.update();
//         }

//         let now = Instant::now();
//         let elapsed_time = now.duration_since(last_frame_time);
//         if elapsed_time < d {
//             thread::sleep(d - elapsed_time);
//         }
//         last_frame_time = Instant::now();
//     }
// }
