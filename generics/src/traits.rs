pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    definition::run();
    traits_as_parameters::run();
    return_trait_implementing_types::run();
    conditional_implementation::run();
}

mod conditional_implementation {
    use std::fmt::Display;

    pub fn run() {}

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

mod return_trait_implementing_types {
    use super::{NewsArticle, Summary, Tweet};

    pub fn run() {}

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

mod traits_as_parameters {
    use std::fmt::Display;

    use super::{NewsArticle, Summary, Tweet};

    pub fn run() {}

    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait bounds
    // fn notify<T: Summary>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }
    //
    // Multiple trait bounds
    // fn notify(item: &(impl Summary + Display)) {}
    // fn notify<T: Summary + Display>(item: &T) {}
    //
    // Where clauses
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    //  where
    //  T: Display + Clone,
    //  U: Clone + Debug {}
}

mod definition {
    use super::{NewsArticle, Summary, Tweet};

    pub fn run() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsbugh Penguins once again are the best \
        hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
    }
}
