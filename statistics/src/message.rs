use crate::pay_tab::{DC, Ip, Mail, TG, X};

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