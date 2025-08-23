use crate::config::{CSV_HAS_HEADERS, EXPENSES_FILE};
use crate::errors::ExpenseError;
use crate::models::Entry;
use crate::validation::{parse_entry_type, validate_amount, validate_category, validate_date};
use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::io::{self, Write};

// สร้างฟังก์ชันสำหรับการเพิ่มรายการใหม่ลง csv
pub fn add_entry() -> Result<(), ExpenseError> {
    let mut input = String::new(); // รับข้อมูลจาก user ผ่าน cli

    // Get and validate date
    print!("📅 วันที่ (YYYY-MM-DD): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let date = validate_date(input.trim())?;

    input.clear();

    // Get and validate entry type
    print!("💸 ประเภท (income/expense): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let entry_type = parse_entry_type(input.trim())?;

    input.clear();

    // Get and validate category
    print!("📂 หมวดหมู่: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let category = validate_category(input.trim())?;

    input.clear();

    // Get and validate amount
    print!("💰 จำนวนเงิน: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let amount = input
        .trim()
        .parse::<f64>()
        .map_err(|_| ExpenseError::ParseError("จำนวนเงินต้องเป็นตัวเลข".to_string()))?;
    let amount = validate_amount(amount)?;

    let entry = Entry {
        date,
        category,
        amount,
        entry_type,
    };

    // เปิดไฟล์ CSV
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(EXPENSES_FILE)?;

    // เขียนลงไฟล์ CSV
    let mut wtr = WriterBuilder::new()
        .has_headers(CSV_HAS_HEADERS)
        .from_writer(file);
    wtr.serialize(entry)?;
    wtr.flush()?;

    println!("✅ บันทึกสำเร็จ");
    Ok(())
}

// ฟังก์ชันแสดงรายการที่เพิ่มสำเร็จ
pub fn new_entry(
    date: &str,
    category: &str,
    amount: f64,
    kind: &str,
) -> Result<Entry, ExpenseError> {
    let date = validate_date(date)?;
    let category = validate_category(category)?;
    let amount = validate_amount(amount)?;
    let entry_type = parse_entry_type(kind)?;

    Ok(Entry {
        date,
        category,
        amount,
        entry_type,
    })
}
