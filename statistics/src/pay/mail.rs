use chrono::Utc;
use eframe::egui::Context;
use super::unit;
//noinspection ALL
#[derive(PartialEq,Clone,Debug)]
pub struct MailInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    expire_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label: unit::FeeLabel,
    office_url:String,
    status: unit::OrderStatus
}
#[derive(PartialEq, Clone, Debug)]
pub struct Mail{
    pub data:Vec<MailInfo>
}
impl Default for Mail{
    fn default() -> Self {
        Self{
            data: Vec::new()
        }
    }
}

impl Mail{
    pub fn new() ->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
    pub fn show(&mut self,ctx:&Context){

    }
}