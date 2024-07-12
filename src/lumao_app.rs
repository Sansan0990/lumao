use eframe::egui::Context;
use eframe::{egui, Frame};

pub enum LeftNavigation{
    Pay,
    Receive
}
pub
pub struct LumaoApp{
    page:LeftNavigation
}
impl LumaoApp {
    fn left_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            if ui.button("Pay").clicked() {
                self.page = LeftNavigation::Pay
            }
            if ui.button("Receive").clicked() {
                self.page = LeftNavigation::Receive
            }
        });
    }
    fn center_ui(&mut self, ui: &mut egui::Ui) {
        match self.page {
            LeftNavigation::Pay => {
                self.page_pay(ui)
            }
            LeftNavigation::Receive => {
                self.page_receive(ui)
            }
        }
    }
    fn page_pay(&mut self, ui: &mut egui::Ui) {
        ui.label("pay");
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
impl Default for LumaoApp{
    fn default() -> Self {
        Self {
            page:LeftNavigation::Pay
        }
    }
}

impl eframe::App for LumaoApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::CentralPanel::default().show(ctx,|ui|{
           self.main_ui(ui)
        });
    }
}