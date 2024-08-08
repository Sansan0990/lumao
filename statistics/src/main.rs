mod lumao_app;
mod pay_tab;
mod message;
mod mail;

use std::iter::repeat_with;
use std::path;
use chrono::Utc;
use eframe::egui;
use log::LevelFilter;
use tokio::runtime::Runtime;
use flume;
use crate::lumao_app::LumaoApp;
use crate::mail::MailInfo;
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
    std::thread::spawn(move || {
        runtime.block_on(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            let mut m = mail::Mail::new();
            m.data = {
                let mut rng = fastrand::Rng::new();
                let mut name_gen = names::Generator::with_naming(names::Name::Numbered);
                repeat_with(move || {
                    MailInfo {
                        selected: false,
                        name: name_gen.next().unwrap(),
                        order_number: "fdjklafjd".to_string(),
                        billing_date: Utc::now(),
                        expire_date: Utc::now(),
                        fee: rng.u32(4..31),
                        fee_label: pay_tab::FeeLabel::RMB,
                        office_url: "www.google.com".to_owned(),
                        status: pay_tab::OrderStatus::Expired
                    }
                })
            }.take(10)
                .collect();
            res_tx.send(Message::MailRes(m)).unwrap()

            loop {}
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
