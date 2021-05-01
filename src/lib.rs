// Traits tell Rust compiler about a functionality a particular type has & can 
// share w/ other types. 
// They define shared behavior in an abstract way.
// Trait bounds specify that a generic can be any type that has a certain behavior.
//
// A types behavior consists of the methods we can call on that type.

pub mod summary {
    pub trait Summary {
        fn summarize(&self) -> String;
        fn read_more(&self) -> String {
            String::from("Read more...")
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
    }
}

