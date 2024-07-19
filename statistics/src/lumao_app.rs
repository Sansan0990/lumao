use std::env;
use std::time::Duration;
use eframe::egui::{Context, include_image, Widget};
use eframe::{egui, Frame};
use egui_extras::{Column, TableBuilder};
use flume::TryRecvError;
use crate::message::Message;
use crate::pay_tab;

#[derive(PartialEq)]
pub enum LeftNavigation{
    Pay,
    Receive
}
#[derive(PartialEq)]
pub enum PayNavigation{
    Mail,
    TG,
    DC,
    X,
    IP,
    Browser
}

pub struct MailUiCtr{
    select_all:bool,
    search_str:String,
}
impl MailUiCtr {
    pub fn new()->Self{
        let mut o = Self{
            select_all:false,
            search_str:String::new()
        };
        o
    }
}
pub struct LumaoApp{
    res_rx:flume::Receiver<Message>,
    req_tx:flume::Sender<Message>,
    left_navigation: LeftNavigation,
    pay_navigation: PayNavigation,
    // data
    mail:pay_tab::Mail,
    tg:pay_tab::TG,
    dc:pay_tab::DC,
    x:pay_tab::X,
    ip:pay_tab::Ip,
    browser:pay_tab::Browser,
    // ui state
    mail_ui_ctr:MailUiCtr
}

impl LumaoApp {
    pub fn new(res_rx:flume::Receiver<Message>,req_tx:flume::Sender<Message>)->Self{
        let mut out = Self{
            res_rx,
            req_tx,
            left_navigation:LeftNavigation::Pay,
            pay_navigation:PayNavigation::Mail,
            mail:pay_tab::Mail::new(),
            tg:pay_tab::TG::new(),
            dc:pay_tab::DC::new(),
            x:pay_tab::X::new(),
            ip:pay_tab::Ip::new(),
            browser:pay_tab::Browser::new(),
            mail_ui_ctr:MailUiCtr::new()
        };
        out
    }
    fn left_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            if ui.selectable_value(&mut self.left_navigation,LeftNavigation::Pay,"Pay").clicked() {
                self.left_navigation = LeftNavigation::Pay
            }
            if ui.selectable_value(&mut self.left_navigation,LeftNavigation::Receive,"Receive").clicked() {
                self.left_navigation = LeftNavigation::Receive
            }
        });
    }
    fn center_ui(&mut self, ui: &mut egui::Ui) {
        match self.left_navigation {
            LeftNavigation::Pay => {
                self.page_pay(ui)
            }
            LeftNavigation::Receive => {
                self.page_receive(ui)
            }
        }
    }
    fn page_pay(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                self.reset_data_from_res(self.try_receive_data());
                ui.selectable_value(&mut self.pay_navigation, PayNavigation::Mail, "Mail");
                ui.selectable_value(&mut self.pay_navigation, PayNavigation::TG, "TG");
                ui.selectable_value(&mut self.pay_navigation, PayNavigation::DC, "DC");
                ui.selectable_value(&mut self.pay_navigation, PayNavigation::X, "X");
                ui.selectable_value(&mut self.pay_navigation, PayNavigation::IP, "Ip");
                ui.selectable_value(&mut self.pay_navigation, PayNavigation::Browser, "Browser");

            });
            ui.separator();
            ui.vertical(|ui|{
               match self.pay_navigation {
                   PayNavigation::Mail => {
                       self.show_mail_page(ui)
                   }
                   PayNavigation::TG => {}
                   PayNavigation::DC => {}
                   PayNavigation::X => {}
                   PayNavigation::IP => {}
                   PayNavigation::Browser => {}
               }
            });
            ui.separator()
        });
    }
    fn reset_data_from_res(&mut self,op:Option<Message>){
        if let Some(m) = op{
            match m {
                Message::MailRes(v) => {
                    self.mail = v
                }
                Message::TGRes(v) => {
                    self.tg = v
                }
                Message::DCRes(v) => {
                    self.dc =v
                }
                Message::XRes(v) => {
                    self.x= v
                }
                Message::IPRes(v) => {
                    self.ip = v
                }
                _ =>{}
            }
        }
    }
    fn show_mail_page(&mut self,ui:&mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.mail_ui_ctr.select_all, "Select all");
            ui.separator();
            ui.menu_button("File",|ui|{
                if ui.button("Import").clicked(){

                }
            });
            ui.separator();
            if ui.button("Add").clicked(){

            };
            if ui.button("Delete").clicked(){

            }
            if ui.button("Edit").clicked(){

            }
            if ui.button("Import").clicked(){

            }
            ui.separator();
            ui.text_edit_singleline(&mut self.mail_ui_ctr.search_str);
            ui.button("Search")
            //ui.add(egui::ImageButton::new(include_image!("../../statistics/resource/add.png")));

        });
    }
    fn try_receive_data(&self)->Option<Message>{
        return match self.res_rx.try_recv() {
            Ok(o) => {
                Some(o)
            },
            Err(e) => {
                match e {
                    TryRecvError::Empty => {
                        None
                    }
                    TryRecvError::Disconnected => {
                        log::error!("{}",e.to_string());
                        None
                    }
                }
            }
        }
    }
    fn page_receive(&mut self, ui: &mut egui::Ui) {
        ui.label("receive");
    }
    fn main_ui(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.left_ui(ui)
                })
            });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(100.0..)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("right content")
                })
            });
        egui::TopBottomPanel::bottom("bottom panel")
            .resizable(true)
            .default_height(100.0)
            .height_range(50.0..)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui,|ui| {
                    ui.label("status content")
                });
            });
        egui::CentralPanel::default().show_inside(ui,|ui|{
            egui::ScrollArea::vertical().show(ui,|ui| {
                self.center_ui(ui)
            })
        });
    }
}

impl eframe::App for LumaoApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::CentralPanel::default().show(ctx,|ui|{
           self.main_ui(ui)
        });
        egui_extras::install_image_loaders(ctx);
        ctx.request_repaint_after(Duration::from_millis(100))
    }
}