fn main() {
    // ตัวอย่างการใช้ if-else เพื่อเช็คเงื่อนไข
    let number: i32 = 8; // 8, 9, 10

    if number % 4 == 0 {
        println!("ตัวเลข {} เป็นเลขที่หารด้วย 4 ลงตัว", number);
    } else if number % 3 == 0 {
        println!("ตัวเลข {} เป็นเลขที่หารด้วย 3 ลงตัว", number);
    } else {
        println!("ตัวเลข {} ไม่สามารถหารด้วย 4 หรือ 3 ลงตัว", number);
    }

    println!("--------------------");

    // การใช้ if ในการกำหนดค่าให้ตัวแปร
    // คล้ายกับการใช้ ternary operator ในภาษาอื่นๆ
    // เช่น ใน JavaScript: let value = condition ? 5 : 6;
    // ใน Rust ใช้ if-else แทน
    // และต้องมีการระบุทั้งสองกรณี (if และ else)
    // หากไม่ระบุ else จะเกิดข้อผิดพลาด
    // เนื่องจาก Rust ต้องการให้แน่ใจว่าทุกเส้นทางของโค้ดจะส่งค่ากลับมา
    let condition = false;
    let value = if condition { 5 } else { 6 };
    println!("ค่าที่ได้จาก if-else คือ: {}", value);
}
