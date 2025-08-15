pub fn lifetime_example() {
    let string1 = String::from("Programming");
    let string2 = String::from("Rust");

    let result = longest(&string1, &string2);

    println!("The longest string is: {}", result);

    // Case 1
    // เก็บอ้างอิงไว้ให้อยู่นานกว่าเจ้าของ (owner) จริง
    // เริ่มขอบเขตของ "ห้องใหญ่"
    let s1 = String::from("Programming"); // s1 มีชีวิตอยู่ถึงจบโปรแกรม
    let r; // r ก็มีชีวิตอยู่ถึงจบโปรแกรม
    let s2 = String::from("Rust"); // s2 เกิดขึ้นที่นี่

    {
        // ---- เริ่มขอบเขต "ห้องเล็ก" ----
        // let s2 = String::from("Rust"); // s2 เกิดขึ้นที่นี่
        // r พยายามยืมค่าที่อายุขัยถูกผูกกับตัวที่สั้นที่สุด (คือ s2)
        r = longest(&s1, &s2); // error: does not live long enough
    } // ---- จบขอบเขต "ห้องเล็ก", s2 ถูกทำลายทิ้งทันที ----

    // ปัญหา! r พยายามจะปริ้นท์ค่าที่ยืมมา
    // แต่เจ้าของค่า (s2) ไม่อยู่แล้ว!
    println!("The longest string is: {}", r);

    // Case 2
    // ส่งอ้างอิงของ “temporary” (ค่าชั่วคราว) เข้าไป แล้วเอาผลลัพธ์มาใช้ต่อ
    let str2 = String::from("Rust");
    // let str3 = String::from("Programming").as_str();
    // let result = longest(&String::from("Programming").as_str(), &str2); // ❌ อาจอ้างอิงชี้ไปยัง temporary ที่หมดอายุแล้ว
    // let result = longest(&str3, &str2); // ❌ อาจอ้างอิงชี้ไปยัง temporary ที่หมดอายุแล้ว
    // println!("{}", result);      // ❌ อาจอ้างอิงชี้ไปยัง temporary ที่หมดอายุแล้ว

    bad_longest(&String::from("Programming"), &str2); // ❌ อาจอ้างอิงชี้ไปยัง temporary ที่หมดอายุแล้ว
}

// ใน Rust เพื่อให้แน่ใจว่าอ้างอิงที่ส่งกลับมีอายุการใช้งานที่ถูกต้อง
// ฟังก์ชันนี้รับพารามิเตอร์สองตัวที่เป็นอ้างอิงแบบไม่เปลี่ยนแปลง
// และส่งกลับอ้างอิงที่มีอายุการใช้งานเดียวกัน
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // เปรียบเทียบความยาวของ x และ y
    if x.len() > y.len() { x } else { y }
}

// ❌ ตัวอย่างฟังก์ชันที่ผิด
// pub fn bad_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
pub fn bad_longest<'a>(x: &'a str, y: &'a str) -> String {
    // let local = String::from("temporary");
    // if x.len() > y.len() { x } else { local.as_str() } // คืนอ้างอิง local
    if x.len() > y.len() {
        x.to_string()
    } else {
        String::from("temporary")
    } // คืนอ้างอิง local
} // local ถูก drop ที่นี่ ──── dangling!
