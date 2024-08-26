use egui::Context;
use crate::pay_page::PayPage;

pub struct ReceivePage{
    name:String
}
impl Default for ReceivePage{
    fn default() -> Self {
        Self{
            name:String::new()
        }
    }
}
impl ReceivePage{
    pub fn show(&mut self,ctx:&Context){

    }
}