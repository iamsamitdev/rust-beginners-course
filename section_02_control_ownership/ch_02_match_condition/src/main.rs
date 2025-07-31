fn main() {

    let status_code = 500;

    match status_code {
        200 => println!("OK"),
        404 => {
            println!("Error: Page Not Found.");
            println!("Please check the URL and try again.");
            report_error_to_system(404);
        },
        500 => {
            println!("Error: Internal Server Error.");
            println!("Our engineers have been notified.");
            report_error_to_system(500);
        },
        _ => println!("Unknown Status Code"),
    }

    println!("---------------------------------");

    let http_method = "POST";

    match http_method {
        "GET" => println!("Requesting data..."),
        "POST" | "PUT" => println!("Sending data..."), // จับคู่ทั้ง POST และ PUT
        "DELETE" => println!("Deleting data..."),
        _ => println!("Unknown method"),
    }

    println!("---------------------------------");

    let day = 1;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        _ => println!("Other day"), // ทำงานเมื่อไม่ตรงกับเคสใดๆ ก่อนหน้า
    }
    
    println!("--------------------------");

    let number = 22;

    match number {
        1..=10 => println!("Number is between 1 and 10"),
        11..=20 => println!("Number is between 11 and 20"),
        21..=30 => println!("Number is between 21 and 30"),
        _ => println!("Number is outside the range of 1 to 30"), // ทำงานเมื่อไม่ตรงกับเคสใดๆ ก่อนหน้า
    }

    println!("--------------------------");

    // Some คือการใช้ Option เพื่อจัดการกับค่าที่อาจจะมีหรือไม่มี
    // ในที่นี้เราจะใช้ match เพื่อจัดการกับค่าที่เป็น Option
    // โดยจะมีเคสสำหรับค่าที่เป็น Some และ None
    // จะตก Some เมื่อมีค่า
    // จะตก None เมื่อไม่มีค่า

    let value = Some(42);
    // ตัวอย่างตกไปที่ None
    // let value: Option<i32> = None;

    match value {
        Some(v) => println!("Value is: {}", v), // จับคู่กับ Some
        None => println!("No value provided"), // จับคู่กับ None
    }

}

// ฟังก์ชันสมมติสำหรับรายงานข้อผิดพลาด
fn report_error_to_system(code: u32) {
    println!("-> Reporting error code {} to monitoring system...", code);
}