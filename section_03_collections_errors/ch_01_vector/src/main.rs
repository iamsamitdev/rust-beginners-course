fn main() {
    // การสร้าง Vector
    let mut numbers: Vec<i32> = Vec::new();

    // ทำการ push ค่าใหม่เข้าไป
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("{:?}", numbers);

    // การสร้าง vec! macro
    let fruits = vec!["Apple", "Banana", "Orange", "300"];

    println!("{:?}", fruits);
    println!("{:?}", fruits[1]);

    let fruits2: Vec<String> = vec![
        "Apple".into(),
        "Banana".into(),
        "Orange".into(),
        "300".into(),
        500.to_string(),
    ];

    println!("{:?}", fruits2);

    // Convert the string at fruits2[4] to an integer and multiply by 2
    let value: i32 = fruits2[4].parse().unwrap();
    println!("fruits {:?}", value * 2);

    // การเข้าถึงข้อมูล
    match numbers.get(3) {
        Some(value) => println!("found 1: {}", value),
        None => println!("not found"),
    }

    // การวนลูป
    for fruit in &fruits {
        println!("found fruit: {}", fruit);
    }

    // การแก้ไขข้อมูล
    for number in &mut numbers {
        *number *= 2;
    }

    println!("result after multiplying by 2: {:?}", numbers);

    // ตัวอย่าง Array
    // Array([T; N])
    let arr: [i32; 5] = [10, 20, 30, 40, 50]; // ขนาดตายตัว
    println!("{:?}", arr);
}
