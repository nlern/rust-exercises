use std::fmt::{Display, Debug};
use aggregator::{Summary, Tweet, NewsArticle, notify};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_books"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("Penuins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
       hockey team in the NHL.",
    ),
  };

  notify(&article);
}

fn some_function<T, U>(t: &T, u: &U) -> i32 
  where T: Display + Clone,
        U: Clone + Debug
{10}

fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_books"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  }
}
