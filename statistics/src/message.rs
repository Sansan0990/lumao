use crate::pay::{discord::DC, ip::Ip, mail::Mail, telegram::TG, x::X};

#[derive(Clone, Debug)]
pub enum Message{
    MailReq,
    MailRes(Mail),
    TGReq,
    TGRes(TG),
    DCReq,
    DCRes(DC),
    XReq,
    XRes(X),
    IPReq,
    IPRes(Ip)
}