use chrono::NaiveDate;
use workshop_expense_tracker::models::{Entry, EntryType};
use workshop_expense_tracker::validation::{parse_entry_type, validate_amount, validate_category};
use workshop_expense_tracker::{add::new_entry, summary::calculate_balance};

// ทดสอบการสร้างรายการใหม่ entry ประเภทรายรับที่ถูกต้อง
#[test]
fn test_valid_income_entry() {
    let entry = new_entry("2025-07-01", "เงินเดือน", 15000.0, "income").unwrap();
    assert_eq!(entry.entry_type, EntryType::Income);
    assert_eq!(entry.category, "เงินเดือน");
    assert_eq!(entry.amount, 15000.0);
}

// ทดสอบการสร้าง entry ประเภทรายจ่ายที่ถูกต้อง
#[test]
fn test_valid_expense_entry() {
    let entry = new_entry("2025-07-02", "กาแฟ", 60.0, "expense").unwrap();
    assert_eq!(entry.entry_type, EntryType::Expense);
    assert_eq!(entry.category, "กาแฟ");
    assert_eq!(entry.amount, 60.0);
}

// ทดสอบการสร้าง entry ด้วยประเภทที่ไม่ถูกต้อง (ต้อง error)
#[test]
fn test_invalid_entry_type() {
    let result = new_entry("2025-07-03", "อื่นๆ", 100.0, "other");
    assert!(result.is_err());
}

// ทดสอบการคำนวณยอดเงินคงเหลือจากรายการที่มีข้อมูล
#[test]
fn test_calculate_balance_normal() {
    let entries = vec![
        new_entry("2025-07-01", "เงินเดือน", 10000.0, "income").unwrap(),
        new_entry("2025-07-02", "ค่าเช่า", 3000.0, "expense").unwrap(),
        new_entry("2025-07-03", "ข้าว", 150.0, "expense").unwrap(),
    ];

    let (income, expense, balance) = calculate_balance(&entries);
    assert_eq!(income, 10000.0);
    assert_eq!(expense, 3150.0);
    assert_eq!(balance, 6850.0);
}

// ทดสอบการคำนวณยอดเงินคงเหลือจากรายการที่ว่างเปล่า
#[test]
fn test_calculate_balance_empty() {
    let entries: Vec<Entry> = Vec::new();
    let (income, expense, balance) = calculate_balance(&entries);
    assert_eq!(income, 0.0);
    assert_eq!(expense, 0.0);
    assert_eq!(balance, 0.0);
}

// ทดสอบการสร้าง Entry struct โดยตรงแบบ manual
#[test]
fn test_entry_direct_create() {
    let entry = Entry {
        date: NaiveDate::from_ymd_opt(2025, 7, 4).unwrap(),
        category: "โบนัส".to_string(),
        amount: 5000.0,
        entry_type: EntryType::Income,
    };
    assert_eq!(entry.amount, 5000.0);
}

// ทดสอบการตรวจสอบจำนวนเงินที่ถูกต้อง (ผ่านเงื่อนไข)
#[test]
fn test_validate_amount_valid() {
    assert!(validate_amount(100.0).is_ok());
    assert!(validate_amount(0.01).is_ok());
    assert!(validate_amount(999999.99).is_ok());
}

// ทดสอบการตรวจสอบจำนวนเงินที่ไม่ถูกต้อง (ต้อง error)
#[test]
fn test_validate_amount_invalid() {
    assert!(validate_amount(0.0).is_err());
    assert!(validate_amount(-100.0).is_err());
    assert!(validate_amount(1_000_001.0).is_err());
}

// ทดสอบการตรวจสอบหมวดหมู่ที่ถูกต้อง (ผ่านเงื่อนไข)
#[test]
fn test_validate_category_valid() {
    assert!(validate_category("กาแฟ").is_ok());
    assert!(validate_category("  เงินเดือน  ").is_ok()); // should trim
}

// ทดสอบการตรวจสอบหมวดหมู่ที่ไม่ถูกต้อง (ต้อง error)
#[test]
fn test_validate_category_invalid() {
    assert!(validate_category("").is_err());
    assert!(validate_category("   ").is_err()); // only spaces
    assert!(validate_category(&"a".repeat(51)).is_err()); // too long
}

// ทดสอบการแปลงประเภทรายการที่ถูกต้อง (ผ่านเงื่อนไข)
#[test]
fn test_parse_entry_type_valid() {
    assert!(parse_entry_type("income").is_ok());
    assert!(parse_entry_type("expense").is_ok());
    assert!(parse_entry_type("INCOME").is_ok()); // case insensitive
    assert!(parse_entry_type("รายรับ").is_ok());
    assert!(parse_entry_type("รายจ่าย").is_ok());
}

// ทดสอบการแปลงประเภทรายการที่ไม่ถูกต้อง (ต้อง error)
#[test]
fn test_parse_entry_type_invalid() {
    assert!(parse_entry_type("other").is_err());
    assert!(parse_entry_type("").is_err());
    assert!(parse_entry_type("money").is_err());
}
