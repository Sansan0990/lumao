use super::unit::FeeLabel;
use chrono::{Utc};
use eframe::egui::Context;
use crate::pay::telegram::TGInfo;

#[derive(PartialEq,Clone,Debug)]
pub struct XInfo{
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
    pub data:Vec<XInfo>
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
    pub fn processing_data_res(&mut self,data:Vec<XInfo>){
        self.data = data
    }}