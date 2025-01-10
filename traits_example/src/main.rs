use traits_example::{notify, Summary, Task, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("Suhird"),
        content: String::from("Learning Rust right now..."),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet {}", tweet.summarize());
    println!("{:?}", notify(&tweet));
    let task = Task {
        heading: String::from("Create a performance test"),
        details: String::from("Create a perf test in locust for custom attributes"),
    };
    println!("{}", task.summarize());
}
