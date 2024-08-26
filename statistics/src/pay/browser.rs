use super::unit;
use chrono::{Utc};
use eframe::egui;

#[derive(PartialEq,Clone)]
pub struct BrowserInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    expired_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label: unit::FeeLabel,
    office_url:String,
    status: unit::OrderStatus
}
#[derive(PartialEq,Clone)]
pub struct Browser{
    data:Vec<BrowserInfo>
}
impl Default for Browser{
    fn default() -> Self {
        Self{
            data: Vec::new()
        }
    }
}
impl Browser{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
    pub fn show(&mut self,ctx:&egui::Context){

    }
    pub fn processing_data_res(&mut self,data:Vec<BrowserInfo>){
        self.data = data
    }
}