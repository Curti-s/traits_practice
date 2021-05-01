use traits::tweet::Tweet;
use traits::news_article::NewsArticle;
use traits::summary::Summary;
use traits::notify::{notify, another_notify};
use traits::summarizable::{summarizable_tweet, summarizable_news_article};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content:  String::from("of course, as you probably already know, people"),
        reply:    false,
        retweet:  false,
    };

    println!("{}", tweet.read_more());
    println!("{:?}", notify(&tweet));
    println!("{:?}", another_notify(&tweet));


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(" \
              The Pittsburgh Penguis once again are the best \
              hockey team in NHL"),
    };

    println!("{}", article.read_more());
    println!("{:?}", notify(&article));
    println!("{:?}", another_notify(&article));

    let summary_tweet = summarizable_tweet(
        String::from("Kent Katchmi"), 
        String::from("You see me rolling, they hating, patrollin' they tryna catch me riding dirty"),
        false, false);
    println!("{}", summary_tweet);

    let summary_news_article = summarizable_news_article(
        String::from("Guantanamo diaries"),
        String::from("Timbuktu"),
        String::from("The Mauritanian"),
        String::from(
            "Storyline Based on the NY Times best-selling memoir Guantánamo Diary by Mohamedou Ould Slahi,\
             this is the true story of Slahi's fight for freedom after being detained and imprisoned without charge by the U.S. Government for years."));
    println!("{:?}", summary_news_article);
}
