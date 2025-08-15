pub fn borrowing_example() {
    let s1 = String::from("Hello Rustaceans!");

    // Borrowing: s1 ถูกยืมโดย s2
    let s2 = &s1;

    // s1 ยังสามารถใช้งานได้
    println!("{}", s1);
    println!("{}", s2);

    // let len = calculate_length(&s1);
    let len = calculate_length(s2);

    println!("Length: {}", len);

    // การยืมแบบสามารถแก้ไขค่าได้
    let mut s = String::from("Hello");
    change_string(&mut s);
    println!("Modified String: {}", s);

    // ไม่สามารถมีทั้ง mutable และ immutable references พร้อมกันได้
    let mut r1 = String::from("Rust is Awesome!");

    // หลาย imutable referrence - OK
    let r2 = &r1;
    let r3 = &r1;

    // let r4 = &mut r1; // error: cannot borrow `r1` as mutable because it is also borrowed as immutable

    println!("r2: {}, r3: {}", r2, r3);

    // r2, r3 ใช้งานครั้งสุดท้ายแล้ว (NLL - Non-Lexical Lifetimes)

    // ตอนนี้ r1 สามารถยืมแบบแก้ไขได้
    let r4 = &mut r1;
    // let r5 = &mut r1; // จะเกิดข้อผิดพลาดถ้า r4 ยังใช้งานอยู่
    r4.push_str(" Programming");
    println!("r4: {}", r4);

    // r4 ใช้งานครั้งสุดท้ายแล้ว
    let r5 = &mut r1; // ตอนนี้ r1 สามารถยืมแบบแก้ไขได้อีกครั้ง
    r5.push_str(" is fun!");
    println!("r4: {}", r5);
} // s1 และ s2 จะถูก drop ที่นี่

// ฟังก์ชันแบบไม่สามารถแก้ไขค่าได้
fn calculate_length(s: &String) -> i32 {
    // พยายามลองแก้ไข
    // s.push_str("Cool !");
    s.len() as i32 // อ่านค่าได้ แต่แก้ไขไม่ได้
}

// ฟังก์ชันแบบสามารถแก้ไขค่าได้
fn change_string(s: &mut String) {
    s.push_str(", Rust!");
}
