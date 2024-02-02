// use std::io::Write;
// use std::net::TcpStream;
// use std::sync::{mpsc, Arc, Mutex};

// pub fn dispatcher_logic(
//     receiver: mpsc::Receiver<bool>,
//     dump: Arc<Mutex<Vec<u8>>>,
//     clients: Arc<Mutex<Vec<TcpStream>>>,
// ) {
//     println!("Dispatcher thread started!");
//     let mut disconnects = vec![];
//     loop {
//         let msg = receiver.recv().unwrap();
//         if msg {
//             let mut clients = clients.lock().unwrap();
//             let dump = dump.lock().unwrap();
//             let mut i = 0;
//             for con in clients.iter_mut() {
//                 match con.write(dump.as_slice()) {
//                     Ok(_) => {}
//                     Err(_) => {
//                         disconnects.push(i);
//                     }
//                 }
//                 i += 1;
//             }
//             for i in disconnects.iter().rev() {
//                 clients.remove(*i);
//             }
//             disconnects.clear();
//         }
//     }
// }
