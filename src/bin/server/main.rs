// use std::io::Write;
// #[allow(dead_code, unused)]
// use std::net::TcpListener;
// use std::sync::mpsc::channel;
// use std::sync::{Arc, Mutex};
// use std::thread;

// use nito::{open_config, Simulation};

// use crate::dispatcher_thread::dispatcher_logic;
// use crate::input_thread::handle_connection;
// use crate::simulation_thread::simulation_logic;

// mod dispatcher_thread;
// mod input_thread;
// mod simulation_thread;


// fn main() -> std::io::Result<()> {
//     let cfg = open_config();

//     // Start listening for incoming connections
//     let addr = format!("{}:{}", cfg.endpoint.address, cfg.endpoint.port);
//     let listener = TcpListener::bind(addr).expect("Failed to bind to address");
//     println!("Server running and listening on port {}", cfg.endpoint.port);

//     // Create shared data
//     let mut handles = Vec::new();
//     let clients = Arc::new(Mutex::new(Vec::new()));
//     let dump = Arc::new(Mutex::new(Vec::new()));
//     let (sender, receiver) = channel();
//     let simulation = Arc::new(Mutex::new(Simulation::new(
//         cfg.world.x as usize,
//         cfg.world.y as usize,
//     )));

//     // Simulation thread
//     // let dump_clone = Arc::clone(&dump);
//     // let sim_clone = Arc::clone(&simulation);
//     // let handle = thread::Builder::new()
//     //     .name("simulation".to_string())
//     //     .spawn(move || {
//             simulation_logic(
//                 sim_clone,
//                 dump_clone,
//                 sender,
//                 1.0 / cfg.world.frequency as f64,
//             );
//     //     });
//     // handles.push(handle);

//     // Dispatcher thread
//     // let dump_clone = Arc::clone(&dump);
//     // let clients_clone = Arc::clone(&clients);
//     // let handle = thread::Builder::new()
//     //     .name("dispatcher".to_string())
//     //     .spawn(move || {
//     //         dispatcher_logic(receiver, dump_clone, clients_clone);
//     //     });
//     // handles.push(handle);

//     // Listen for incoming connection
//     // for stream in listener.incoming() {
//     //     let clients = Arc::clone(&clients);

//     //     match stream {
//     //         Ok(stream) => {
//     //             println!("Received new connection");
//     //             // Add client to clients vector
//     //             let mut clients = clients.lock().unwrap();
//     //             let mut client = stream.try_clone().expect("Failed to clone connection");

//     //             // On new connection spawn a thread
//     //             let clone_stream = stream.try_clone().expect("Failed to clone connection");
//     //             let sim_clone = Arc::clone(&simulation);
//     //             let mut dump = dump.lock().unwrap();
//     //             *dump = sim_clone.lock().unwrap().dump(true);
//     //             client.write(dump.as_slice()).unwrap();
//     //             clients.push(client);
//     //             let handle = thread::Builder::new()
//     //                 .name(format!("client#{}", clients.len()).to_string())
//     //                 .spawn(move || {
//     //                     handle_connection(sim_clone, clone_stream)
//     //                         .expect("Failed to handle connection");
//     //                 });
//     //             handles.push(handle);
//     //         }
//     //         Err(e) => {
//     //             eprintln!("Failed to establish a connection: {}", e);
//     //         }
//     //     };
//     // }

//     // for handle in handles {
//     //     handle.unwrap().join().unwrap();
//     // }

//     Ok(())
// }
