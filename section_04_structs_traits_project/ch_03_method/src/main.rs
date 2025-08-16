#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method ที่ใช้ &self (immutable reference)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method ที่ใช้ &mut self (mutable reference)
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Associated function (ไม่ใช้ self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[allow(dead_code)]
fn main_demo() {
    
    // ต้องเป็น mut เพื่อให้สามารถเรียก double_size() ได้
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // เรียกใช้ area() ซึ่งใช้ &self (ยืมมาอ่าน)
    println!("Area of react1 is {}", rect1.area());

    // เรียกใช้ double_size() ซึ่งใช้ &mut self (ยืมมาแก้ไข)
    rect1.double_size();
    println!("New width: {}, New height: {}", rect1.width, rect1.height);

    // เรียกใช้ associated function `square` เพื่อสร้าง instance ใหม่
    let sq = Rectangle::square(25);
    println!("Created a new square: {:?}", sq);
}


impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main(){
    // ตัวอย่างการใช้ can_hold
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle 1: {:?}", rect1);

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // ตัวอย่างการใช้ perimeter
    println!("Perimeter of rect1 is {}", rect1.perimeter());
    println!("Perimeter of rect2 is {}", rect2.perimeter());
}