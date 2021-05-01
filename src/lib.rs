// Traits tell Rust compiler about a functionality a particular type has & can 
// share w/ other types. 
// They define shared behavior in an abstract way.
// Trait bounds specify that a generic can be any type that has a certain behavior.
//
// A types behavior consists of the methods we can call on that type.
//
// Note that it isn't possible to call the default implementation from an 
// overriding implementation of that same method.

pub mod summary {
    pub trait Summary {
        fn summarize(&self) -> String;
        fn summarize_author(&self) -> String;
        fn read_more(&self) -> String {
            format!("Read more from {}", self.summarize_author())
        }
    }
}

pub mod news_article {
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author:   String,
        pub content:  String,
    }

    impl crate::summary::Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            // self.read_more(); this is not allowed
            format!("@{}", self.author)
        }
    }
}

pub mod tweet {
    pub struct Tweet {
        pub username: String,
        pub content:  String,
        pub reply:    bool,
        pub retweet:  bool,
    }

    impl crate::summary::Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            // self.read_more(); this is not allowed
            format!("@{}", self.username)
        }
    }
}

pub mod notify {
    pub fn notify(item: &impl crate::summary::Summary) { // traits as params
        println!("Breaking news! {} \n", item.summarize());
    }

    pub fn another_notify<T: crate::summary::Summary>(item: &T) { // trait bounds
        println!("Another breaking news: {} \n", item.summarize());
    }
}


// returning types that implement traits -> useful esp w/i context of closures and iterators
pub mod summarizable {
    pub fn summarizable_tweet(username: String, content:String, reply:bool, retweet:bool) -> impl crate::summary::Summary {
        crate::tweet::Tweet {
            username,
            content,
            reply,
            retweet,
        }
    }

    pub fn summarizable_news_article(headline: String, location: String, author: String, content: String) -> impl crate::summary::Summary {
        crate::news_article::NewsArticle {
            headline,
            location,
            author ,
            content,
        }
    }
}
