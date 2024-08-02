use std::env;
use std::iter::repeat_with;
use std::time::Duration;
use chrono::Utc;
use eframe::egui::{Context, include_image, Widget};
use eframe::{egui, Frame};
use egui_extras::{Column, TableBuilder};
use flume::TryRecvError;
use crate::message::Message;
use crate::{mail, pay_tab};
use crate::mail::MailInfo;

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
    mail:mail::Mail,
    mail_table:egui_data_table::DataTable<mail::MailInfo>,
    mail_viewer:mail::Viewer,
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
            mail:mail::Mail::new(),
            mail_table:egui_data_table::DataTable::new(),
            mail_viewer:mail::Viewer{select_all:false,row_protection:false,search_text:String::new(),hotkeys:Vec::new()},
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
                // 检查是否有新输入
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
        ui.separator();
        self.show_mail_table(ui)

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
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("right content")
                })
            });
        egui::TopBottomPanel::bottom("bottom panel")
            .resizable(true)
            .default_height(100.0)
            .height_range(50.0..=200.0)
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
    fn show_mail_table(&mut self,ui:&mut egui::Ui) {
        self.mail_table = {
            let mut rng = fastrand::Rng::new();
                let mut name_gen = names::Generator::with_naming(names::Name::Numbered);

                repeat_with(move || {
                    MailInfo {
                        selected:false,
                        name: name_gen.next().unwrap(),
                        order_number:"fdjklafjd".to_string(),
                        billing_date:Utc::now(),
                        expire_date:Utc::now(),
                        fee: rng.u32(4..31),
                        fee_label:pay_tab::FeeLabel::RMB,
                        office_url:"www.google.com".to_owned(),
                        status:pay_tab::OrderStatus::Expired
                    }

                })
            }
            .take(100000)
            .collect();
        fn is_send<T: Send>(_: &T) {}
        fn is_sync<T: Sync>(_: &T) {}

        is_send(&self.mail_table);
        is_sync(&self.mail_viewer);

        ui.add(egui_data_table::Renderer::new(
            &mut self.mail_table,
            &mut self.mail_viewer,
        ));

        //egui::Frame::default()
        //    .show(ui, |ui| {
        //        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
        //        egui_extras::TableBuilder::new(ui)
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .column(Column::auto())
        //            .header(20.0, |mut header| {
        //                header.col(|ui| {
        //                    ui.strong("selected");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("name");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("order number");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("billing date");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("expired date");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("fee");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("fee label");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("office link");
        //                });
        //                header.col(|ui| {
        //                    ui.strong("status");
        //                });
        //            }).body(|mut body| {
        //            body.row(20.0, |mut row| {
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //                row.col(|ui| {});
        //            })
        //        })
        //    });
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

