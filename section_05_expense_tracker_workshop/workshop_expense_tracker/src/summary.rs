use crate::config::{
    CSV_HAS_HEADERS, EXPENSES_FILE, TABLE_AMOUNT_WIDTH, TABLE_DATE_WIDTH, TABLE_TYPE_WIDTH,
};
use crate::errors::ExpenseError;
use crate::models::{Entry, EntryType};
use csv::ReaderBuilder;
use std::fs::OpenOptions;

// ฟังก์ชันในการแสดงรายการที่บันทึกไว้ใน csv
pub fn list_entries() -> Result<(), ExpenseError> {
    let file = match OpenOptions::new().read(true).open(EXPENSES_FILE) {
        Ok(file) => file,
        Err(_) => {
            println!("📭 ยังไม่มีรายการ กรุณาเพิ่มรายการก่อน");
            return Ok(());
        }
    };

    let mut rdr = ReaderBuilder::new()
        .has_headers(CSV_HAS_HEADERS)
        .from_reader(file);

    println!("\n📄 รายการทั้งหมด:");
    println!(
        "{:<width_date$} {:<width_type$} {:<width_amount$} หมวดหมู่",
        "วันที่",
        "ประเภท",
        "จำนวน",
        width_date = TABLE_DATE_WIDTH,
        width_type = TABLE_TYPE_WIDTH,
        width_amount = TABLE_AMOUNT_WIDTH
    );
    println!("{:-<60}", ""); // separator line

    for result in rdr.deserialize::<Entry>() {
        let entry = result?;
        let entry_type = match entry.entry_type {
            EntryType::Income => "รายรับ",
            EntryType::Expense => "รายจ่าย",
        };

        println!(
            "{:<width_date$} {:<width_type$} {:<width_amount$.2} {}",
            entry.date,
            entry_type,
            entry.amount,
            entry.category,
            width_date = TABLE_DATE_WIDTH,
            width_type = TABLE_TYPE_WIDTH,
            width_amount = TABLE_AMOUNT_WIDTH
        );
    }

    Ok(())
}

// ฟังก์ชันในการคำนวณยอดรวม
pub fn calculate_summary() -> Result<(), ExpenseError> {
    let file = match OpenOptions::new().read(true).open(EXPENSES_FILE) {
        Ok(file) => file,
        Err(_) => {
            println!("📭 ยังไม่มีรายการ กรุณาเพิ่มรายการก่อน");
            return Ok(());
        }
    };

    let mut rdr = ReaderBuilder::new()
        .has_headers(CSV_HAS_HEADERS)
        .from_reader(file);

    let mut income_total = 0.0;
    let mut expense_total = 0.0;
    let mut entry_count = 0;

    for result in rdr.deserialize::<Entry>() {
        let entry = result?;
        entry_count += 1;
        match entry.entry_type {
            EntryType::Income => income_total += entry.amount,
            EntryType::Expense => expense_total += entry.amount,
        }
    }

    if entry_count == 0 {
        println!("📭 ยังไม่มีรายการ กรุณาเพิ่มรายการก่อน");
        return Ok(());
    }

    let balance = income_total - expense_total;

    println!("\n💹 สรุปยอดรวม:");
    println!("📊 จำนวนรายการ : {entry_count} รายการ");
    println!("💵 รายรับรวม    : {income_total:.2} บาท");
    println!("💸 รายจ่ายรวม   : {expense_total:.2} บาท");
    println!("📉 คงเหลือสุทธิ : {balance:.2} บาท");
    println!("{:-<60}", ""); // separator line
    if balance > 0.0 {
        println!("✅ คุณมีเงินเหลือ!");
    } else if balance < 0.0 {
        println!("⚠️ คุณใช้จ่ายเกินรายรับ!");
    } else {
        println!("⚖️ รายรับและรายจ่ายเท่ากัน");
    }

    Ok(())
}

// ฟังก์ชันคำนวณยอด balance
pub fn calculate_balance(entries: &[Entry]) -> (f64, f64, f64) {
    let income: f64 = entries
        .iter()
        .filter(|e| e.entry_type == EntryType::Income)
        .map(|e| e.amount)
        .sum();
    let expense: f64 = entries
        .iter()
        .filter(|e| e.entry_type == EntryType::Expense)
        .map(|e| e.amount)
        .sum();
    (income, expense, income - expense)
}
