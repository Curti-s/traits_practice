use traits::tweet::Tweet;
use traits::news_article::NewsArticle;
use traits::summary::Summary;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content:  String::from("of course, as you probably already know, people"),
        reply:    false,
        retweet:  false,
    };

    println!("{}", tweet.read_more());
    println!("1 new tweet: {} \n", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(" \
              The Pittsburgh Penguis once again are the best \
              hockey team in NHL"),
    };

    println!("{}", article.read_more());
    println!("New aritcle: {}", article.summarize());
}
