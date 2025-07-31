// Ownership: การเป็นเจ้าของ
// fn main() {
//     let s1 = String::from("hello"); // s1 เป็นเจ้าของ String "hello"

//     let s2 = s1; // ความเป็นเจ้าของ "ย้าย" จาก s1 ไปยัง s2
//                  // ตอนนี้ s1 ไม่สามารถใช้งานได้แล้ว

//     println!("{}", s1); // จะเกิดข้อผิดพลาด: s1 ไม่มีความเป็นเจ้าของอีกต่อไป
//     println!("{}, world!", s2);

//     takes_ownership(s1); // s1 ถูกย้ายเข้าสู่ฟังก์ชัน takes_ownership
//                         // ตอนนี้ s1 ไม่สามารถใช้งานได้แล้ว

//     println!("{}", s1); // จะเกิดข้อผิดพลาด: s1 ไม่มีความเป็นเจ้าของอีกต่อไป

//     let x = 5;

//     makes_copy(x); // x ถูกส่งเข้าไปในฟังก์ชัน makes_copy
//                    // x ยังคงสามารถใช้งานได้หลังจากฟังก์ชันนี้จบ

//     println!("x is still accessible: {}", x); // x ยังคงสามารถใช้งานได้
// }


// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } // some_string หลุดออกจาก scope, หน่วยความจำถูกคืน

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// } // some_integer หลุดออกจาก scope, แต่ i32 เป็นประเภทที่สามารถคัดลอกได้

// Borrowing: การยืม
// Rust มีการยืม 2 แบบ
// 1. การยืมแบบไม่เปลี่ยนแปลง (Immutable Borrowing)
// fn main() {
//     let s = String::from("hello");

//     let len = calculate_length(&s); // ยืม s แบบ immutable

//     println!("The length of '{}' is {}.", s, len); // s ยังคงใช้งานได้
// }

// fn calculate_length(s: &String) -> usize { // s เป็น reference (การยืม) ของ String
//     s.len()
// } // s หลุดออกจาก scope, แต่ไม่ได้เป็นเจ้าของค่า จึงไม่มีอะไรถูกทิ้ง

// 2. การยืมแบบเปลี่ยนแปลง (Mutable Borrowing
fn main() {

    let mut s = String::from("hello");

    change(&mut s); // ยืม s แบบ mutable

    println!("{}", s); // s ถูกแก้ไขแล้ว
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
} // some_string หลุดออกจาก scope