#[derive(PartialEq, Clone, Debug)]
pub enum FeeLabel{
    RMB,
    USD,
}

#[derive(PartialEq, Clone, Debug)]
pub enum OrderStatus{
    InPeriod,
    Expired,
}