fn main() {

    // การแสดงผลข้อความพื้นฐานขึ้นบรรทัดใหม่
    println!("Hello, world! 888");

    // การแสดงผลข้อความแบบไม่ขึ้นบรรทัดใหม่
    print!("Hello, ");
    print!("world!\n\n");
    print!("\taaa");

    println!("-- Basic Output --");

    // การแทรกค่าลงในสตริง (String Interpolation)
    println!("Welcome {} {}", "Samit", "Koyom");

    // {0} จะอ้างอิงถึง "Alice" (อาร์กิวเมนต์ตัวแรก)
    // {1} จะอ้างอิงถึง "Bob" (อาร์กิวเมนต์ตัวที่สอง)
    println!("{0} is a friend of {1}. And {1} is also a friend of {0}.", "Alice", "Bob");

    println!("-- Named Arguments --");
    println!("My name is {name}", name = "Samit");
    println!("I am {old} years old. I live in {city}.", old = 25, city = "Bangkok");

    println!("-- Formatting Numbers --");
    // การจัดรูปแบบตัวเลข
    println!("Pi is approximately {:.2}", 3.14159);
    println!("Pi is approximately {value:.2}", value = 3.14159);
    println!("Number with padding: {:05}", 32); // แสดงเป็น 00032
    println!("Binary: {:b}", 32);  // แสดงเป็นเลขฐาน 2
    println!("Hexadecimal: {:x}", 32);  // แสดงเป็นเลขฐาน 16
    println!("Octal: {:o}", 32);  // แสดงเป็นเลขฐาน 8

    println!("-- Formatting Text --");

    // การจัดรูปแบบข้อความ
    println!("Left aligned: '{:<10}'", "Hello");   // จัดชิดซ้าย
    println!("Right aligned: '{:>10}'", "Hello");  // จัดชิดขวา
    println!("Center aligned: '{:^10}'", "Hello"); // จัดกึ่งกลาง

    // การใช้ escape characters
    println!("-- Escape Characters --");
    println!("Quote: \"Hello World\"");
    println!("Backslash: \\");
    println!("New line:\nSecond line");
    println!("Tab:\tTabbed text");

}
