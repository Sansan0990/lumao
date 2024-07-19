use chrono::Utc;

#[derive(PartialEq, Clone, Debug)]
pub struct Mail{
    pub data:Vec<MailInfo>
}
impl Mail{
    pub fn new() ->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
}
#[derive(PartialEq,Clone,Debug)]
pub struct MailInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    expire_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:FeeLabel,
    office_url:String,
    status:OrderStatus
}

#[derive(PartialEq, Clone, Debug)]
pub struct TG{
    data:Vec<TGInfo>
}
impl TG{
    pub fn new()->Self{
        let mut out  = Self{
            data:Vec::new()
        };
        out
    }
}
#[derive(PartialEq,Clone,Debug)]
struct TGInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:FeeLabel,
    office_url:String,
    accounts:Vec<String>
}
#[derive(PartialEq, Clone, Debug)]
pub struct DC {
    data:Vec<DCInfo>
}
impl DC{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
}
#[derive(PartialEq,Clone,Debug)]
struct DCInfo{
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
    data:Vec<XInfo>
}
impl X{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
}
#[derive(PartialEq,Clone,Debug)]
struct XInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:FeeLabel,
    office_url:String,
    accounts:Vec<String>
}

#[derive(PartialEq, Clone, Debug)]
pub struct Ip{
    data:Vec<IpInfo>
}
impl Ip{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
}
#[derive(PartialEq,Clone,Debug)]
struct IpInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:FeeLabel,
    office_url:String,
    accounts:Vec<String>
}

#[derive(PartialEq,Clone)]
pub struct Browser{
    data:Vec<BrowserInfo>
}
impl Browser{
    pub fn new()->Self{
        let mut out = Self{
            data:Vec::new()
        };
        out
    }
}
#[derive(PartialEq,Clone)]
struct BrowserInfo{
    name:String,
    order_number:String,
    billing_date:chrono::DateTime<Utc>,
    expired_date:chrono::DateTime<Utc>,
    fee:u32,
    fee_label:FeeLabel,
    office_url:String,
    status:OrderStatus
}
#[derive(PartialEq, Clone, Debug)]
enum FeeLabel{
    RMB,
    USD,
}

#[derive(PartialEq, Clone, Debug)]
enum OrderStatus{
    InPeriod,
    Expired,
}

