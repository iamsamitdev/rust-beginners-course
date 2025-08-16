// ประกาศ trait
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement trait สำหรับ struct
#[allow(dead_code)]
struct Article {
    headline: String,
    content: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ฟังก์ชันนี้รับ "อะไรก็ได้" ที่ implement trait `Summary`
fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

fn main() {

    let article = Article {
        headline: String::from("Rust is amazing!"),
        author: String::from("Jane Doe"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("johndoe"),
        content: String::from("I love Rust!"),
    };

    // เรียกใช้ default implementation
    println!("Default Summary: {}", article.default_summary());

    // เราสามารถส่งทั้ง article และ tweet เข้าไปในฟังก์ชันเดียวกันได้!
    notify(&article);
    notify(&tweet);
}
