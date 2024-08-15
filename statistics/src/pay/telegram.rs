use super::unit;
use chrono::{Utc};
use eframe::egui::Context;

#[derive(PartialEq,Clone,Debug)]
struct TGInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:unit::FeeLabel,
    office_url:String,
    accounts:Vec<String>
}
#[derive(PartialEq, Clone, Debug)]
pub struct TG{
    data:Vec<TGInfo>
}
impl Default for TG{
    fn default() -> Self {
        Self{
            data:Vec::new()
        }
    }
}
impl TG{
    pub fn new()->Self{
        let mut out  = Self{
            data:Vec::new()
        };
        out
    }
    pub fn show(&mut self,ctx:&Context){

    }
}