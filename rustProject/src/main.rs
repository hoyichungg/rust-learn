// use rand::Rng;
// use std::io;

// #[derive(Debug)]
// struct GetNumberStruct<T> {
//     nums: Vec<T>,
//     left: T,
//     right: T,
// }

// impl<T: PartialOrd + Copy + rand::distributions::uniform::SampleUniform> GetNumberStruct<T> {
//     fn construct(nums: Vec<T>, left: T, right: T) -> Self {
//         Self { nums, left, right }
//     }

//     fn lotton_number(&mut self) {
//         let mut rng = rand::thread_rng();
//         let mut flag;
//         let mut count = 0;

//         while count < self.nums.len() {
//             let temp = rng.gen_range(self.left..=self.right);
//             flag = false;

//             for index2 in 0..count {
//                 if temp == self.nums[index2] {
//                     flag = true;
//                     break;
//                 }
//             }

//             if !flag {
//                 self.nums[count] = temp;
//                 count += 1;
//             }
//         }
//     }
// }

// fn main() {
//     let mut count = String::new();
//     let mut left = String::new();
//     let mut right = String::new();

//     println!("Please enter count:");
//     io::stdin().read_line(&mut count).expect("Error reading input");

//     println!("Please enter left bound:");
//     io::stdin().read_line(&mut left).expect("Error reading input");

//     println!("Please enter right bound:");
//     io::stdin().read_line(&mut right).expect("Error reading input");

//     let n: usize = count.trim().parse().expect("Error parsing count");
//     let l: f64 = left.trim().parse().expect("Error parsing left bound");
//     let r: f64 = right.trim().parse().expect("Error parsing right bound");

//     let mut get_number = GetNumberStruct::construct(vec![-1.0; n], l, r);

//     get_number.lotton_number();
//     dbg!(&get_number);
// }
// use rustProject::Summary;
use rustProject::{self, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 則推文：{}", tweet.summarize());
}
