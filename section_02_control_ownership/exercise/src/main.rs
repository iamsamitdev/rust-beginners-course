// ประกาศให้ Rust รู้ว่ามีโมดูลชื่อ calculator (จากไฟล์ calculator.rs)
mod calculator;

fn main() {
    // สร้างตัวแปรสำหรับสินค้าชิ้นที่ 1
    let apple_price: f64 = 15.50;
    let apple_quantity: u32 = 5;

    // สร้างตัวแปรสำหรับสินค้าชิ้นที่ 2
    let banana_price: f64 = 8.75;
    let banana_quantity: u32 = 8;

    // เรียกใช้ฟังก์ชันผ่านชื่อโมดูล `calculator::`
    let total_apple = calculator::calculate_item_total(apple_price, apple_quantity);
    let total_banana = calculator::calculate_item_total(banana_price, banana_quantity);

    // คำนวณยอดรวมสุดท้าย
    let grand_total = total_apple + total_banana;

    // แสดงผลลัพธ์
    println!("ราคารวมของแอปเปิ้ล: {:.2}", total_apple);
    println!("ราคารวมของกล้วย: {:.2}", total_banana);
    println!("ยอดรวมทั้งหมดที่ต้องชำระ: {:.2} บาท", grand_total);
}
