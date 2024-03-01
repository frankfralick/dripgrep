// use tokio::sync::{mpsc, Mutex};
//
// pub struct Logger {
//     pub sender: mpsc::Sender<String>,
// }
//
// impl Logger {
//     pub async fn log(&self, message: String) {
//         let _ = self.sender.send(message).await;
//     }
// }
//
// pub async fn logger_task(mut receiver: mpsc::Receiver<String>) {
//     while let Some(message) = receiver.recv().await {
//         println!("{}", message);
//     }
// }
