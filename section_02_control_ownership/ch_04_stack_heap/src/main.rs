// ตัวอย่างโค้ด: การทำงานบน Stack
// fn main() {

//     let x = 5; // i32 มีขนาดคงที่, เก็บไว้ใน Stack
//     let y = 10; // i32 มีขนาดคงที่, เก็บไว้ใน Stack

//     // การเรียกใช้ฟังก์ชัน add
//     let sum = add(x, y); // ค่า x และ y ถูก "copy" ไปให้ฟังก์ชัน add

//     println!("Some of {} and {} is {}", x, y, sum);

// }  // ตัวแปร sum, y, x ถูกลบออกจาก Stack ตามลำดับ

// fn add(a: i32, b: i32) -> i32 {
//     let result = a + b;
//     result
// } // ตัวแปร result, b, a ถูกลบออกจาก Stack

// ตัวอย่างโค้ด: การทำงานบน Heap

// fn main() {
//     let s1 = String::from("Hello"); // String ถูกเก็บไว้ใน Heap
//     let s2 = String::from("World"); // String ถูกเก็บไว้ใน Heap

//     println!("The original strings are: {}, {}", s1, s2); // s1 และ s2 ยังคงอยู่บน Stack

//     // Calling the concatenate function
//     let result = concatenate(s1, s2); // ค่า s1 และ s2 ถูก "move" ไปให้ฟังก์ชัน concatenate

//     println!("The concatenated string is: {}", result); // result เป็น String ใหม่ที่ถูกสร้างขึ้น

//     // s1 และ s2 ถูกย้ายไปยังฟังก์ชัน concatenate แล้ว ดังนั้นไม่สามารถใช้งานได้อีก
//     // println!("s1: {}, s2: {}", s1, s2); // จะเกิดข้อผิดพลาดที่นี่
//     // แต่ข้อมูลใน Heap ยังคงอยู่จนกว่าจะไม่มีตัวแปรใด
//     // อ้างอิงถึงมันอีกต่อไป
// } // ตัวแปร result ถูกลบออกจาก Stack แต่ข้อมูลใน Heap ยังคงอยู่

// #[allow(unused)]
// fn concatenate(a: String, b: String) -> String {
//     let result = format!("{} {}", a, b); // a, b, และ result อยู่บน Stack ของฟังก์ชัน concatenate
//     result // คืนค่า result ซึ่งเป็น String ใหม่
// } // ตัวแปร result, b, a ถูกลบออกจาก Stack

// ตัวอย่างโค้ด: การทำงานบน Heap เพิ่มเติม
fn main() {
    // สร้างข้อมูลบน Heap
    // b1 คือ Box<i32> ซึ่งเป็น pointer และถูกเก็บไว้บน Stack
    // ส่วนค่า `5` จริงๆ ถูกเก็บไว้ใน Heap
    let b1 = Box::new(5);

    println!("b1 = {}", b1);

    // ลองสร้างข้อมูลที่ซับซ้อนขึ้นมา
    let game = create_game();

    // ตัวแปร game (struct Game) อยู่บน Stack
    // แต่ field ที่ชื่อ `name` (String) มีข้อมูลจริงๆ ("Minecraft") อยู่บน Heap
    println!("Playing {} with score {}", game.name, game.score);
} // เมื่อจบ main, game จะถูก drop -> String ภายในจะคืนหน่วยความจำบน Heap
// b1 ก็จะถูก drop -> คืนหน่วยความจำของเลข 5 บน Heap

struct Game {
    name: String, // String จัดการข้อมูลบน Heap
    score: u32,   // u32 อยู่บน Stack (เป็นส่วนหนึ่งของ struct)
}

fn create_game() -> Game {
    let game_name = String::from("Minecraft"); // สร้าง String บน Heap
    Game {
        name: game_name,
        score: 100,
    }
}
