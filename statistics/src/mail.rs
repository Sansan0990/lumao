use std::borrow::Cow;
use std::cmp::Ordering;
use std::hash::Hash;
use chrono::Utc;
use crate::pay_tab;
use egui_data_table::{RowViewer, UiAction};
use egui_data_table::egui::{KeyboardShortcut, Response, Ui};
use egui_data_table::viewer::{CellWriteContext, default_hotkeys, UiActionContext};

pub struct Viewer{
    pub select_all:bool,
    pub search_text:String,
    pub row_protection:bool,
    pub hotkeys:Vec<(KeyboardShortcut, UiAction)>
}
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
    pub selected: bool,
    pub name:String,
    pub order_number:String,
    pub billing_date:chrono::DateTime<Utc>,
    pub expire_date:chrono::DateTime<Utc>,
    pub fee:u32,
    pub fee_label: pay_tab::FeeLabel,
    pub office_url:String,
    pub status: pay_tab::OrderStatus
}
impl RowViewer<MailInfo> for Viewer{
    fn num_columns(&mut self) -> usize {
        9
    }
    fn column_name(&mut self, column: usize) -> Cow<'static, str> {
        [
            "Is Selected",
            "Name",
            "Order Number",
            "Billing Date",
            "Expire Date",
            "Fee",
            "Fee Label",
            "Office Link",
            "Status"
        ][column].into()
    }
    fn is_sortable_column(&mut self, column: usize) -> bool {
        [false,true,false,true,true,true,false,false,true][column]
    }
    fn create_cell_comparator(&mut self) -> impl Fn(&MailInfo, &MailInfo, usize) -> Ordering {
        fn cmp(row_l:&MailInfo,row_r:&MailInfo,column:usize)->std::cmp::Ordering{
            match column {
                0=>unreachable!(),
                1=>row_l.name.cmp(&row_r.name),
                2=>unreachable!(),
                3=>row_l.billing_date.cmp(&row_r.billing_date),
                4=>row_l.expire_date.cmp(&row_r.expire_date),
                5=>row_l.fee.cmp(&row_r.fee),
                6=>unreachable!(),
                7=>unreachable!(),
                8=>row_l.status.cmp(&row_r.status),
                _=>unreachable!(),
            }
        }
        cmp
    }
    fn row_filter_hash(&mut self) -> &impl Hash {
        &self.search_text
    }
    fn create_row_filter(&mut self) -> impl Fn(&MailInfo) -> bool {
        |r|
        r.name.contains(&self.search_text)
            || r.order_number.contains(&self.search_text)
            || r.billing_date.to_rfc3339().contains(&self.search_text)
            || r.expire_date.to_rfc3339().contains(&self.search_text)
            || format!("{}",r.fee).contains(&self.search_text)
            || format!("{:?}", r.fee_label).contains(&self.search_text)
            || r.office_url.contains(&self.search_text)
            || format!("{:?}",r.status).contains(&self.search_text)
    }
    fn show_cell_view(&mut self, ui: &mut Ui, row: &MailInfo, column: usize) {
        let _ = match column {
            0 => ui.checkbox(&mut { row.selected }, ""),
            1 => ui.label(&row.name),
            2 => ui.label(&row.order_number),
            3 => ui.label(&row.billing_date.to_rfc3339()),
            4 => ui.label(&row.expire_date.to_rfc3339()),
            5 => ui.label(format!("{}",row.fee)),
            6 => ui.label(format!("{:?}",row.fee_label)),
            7 => ui.hyperlink(&row.office_url),
            8 => ui.label(format!("{:?}",row.status)),
            _ => unreachable!(),
        };
    }
    //fn on_cell_view_response(&mut self, row: &MailInfo, column: usize, resp: &Response) -> Option<Box<MailInfo>> {
    //}
    fn show_cell_editor(&mut self, ui: &mut Ui, row: &mut MailInfo, column: usize) -> Option<Response> {
        match column {
            0 => ui.checkbox(&mut  row.selected , ""),
            _ => unreachable!(),
        }.into()
    }
    fn set_cell_value(&mut self, src: &MailInfo, dst: &mut MailInfo, column: usize) {
        match column {
                0=>dst.selected = src.selected,
                1=>dst.name.clone_from(&src.name),
                2=>dst.order_number.clone_from(&src.order_number),
                3=>dst.billing_date.clone_from(&src.billing_date),
                4=>dst.expire_date.clone_from(&src.expire_date),
                5=>dst.fee.clone_from(&src.fee),
                6=>dst.fee_label.clone_from(&src.fee_label),
                7=>dst.office_url.clone_from(&src.office_url),
                8=>dst.status.clone_from(&src.status),
                _=>unreachable!(),
            }
    }
    fn confirm_cell_write_by_ui(&mut self, current: &MailInfo, next: &MailInfo, column: usize, context: CellWriteContext) -> bool {
        if !self.row_protection {
            return true
        }
        false
    }
    fn confirm_row_deletion_by_ui(&mut self, row: &MailInfo) -> bool {
        if !self.row_protection {
            return true
        }
        false
    }
    fn new_empty_row(&mut self) -> MailInfo {
        MailInfo{
            selected: false,
            name: "".to_string(),
            order_number: "".to_string(),
            billing_date: Default::default(),
            expire_date: Default::default(),
            fee: 0,
            fee_label: pay_tab::FeeLabel::RMB,
            office_url: "".to_string(),
            status: pay_tab::OrderStatus::InPeriod,
        }
    }
    fn hotkeys(&mut self, context: &UiActionContext) -> Vec<(KeyboardShortcut, UiAction)> {
        let hotkeys = default_hotkeys(context);
        self.hotkeys.clone_from(&hotkeys);
        hotkeys
    }

    fn persist_ui_state(&self) -> bool {
        true
    }
}