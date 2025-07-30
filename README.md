# Rust Programming for Beginners 🦀

## ### การรันแต่ละโปรเจ็กต์
```bash
# รันจากไดเรกทอรีหลัก (workspace)
cargo run -p ch_01_hello_world
cargo run -p ch_02_types_variables
cargo run -p ch_03_functions
cargo run -p ch_04_modules
cargo run -p exercise_module

# หรือเข้าไปในโฟลเดอร์แล้วรัน
cd section_01_basics/ch_01_hello_world
cargo run
```ย่างและ workshop สำหรับการเรียนรู้ภาษา Rust อย่างเป็นขั้นตอน

> คอร์สนี้ออกแบบมาสำหรับผู้เริ่มต้นที่ต้องการเรียนรู้ภาษา Rust จากพื้นฐานสู่การสร้างแอปพลิเคชันจริง

## 🎯 วัตถุประสงค์

- เรียนรู้ syntax และแนวคิดพื้นฐานของภาษา Rust
- เข้าใจ memory safety และ ownership system
- ฝึกใช้เครื่องมือ Rust ecosystem (Cargo, Rustfmt, Clippy)
- สร้างโปรเจกต์จริงเพื่อประยุกต์ความรู้

## 🏗️ ความต้องการของระบบ

- Rust 1.70+ ([ติดตั้งที่นี่](https://www.rust-lang.org/tools/install))
- VS Code หรือ text editor ที่รองรับ Rust
- Git (สำหรับ version control)

## 📚 โครงสร้างคอร์ส

### Section 01: พื้นฐาน Rust
- `section_01_basics/` – เริ่มต้นกับไวยากรณ์และแนวคิดพื้นฐานของ Rust
  - `ch_01_hello_world/` – Hello World และ Cargo basics
  - `ch_02_types_variables/` – ตัวแปร, ชนิดข้อมูล, immutability
  - `ch_03_functions/` – การสร้างและใช้งาน functions
  - `ch_04_modules/` – โมดูล, การแบ่งโค้ด และ visibility
  - `exercise_module/` – แบบฝึกหัดการสร้างและใช้งานโมดูล

## 🚀 วิธีใช้งาน

### การเริ่มต้น
```bash
# Clone repository
git clone https://github.com/iamsamitdev/rust-beginners-course
cd rust-beginners-course

# เริ่มจาก chapter แรก
cd section_01_basics/ch_01_hello_world
cargo run

# หรือรันจาก workspace root (จะรัน default package)
cargo run
```

### การรันแต่ละโปรเจกต์
```bash
# รันจากไดเรกทอรีหลัก (workspace)
cargo run -p ch_01_hello_world
cargo run -p ch_02_variables_types
cargo run -p ch_03_cargo_demo
cargo run -p ch_04_rustfmt_clippy

# หรือเข้าไปในโฟลเดอร์แล้วรัน
cd section_01_basics/ch_01_hello_world
cargo run
```

### เครื่องมือที่มีประโยชน์
```bash
# ตรวจสอบ syntax
cargo check

# จัดรูปแบบโค้ด
cargo fmt

# ตรวจสอบคุณภาพโค้ด
cargo clippy

# รัน tests
cargo test
```

## 📖 วิธีการเรียน

1. **เรียนตามลำดับ** - แต่ละ chapter สร้างต่อจากความรู้ของ chapter ก่อนหน้า
2. **ทำ hands-on** - รันโค้ดและแก้ไขเพื่อทดลอง
3. **อ่าน comments** - แต่ละไฟล์มี comments อธิบายการทำงาน
4. **ฝึกปฏิบัติ** - ลองเขียนโค้ดเพิ่มเติมด้วยตัวเอง
5. **ทำแบบฝึกหัด** - ใน `exercise_module/` มีแบบฝึกหัดให้ลองทำ

## 🎯 สิ่งที่จะได้เรียนรู้ใน Section 01

- **Chapter 1**: การสร้างโปรเจ็กต์ Rust แรก และพื้นฐาน Cargo
- **Chapter 2**: ตัวแปร, ชนิดข้อมูล, และ mutability
- **Chapter 3**: การสร้างและใช้งาน functions
- **Chapter 4**: การจัดการโค้ดด้วย modules และ visibility
- **Exercise Module**: การสร้างและใช้งาน library crate

## 🤝 การสนับสนุน

- หากพบปัญหาหรือมีคำถาม กรุณาสร้าง issue
- ยินดีรับ contributions และ pull requests
- แชร์ feedback เพื่อปรับปรุงคอร์ส

## 📝 License

MIT License - ใช้งานและแชร์ได้อย่างอิสระ

---

**Happy Coding with Rust! 🦀✨**
