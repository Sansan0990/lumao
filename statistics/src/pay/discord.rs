use egui::Context;
use chrono::{Utc};
use eframe::egui;
use super::unit;

#[derive(PartialEq,Clone,Debug)]
pub struct DCInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:unit::FeeLabel,
    office_url:String,
    accounts:Vec<String>
}
#[derive(PartialEq, Clone, Debug)]
pub struct DC {
    pub data:Vec<DCInfo>
}
impl Default for DC{
    fn default() -> Self {
        Self{
            data:Vec::new()
        }
    }
}
impl DC{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
    pub fn show(&mut self,ctx:&Context){

    }
    pub fn processing_data_res(&mut self,data:Vec<DCInfo>){
        self.data = data
    }
}