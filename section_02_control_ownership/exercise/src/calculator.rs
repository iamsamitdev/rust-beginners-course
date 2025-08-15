// ใส่ `pub` เพื่อทำให้ฟังก์ชันนี้เป็น public และสามารถเข้าถึงได้จากไฟล์อื่นๆ
pub fn calculate_item_total(price: f64, quantity: u32) -> f64 {
    // แปลง quantity (u32) ให้เป็น f64 ก่อนนำไปคำนวณ
    price * (quantity as f64)
}
