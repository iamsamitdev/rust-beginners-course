# Rust Programming for Beginners 🦀

## แหล่งรวมตัวอย่างและ workshop สำหรับการเรียนรู้ภาษา Rust อย่างเป็นขั้นตอน

> คอร์สนี้ออกแบบมาสำหรับผู้เริ่มต้นที่ต้องการเรียนรู้ภาษา Rust จากพื้นฐานสู่การสร้างแอปพลิเคชันจริง

## 🎯 วัตถุประสงค์

- ฝึกใช้เครื่องมือ Rust ecosystem (Cargo, Rustfmt, Clippy)

## 🏗️ ความต้องการของระบบ

- Rust 1.70+ ([ติดตั้งที่นี่](https://www.rust-lang.org/tools/install))
- VS Code หรือ text editor ที่รองรับ Rust
- Git (สำหรับ version control)

## 🚀 วิธีใช้งาน

### การเริ่มต้น
```bash
# Clone repository
git clone https://github.com/iamsamitdev/rust-beginners-course
```

### การรันโปรเจกต์
```bash
# รันจากไดเรกทอรีหลัก (workspace)
cargo run
cargo run --bin rust-beginners-course
cargo run --bin client
cargo run --bin server
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
1. **อ่าน README** - แต่ละโฟลเดอร์มี README อธิบายจุดประสงค์
2. **ทำ hands-on** - รันโค้ดและแก้ไขเพื่อทดลอง
3. **ฝึกปฏิบัติ** - ลองเขียนโค้ดเพิ่มเติมด้วยตัวเอง

## 🤝 การสนับสนุน

- หากพบปัญหาหรือมีคำถาม กรุณาสร้าง issue
- ยินดีรับ contributions และ pull requests
- แชร์ feedback เพื่อปรับปรุงคอร์ส

## 📝 License

MIT License - ใช้งานและแชร์ได้อย่างอิสระ

---

**Happy Coding with Rust! 🦀✨**
