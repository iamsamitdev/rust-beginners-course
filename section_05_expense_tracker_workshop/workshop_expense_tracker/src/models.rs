use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryType {
    Income,
    Expense
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    pub date: NaiveDate,
    pub category: String,
    pub amount: f64,
    pub entry_type: EntryType,
}