mod lumao_app;
mod pay_tab;
mod message;
mod mail;

use std::path;
use eframe::egui;
use log::LevelFilter;
use tokio::runtime::Runtime;
use flume;
use crate::lumao_app::LumaoApp;
use crate::message::Message;

fn main() ->eframe::Result<()>{
    if let Err(_) =  log_init::init_log_file_stdout(path::Path::new("log"), LevelFilter::Info, LevelFilter::Info){
        println!("Unable to create log.")
    }
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0,720.0]),
        ..Default::default()
    };
    let runtime = Runtime::new().expect("Unable to create Runtime.");
    let _enter = runtime.enter();
    let (req_tx,req_rx) = flume::unbounded();
    let (res_tx,res_rx) = flume::unbounded();
    std::thread::spawn(move ||{
       runtime.block_on(async {
           loop {
               tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
               res_tx.send(Message::MailRes(mail::Mail::new())).unwrap()
           }
       })
    });
    eframe::run_native(
        "lumao statistic",
        options,
        Box::new(|cc|{
            Box::new(LumaoApp::new(res_rx,req_tx))
        })
    )
}
