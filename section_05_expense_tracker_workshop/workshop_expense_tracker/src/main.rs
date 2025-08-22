use workshop_expense_tracker::config::ensure_data_directory;
use workshop_expense_tracker::errors::ExpenseError;
use workshop_expense_tracker::{
    add::add_entry,
    summary:: {calculate_summary, list_entries},
};
use std::io::{self, Write};

fn main() -> Result<(), ExpenseError>{
    // Initialize data directory
    if let Err(e) = ensure_data_directory() {
        eprintln!("⚠️ ไม่สามารถสร้างโฟลเดอร์ข้อมูลได้: {e}")
    }

    loop {
        println!("\n📒 ระบบบันทึกรายรับรายจ่าย");
        println!("1. เพิ่มรายการใหม่");
        println!("2. แสดงรายการทั้งหมด");
        println!("3. คำนวณยอดรวม");
        println!("4. ออกจากโปรแกรม");
        print!("👉 กรุณาเลือกเมนู (1-4): ");
        
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                if let Err(e) = add_entry() {
                    eprintln!("❌ เกิดข้อผิดพลาด: {e}");
                }
            }
            "2" => {
                if let Err(e) = list_entries() {
                    eprintln!("❌ เกิดข้อผิดพลาด: {e}");
                }
            }
            "3" => {
                if let Err(e) = calculate_summary() {
                    eprintln!("❌ เกิดข้อผิดพลาด: {e}");
                }
            }
            "4" => {
                println!("👋 ขอบคุณที่ใช้งาน!");
                break;
            }
            _ => println!("⚠️ กรุณาเลือกเมนูให้ถูกต้อง (1-4)"),
        }
    }
    Ok(())
}
