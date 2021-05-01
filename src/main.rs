use traits::tweet::Tweet;
use traits::summary::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content:  String::from("of course, as you probably already know, people"),
        reply:    false,
        retweet:  false,
    };

    println!("{}", tweet.read_more());
    println!("1 new tweet: {}", tweet.summarize());
}
