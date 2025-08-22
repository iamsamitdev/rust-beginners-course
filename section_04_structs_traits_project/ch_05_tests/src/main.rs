// ฟังก์ชันบวกเลข
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// เขียน Unit Test สำหรับฟังก์ชัน add()
#[cfg(test)]
mod tests {
    // นำเข้่าฟังก์ชัน add() จากภายนอกเข้ามา
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        // ตรวจสอบว่า add(2, 3) ได้ผลลัพธ์เท่ากับ 5 หรือไม่
        assert_eq!(add(2, 3), 8);
    }

    #[test]
    fn test_add_negative_numbers() {
        // ตรวจสอบว่า add(-1, -5) ได้ผลลัพธ์เท่ากับ -6 หรือไม่
        assert_eq!(add(-1, -5), -6);
    }
}

fn main() {
    // เรียกใช้ฟังก์ชัน add
    let result = add(10, 20);
    println!("The result of adding 10 and 20 is: {}", result);
}
