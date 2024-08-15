use chrono::Utc;
use eframe::egui::Context;
use crate::pay;
pub struct PayPage{
    page_mail:pay::mail::Mail,
    page_browser:pay::browser::Browser,
    page_discord:pay::discord::DC,
    page_telegram:pay::telegram::TG,
    page_ip:pay::ip::Ip,
    page_x:pay::x::X
}
impl Default for PayPage{
    fn default() -> Self {
        Self{
            page_mail: pay::mail::Mail::default(),
            page_browser:pay::browser::Browser::default(),
            page_discord:pay::discord::DC::default(),
            page_telegram:pay::telegram::TG::default(),
            page_ip:pay::ip::Ip::default(),
            page_x:pay::x::X::default(),
        }
    }
}
impl PayPage{
    pub fn show(&mut self,ctx:&Context){

    }
}


















