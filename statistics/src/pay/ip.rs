use super::unit;
use chrono::{Utc};
use eframe::egui::Context;

#[derive(PartialEq,Clone,Debug)]
struct IpInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:unit::FeeLabel,
    office_url:String,
    accounts:Vec<String>
}
#[derive(PartialEq, Clone, Debug)]
pub struct Ip{
    data:Vec<IpInfo>
}
impl Default for Ip{
    fn default() -> Self {
        Self{
            data: Vec::new()
        }
    }
}
impl Ip{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
    pub fn show(&mut self,ctx:&Context){

    }
}
