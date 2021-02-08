trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    _headline: String,
    _location: String,
    author: String,
    _content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    _reply: bool,
    _retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn _notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn _notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn _notify3<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        _reply: false,
        _retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        _headline: String::from("Penguins win the Stanley Cup Championship!"),
        _location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        _content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
