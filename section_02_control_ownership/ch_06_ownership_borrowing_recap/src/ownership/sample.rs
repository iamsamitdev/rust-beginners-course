pub fn ownership_example() {
    // s1 เป็น owner ของ String
    let s1 = String::from("Hello Rustaceans!");

    println!("{}", s1);

    // Move: ownership ถูกย้ายจาก s1 ไปยัง s2
    let s2 = s1;
    println!("{}", s2);

    // Error! s1 ไม่สามารถใช้งานได้แล้ว
    // println!("{}", s1);

    takes_ownership(s2);

    // จะเกิดข้อผิดพลาด: s2 ไม่มีความเป็นเจ้าของอีกต่อไป
    // println!("{}", s2); // จะ compile error
} // s2 จะถูก drop ที่นี่

pub fn takes_ownership(str: String) {
    println!("{}", str);
} // str หลุดออกจาก scope, หน่วยความจำถูกคืน
