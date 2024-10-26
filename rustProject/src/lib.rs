use std::io;

pub struct User {
    pub uid: String,
    pub phone: String,
    pub email: String,
    pub age: u8,
}

impl User {
    // 初始化 User 的構造方法
    pub fn new_user(uid: String, phone: String, email: String, age: u8) -> Self {
        User {
            uid,
            phone,
            email,
            age,
        }
    }

    // 驗證身份證字號
    pub fn uid(&self) -> Option<String> {
        match self.uid.len() {
            10 => match self.uid.as_bytes()[0] {
                b'A'..=b'Z' => match self.uid.as_bytes()[1] {
                    b'1' | b'2' => {
                        match self.uid[2..].as_bytes().iter().all(|&c| c.is_ascii_digit()) {
                            true => Some("身份證字號符合".to_string()),
                            false => None,
                        }
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    // 驗證手機號碼
    pub fn phone(&self) -> Option<String> {
        match self.phone.len() {
            10 => match &self.phone[0..2] {
                "09" => match self.phone[2..]
                    .as_bytes()
                    .iter()
                    .all(|&c| c.is_ascii_digit())
                {
                    true => Some("手機號碼符合".to_string()),
                    false => None,
                },
                _ => None,
            },
            _ => None,
        }
    }

    // 驗證電子郵件
    pub fn email(&self) -> Option<String> {
        match (
            self.email.contains('@'),
            &self.email[self.email.len() - 4..],
        ) {
            (true, ".com") => Some("電子郵件符合".to_string()),
            _ => None,
        }
    }

    // 年齡分類檢查
    pub fn age(&self) -> String {
        match self.age {
            0..=10 => "你應該玩".to_string(),
            11..=20 => "你應該學習".to_string(),
            21..=30 => "你應該賺錢".to_string(),
            31..=40 => "你應該財富自由".to_string(),
            _ => "你應該環遊世界".to_string(),
        }
    }

    // 驗證所有資訊
    pub fn validate(&self) {
        match self.uid() {
            Some(msg) => println!("{}", msg),
            None => println!("身份證字號錯誤！！"),
        }

        match self.phone() {
            Some(msg) => println!("{}", msg),
            None => println!("手機號碼錯誤！！"),
        }

        match self.email() {
            Some(msg) => println!("{}", msg),
            None => println!("電子郵件錯誤！！"),
        }

        println!("{}", self.age());
    }
}

// 輔助函數，讀取輸入
pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("輸入錯誤");
    input.trim().to_string()
}

pub fn read_input_u8(prompt: &str) -> u8 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("輸入錯誤");
    input.trim().parse().expect("無效的年齡輸入")
}

pub fn read_input_usize(prompt: &str) -> usize {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("輸入錯誤");
    input.trim().parse().expect("無效的數字輸入")
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{} {} 著 ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
