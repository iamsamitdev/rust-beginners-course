# Rust Programming for Beginners 🦀

## แหล่งรวมตัวอย่างและ workshop สำหรับการเรียนรู้ภาษา Rust อย่างเป็นขั้นตอน

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

### Section 02: Control Flow และ Ownership
- `section_02_control_ownership/` – การควบคุมการทำงานและระบบความเป็นเจ้าของ
  - `ch_01_ifelse_condition/` – เงื่อนไข if/else และการตัดสินใจ
  - `ch_02_match_condition/` – Pattern matching และ match expressions
  - `ch_03_loop/` – การวนซ้ำ (for, while, loop)
  - `ch_04_stack_heap/` – หน่วยความจำ Stack และ Heap
  - `ch_05_ownership_borrowing/` – Ownership และ Borrowing system
  - `ch_06_ownership_borrowing_recap/` – ทบทวน Ownership และ Borrowing
  - `exercise/` – แบบฝึกหัด Control Flow และ Ownership

### Section 03: Collections และ Error Handling
 - `section_03_collections_errors/` – โครงสร้างข้อมูลและการจัดการข้อผิดพลาด
   - `ch_01_vector/` – Vector และการจัดการ collections
   - `ch_02_string/` – String และการจัดการข้อความ
   - `ch_03_hashmap/` – HashMap และการจัดการ key-value
   - `ch_04_option_result/` – Option, Result และ error handling

### Section 04: Structs, Enum, Method, Trait
- `section_04_structs_traits_project/` – การสร้าง custom types และการใช้งาน trait
  - `ch_01_struct/` – การสร้างและใช้งาน struct
  - `ch_02_enum/` – การสร้างและใช้งาน enum
  - `ch_03_method/` – การสร้าง method ให้กับ struct/enum
  - `ch_04_trait/` – การสร้างและใช้งาน trait

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

### การรันแต่ละโปรเจ็กต์
```bash
# Section 01: พื้นฐาน Rust
cargo run -p ch_01_hello_world
cargo run -p ch_02_types_variables
cargo run -p ch_03_functions
cargo run -p ch_04_modules
cargo run -p exercise_module

# Section 02: Control Flow และ Ownership
cargo run -p ch_01_ifelse_condition
cargo run -p ch_02_match_condition
cargo run -p ch_03_loop
cargo run -p ch_04_stack_heap
cargo run -p ch_05_ownership_borrowing
cargo run -p ch_06_ownership_borrowing_recap
cargo run -p exercise

# Section 03: Collections และ Error Handling
cargo run -p ch_01_vector
cargo run -p ch_02_string
cargo run -p ch_03_hashmap
cargo run -p ch_04_option_result

# Section 04: Structs, Enum, Method, Trait
cargo run -p ch_01_struct
cargo run -p ch_02_enum
cargo run -p ch_03_method
cargo run -p ch_04_trait

# หรือเข้าไปในโฟลเดอร์แล้วรัน
cd section_01_basics/ch_01_hello_world
cargo run
```
### การรันแบบ watch
```bash
# ติดตั้ง cargo watch (สำหรับ auto-reload)
cargo install cargo-watch

# Section 01: พื้นฐาน Rust
cargo watch -q -c -x "run -p ch_01_hello_world"
cargo watch -q -c -x "run -p ch_02_types_variables"
cargo watch -q -c -x "run -p ch_03_functions"
cargo watch -q -c -x "run -p ch_04_modules"
cargo watch -q -c -x "run -p exercise_module"

# Section 02: Control Flow และ Ownership
cargo watch -q -c -x "run -p ch_01_ifelse_condition"
cargo watch -q -c -x "run -p ch_02_match_condition"
cargo watch -q -c -x "run -p ch_03_loop"
cargo watch -q -c -x "run -p ch_04_stack_heap"
cargo watch -q -c -x "run -p ch_05_ownership_borrowing"
cargo watch -q -c -x "run -p ch_06_ownership_borrowing_recap"
cargo watch -q -c -x "run -p exercise"

# Section 03: Collections และ Error Handling
cargo watch -q -c -x "run -p ch_01_vector"
cargo watch -q -c -x "run -p ch_02_string"
cargo watch -q -c -x "run -p ch_03_hashmap"
cargo watch -q -c -x "run -p ch_04_option_result"

# Section 04: Structs, Enum, Method, Trait
cargo watch -q -c -x "run -p ch_01_struct"
cargo watch -q -c -x "run -p ch_02_enum"
cargo watch -q -c -x "run -p ch_03_method"
cargo watch -q -c -x "run -p ch_04_trait"

# หรือ watch ในโฟลเดอร์ของ package
cd section_01_basics/ch_01_hello_world
cargo watch -q -c -x run
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

## 🎯 สิ่งที่จะได้เรียนรู้ใน Section 02

- **Chapter 1**: เงื่อนไข if/else และการตัดสินใจในโปรแกรม
- **Chapter 2**: Pattern matching และการใช้ match expressions
- **Chapter 3**: การวนซ้ำด้วย for, while, และ loop
- **Chapter 4**: ความแตกต่างระหว่าง Stack และ Heap memory
- **Chapter 5**: Ownership system และ Borrowing ใน Rust
- **Chapter 6**: ทบทวนและฝึกปฏิบัติ Ownership และ Borrowing
- **Exercise**: แบบฝึกหัดการใช้ Control Flow และ Ownership

## 🎯 สิ่งที่จะได้เรียนรู้ใน Section 03

- **Chapter 1**: Vector และการจัดการ collections
- **Chapter 2**: String และการจัดการข้อความใน Rust
- **Chapter 3**: HashMap และการจัดการ key-value
- **Chapter 4**: Option, Result และ error handling

## 🎯 สิ่งที่จะได้เรียนรู้ใน Section 04

- **Chapter 1**: การสร้างและใช้งาน struct
- **Chapter 2**: การสร้างและใช้งาน enum
- **Chapter 3**: การสร้าง method ให้กับ struct/enum
- **Chapter 4**: การสร้างและใช้งาน trait

## 🤝 การสนับสนุน

- หากพบปัญหาหรือมีคำถาม กรุณาสร้าง issue
- ยินดีรับ contributions และ pull requests
- แชร์ feedback เพื่อปรับปรุงคอร์ส

## 📝 License

MIT License - ใช้งานและแชร์ได้อย่างอิสระ

---

**Happy Coding with Rust! 🦀✨**
