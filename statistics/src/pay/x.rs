use super::unit::FeeLabel;
use chrono::{Utc};
use eframe::egui::Context;

#[derive(PartialEq,Clone,Debug)]
struct XInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:FeeLabel,
    office_url:String,
    accounts:Vec<String>
}
#[derive(PartialEq, Clone, Debug)]
pub struct X{
    data:Vec<XInfo>
}
impl Default for X{
    fn default() -> Self {
        Self{
            data:Vec::new()
        }
    }
}
impl X{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
    pub fn show(&mut self,ctx:&Context){

    }
}