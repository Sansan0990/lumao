use eframe::egui::Context;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use crate::pay::browser::BrowserInfo;
use crate::pay::discord::DCInfo;
use crate::pay::ip::IpInfo;
use crate::pay::mail::MailInfo;
use crate::pay::telegram::TGInfo;
use crate::pay::x::XInfo;
use super::pay;
#[derive(PartialEq,EnumIter,AsRefStr)]
pub enum PayNavigation{
    Mail,
    TG,
    DC,
    X,
    IP,
    Browser
}
pub struct PayPage{
    page_mail:pay::mail::Mail,
    page_browser:pay::browser::Browser,
    page_discord:pay::discord::DC,
    page_telegram:pay::telegram::TG,
    page_ip:pay::ip::Ip,
    page_x:pay::x::X,
    pay_navigation: PayNavigation
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
            pay_navigation : PayNavigation::Mail
        }
    }
}
impl PayPage{
    pub fn show(&mut self,ctx:&Context){
        egui::TopBottomPanel::top("pay_page_top").show(ctx,|ui|
        {
            ui.horizontal(|ui|{
                for page in PayNavigation::iter() {
                    if ui.selectable_label(self.pay_navigation == page,page.as_ref()).clicked(){
                        self.pay_navigation = page
                    }
                }
            })
        });
    }

    pub fn processing_mail_data_res(&mut self,data:Vec<MailInfo>){
        self.page_mail.processing_data_res(data)
    }
    pub fn processing_dc_data_res(&mut self,data:Vec<DCInfo>){
        self.page_discord.processing_data_res(data)
    }
    pub fn processing_browser_data_res(&mut self,data:Vec<BrowserInfo>){
        self.page_browser.processing_data_res(data)
    }
    pub fn processing_ip_data_res(&mut self,data:Vec<IpInfo>){
        self.page_ip.processing_data_res(data)
    }
    pub fn processing_tg_data_res(&mut self,data:Vec<TGInfo>){
        self.page_telegram.processing_data_res(data)
    }
    pub fn processing_x_data_res(&mut self,data:Vec<XInfo>){
        self.page_x.processing_data_res(data)
    }


}


















