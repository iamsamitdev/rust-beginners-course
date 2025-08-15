fn main() {
    println!("message from main function");

    // การเรียกใช้ฟังก์ชัน (Calling functions)
    another_function();

    // เรียกใช้ฟังก์ชันที่ไม่มีการคืนค่า
    print_sum(5, 10);

    // เรียกใช้ฟังก์ชันที่คืนค่า
    let result = subtract(10, 3);
    println!("Result of subtraction: {}", result);

    // เรียกใช้ฟังก์ชันที่คืนค่าแบบแฝง
    let product = multiply(4, 5);
    println!("Product of multiplication: {}", product);

    // การประเมินค่าของบล็อก (Block Evaluations)
    // ใน Rust บล็อก `{}` สามารถใช้เพื่อประเมินค่าและคืนค่าจากบล็อกนั้นได้
    // ค่าของบล็อกจะถูกกำหนดให้กับตัวแปร `y`
    let y = {
        let x = 3;
        // นิพจน์สุดท้ายในบล็อก (ไม่มีเซมิโคลอน) จะเป็นค่าของบล็อกนั้น
        x + 1
    };
    println!("Value of y: {}", y);
}

// การนิยามฟังก์ชัน (Defining functions)
// ฟังก์ชันใน Rust จะถูกประกาศด้วยคีย์เวิร์ด `fn` และตั้งชื่อแบบ snake_case. ฟังก์ชัน

fn another_function() {
    println!("message from another function");
}

// พารามิเตอร์และอาร์กิวเมนต์ (Parameters and arguments)
// พารามิเตอร์ (Parameter) คือชื่อของข้อมูลที่คาดว่าจะรับเข้ามาในฟังก์ชันและต้องระบุชนิดข้อมูล. ส่วนอาร์กิวเมนต์ (Argument) คือค่าจริงที่เราส่งเข้าไปในฟังก์ชันเมื่อเรียกใช้
// `x` และ `y` คือ พารามิเตอร์ (parameters)
// หากมีหลายตัว ให้คั่นด้วยจุลภาค (,)

fn print_sum(x: i32, y: i32) {
    println!("Sum value: {}", x + y);
}

// การคืนค่าด้วย return (Explicit Return)
// เราสามารถใช้คีย์เวิร์ด `return` เพื่อคืนค่าจากฟังก์ชัน
// `-> i32` คือการระบุว่าฟังก์ชันนี้จะคืนค่าชนิด i32

fn subtract(x: i32, y: i32) -> i32 {
    // ใช้ `return` เพื่อส่งค่าคืนอย่างชัดเจน
    return x - y;
}

// การคืนค่าแบบแฝง (Implicit Returns)
// ฟังก์ชันสามารถคืนค่าของนิพจน์ (expression) สุดท้ายได้โดยอัตโนมัติ โดยบรรทัดนั้นจะต้องไม่มีเครื่องหมายเซมิโคลอน
fn multiply(a: i32, b: i32) -> i32 {
    // บรรทัดสุดท้ายที่ไม่มีเซมิโคลอน จะถูกใช้เป็นค่าที่ส่งคืนจากฟังก์ชัน
    let mul = a * b;
    mul
}
