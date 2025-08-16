// การประกาศ Struct User
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Struct สำหรับข้อมูลที่ไม่ต้องการชื่อ field
struct Color(i32, i32, f64);
struct Point(f64, i32, bool);

// ตัวอย่างการใช้งานจริง: ระบบจัดการ product
struct Product {
    id: u32,
    name: String,
    price: f64,
    category: String,
    in_stock: bool,
}

impl Product {
    // Constructor function
    fn new(id: u32, name: String, price: f64, category: String) -> Self {
        Product {
            id,
            name,
            price,
            category,
            in_stock: true,
        }
    }
}

fn main() {

    // สร้าง instance ของ Product
    let product1 = Product::new(1, String::from("Laptop"), 999.99, String::from("Electronics"));

    // แสดงข้อมูลของ product1
    println!("Product ID: {}", product1.id);
    println!("Product Name: {}", product1.name);
    println!("Product Price: ${:.2}", product1.price);
    println!("Product Category: {}", product1.category);
    println!("In Stock: {}", product1.in_stock);

    // การใช้งาน method อื่น ๆ
    // เช่น การเปลี่ยนแปลงข้อมูล
    let mut product2 = Product::new(2, String::from("Smartphone"), 499.99, String::from("Electronics"));
    product2.price = 399.99;
    println!("Updated Product 2 Price: ${:.2}", product2.price);

    // การใช้งาน method ที่เกี่ยวข้องกับ Product
    // เช่น การตรวจสอบสถานะสินค้า
    if product2.in_stock {
        println!("Product 2 is in stock.");
    } else {
        println!("Product 2 is out of stock.");
    } 

    
    // สร้าง Instance ของ User
    let mut user1 = User {
        username: String::from("john_doe"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
        active: true
    };

    println!("-- Basic Structs --");

    // การเข้าถึงข้อมูลใน User
    println!("Username {}", user1.username);

    // การแก้ไขข้อมูล (ต้องเป็น mutable)
    user1.sign_in_count += 1;

    println!("Updated sign in count: {}", user1.sign_in_count);

    println!("-- Structs (Tuple Structs) --");

    let black = Color(0, 0, 0.0);
    let origin = Point(0.0, 0, true);

    // เข้าถึงข้อมูลผ่าน index
    println!("Red: {}", black.0);
    println!("Green: {}", black.1);
    println!("Blue: {}", black.2);
    println!("Orign: {}, {}, {}", origin.0, origin.1, origin.2);

}
