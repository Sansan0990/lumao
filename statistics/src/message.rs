use crate::mail;
use crate::pay_tab::{DC, Ip, TG, X};

#[derive(Clone, Debug)]
pub enum Message{
    MailReq,
    MailRes(mail::Mail),
    TGReq,
    TGRes(TG),
    DCReq,
    DCRes(DC),
    XReq,
    XRes(X),
    IPReq,
    IPRes(Ip)
}