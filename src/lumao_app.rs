use eframe::egui::Context;
use eframe::{egui, Frame};

pub struct LumaoApp{
    pub name :String,
    pub age: u32
}

impl Default for LumaoApp{
    fn default() -> Self {
        Self {
            name :"sansan".to_owned(),
            age:33
        }
    }
}

impl eframe::App for LumaoApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx,|ui|{
            ui.heading("My egui app");
            ui.horizontal(|ui|{
                let name_label = ui.label("Your name");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id)
            });
            ui.add(egui::Slider::new(&mut self.age,0..=120).text("age"));
            if ui.button("Increment").clicked(){
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}",self.name,self.age));
        });
    }
}