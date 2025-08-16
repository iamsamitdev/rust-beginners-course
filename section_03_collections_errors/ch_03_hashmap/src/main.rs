use std::collections::HashMap;

fn main() {
    
    // การสร้าง HashMap
    let mut scores = HashMap::new();
    // ทำการเพิ่มข้อมูลเข้า HashMap
    scores.insert("Team A", 95);
    scores.insert("Team B", 87);
    scores.insert("Team C", 92);

    // println!("{:?}", scores.get("Team A").unwrap()); // แบบนี้จะพบปัญหาเมื่อไม่มีคีย์ที่กำหนด

    match scores.get("Team A") {
        Some(score) => println!("Score Team A: {}", score),
        None => println!("Not found Team A")
    }

    // การวนลูป
    for (team, score) in &scores {
        println!("{}: {} Score", team, score);
    }

    // การอัพเดทข้อมูล
    scores.entry("Team D").or_insert(0); // ถ้าไม่มีจะใช้ค่า defautl เป็น 0
    scores.entry("Team A").and_modify(|score| *score += 5);

    println!("Score after update: {:?}", scores);

}
