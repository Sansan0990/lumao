
use std::env;
use std::time::Duration;
use eframe::egui::{Align, Context, global_dark_light_mode_switch,  Layout, Ui, Widget};
use eframe::{egui, Frame};
use egui_extras::{Column, TableBuilder};
use flume::TryRecvError;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use crate::message::Message;
use crate::pay_page::PayPage;
use crate::receive_page::ReceivePage;

#[derive(PartialEq,EnumIter,AsRefStr)]
pub enum MainNavigation{
    Pay,
    Receive
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
    main_navigation: MainNavigation,
    pay_page: PayPage,
    receive_page:ReceivePage,
    // ui state
    mail_ui_ctr:MailUiCtr
}

impl LumaoApp {
    pub fn new(res_rx:flume::Receiver<Message>,req_tx:flume::Sender<Message>)->Self{
        let mut out = Self{
            res_rx,
            req_tx,
            main_navigation:MainNavigation::Pay,
            pay_page:PayPage::default(),
            receive_page:ReceivePage::default(),
            mail_ui_ctr:MailUiCtr::new()
        };
        out
    }
    fn show_top_bar(&mut self,ui: &mut Ui){
        ui.horizontal(|ui|{
           ui.menu_button("file",|ui|{
           }) ;
            ui.menu_button("help",|ui|{

            });
            ui.separator();
            for page in MainNavigation::iter(){
                if ui.selectable_label(self.main_navigation == page,page.as_ref()).clicked(){
                    self.main_navigation = page
                }
            }
            ui.with_layout(Layout::right_to_left(Align::Center),|ui|{
                global_dark_light_mode_switch(ui)
            })
        });

    }
    fn left_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            if ui.selectable_value(&mut self.main_navigation,MainNavigation::Pay,"Pay").clicked() {
                self.main_navigation = MainNavigation::Pay
            }
            if ui.selectable_value(&mut self.main_navigation,MainNavigation::Receive,"Receive").clicked() {
                self.main_navigation = MainNavigation::Receive
            }
        });
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
    fn main_ui(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx,|ui|{
           self.show_top_bar(ui)
        });
        match self.main_navigation{
            MainNavigation::Pay=>{
                self.pay_page.show(ctx)
            },
            MainNavigation::Receive=>{
                self.receive_page.show(ctx)
            }
        }
    }
    fn processing_data_res(&mut self){
        if let Some(o) = self.try_receive_data(){
            match o {
                Message::MailRes(v) => {
                    self.pay_page.processing_mail_data_res(v.data)
                }
                Message::DCRes(v) => {
                    self.pay_page.processing_dc_data_res(v.data)
                }
                Message::XRes(v) => {
                    self.pay_page.processing_x_data_res(v.data)
                }
                Message::IPRes(v) => {
                   self.pay_page.processing_ip_data_res(v.data)
                }
                Message::TGRes(v)=>{}
                _ => {}
            }
        }

    }
}

impl eframe::App for LumaoApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.processing_data_res();
        self.main_ui(ctx,frame);
        ctx.request_repaint_after(Duration::from_millis(100))
    }
}