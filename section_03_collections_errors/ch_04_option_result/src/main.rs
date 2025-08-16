use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[allow(dead_code)]
fn find_student_score(name: &str) -> Option<i32> {

    let mut scores = HashMap::new();
    scores.insert("Anan", 85);
    scores.insert("Suda", 92);
    scores.insert("Wichai", 78);

    scores.get(name).copied() // copied คือ การสร้างสำเนาของค่า จะทำให้ค่าที่ได้ไม่ถูกยึดครองโดย reference
}

#[allow(dead_code)]
fn main_option() {
    
    let student_name = "Anan";

    // การใช้ Mach เพื่อ handle Option
    match find_student_score(student_name) {
        Some(score) => println!("{} Score: {}", student_name, score),
        None => println!("Not found {}", student_name)
    }

    // การใช้ if let
    if let Some(score) = find_student_score("Suda") {
        println!("Suda's score: {}", score);
    } else {
        println!("Suda's score not found");
    }

    // การใช้ unwrap_or
    // unwrap_or จะคืนค่าที่ระบุถ้าไม่พบค่า
    let score = find_student_score("Antony").unwrap_or(0);
    println!("Antony's score: {:?}", score);

    // การใช้ map
    // map จะใช้เมื่อเราต้องการเปลี่ยนแปลงค่าภายใน Some
    // map จะรับ closure ที่มีค่าเป็น Some และคืนค่าเป็น Some ใหม่
    // ในกรณีนี้เราต้องการคูณคะแนนด้วย 2
    let doubled_score = find_student_score("Wichai").map(|s| s * 2);
    println!("Wichai's doubled score: {:?}", doubled_score.unwrap_or(0));
    // :? จะทำให้เราสามารถจัดการกับค่าที่เป็น None ได้
}


// ฟังก์ชันสำหรับอ่านไฟล์
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // ? operator
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// ฟังก์ชันในการหารเลข
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}


fn main() {

    // การจัดการ Result ด้วย match
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error)
    }

    // การใช้ unwrap_or_else
    let result = divide(10.0, 0.0).unwrap_or_else(|err| {
        println!("Error: {}", err);
        0.0 // ค่าเริ่มต้น
    });
    println!("Result: {}", result);

    // การใช้ ? operator ใน function
    match read_file_content("test.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error: {}", error)       
    }
}
