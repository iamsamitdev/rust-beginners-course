use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;
use workshop_expense_tracker::models::{Entry, EntryType};

// ทดสอบการอ่านไฟล์ CSV จาก mock data และแปลงเป็น Entry structs
// ตรวจสอบว่าสามารถ deserialize ข้อมูลจาก CSV ได้ถูกต้อง

// การทำงานของ test นี้:
// 1. เปิดไฟล์ tests/mock_data/mock_expenses.csv
// 2. ใช้ csv::ReaderBuilder อ่านไฟล์โดยไม่มี headers
// 3. แปลงแต่ละแถวเป็น Entry struct ผ่าน deserialize
// 4. ตรวจสอบว่า:
//      - จำนวนรายการที่อ่านได้ = 3 รายการ
//      - รายการแรกมี category = "เงินเดือน"
//      - รายการที่สองเป็นประเภท Expense
//      - มีรายการที่มีจำนวนเงินมากกว่า 1000.0

#[test]
fn test_read_mock_csv_and_parse_entries() {
    let path = Path::new("tests/mock_data/mock_expenses.csv");
    let file = File::open(path).expect("❌ ไม่สามารถเปิดไฟล์ mock_expenses.csv");

    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut entries: Vec<Entry> = Vec::new();

    for result in rdr.deserialize::<Entry>() {
        let entry = result.expect("❌ ไม่สามารถ parse แถวใน CSV ได้");
        entries.push(entry);
    }

    assert_eq!(entries.len(), 3); // mock มี 3 รายการ
    assert_eq!(entries[0].category, "เงินเดือน");
    assert_eq!(entries[1].entry_type, EntryType::Expense);
    assert!(entries.iter().any(|e| e.amount > 1000.0));
}
