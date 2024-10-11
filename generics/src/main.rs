mod compare;
mod traits;

use compare::{get_largest, get_largest_f32, get_largest_i32};
use traits::{NewsArticle, Summary, Tweet};

fn main() {
    let number_list_i32 = vec![3, 44, 2, 332, 3, 223, 2, 3];
    let largest_number_i32 = get_largest_i32(&number_list_i32);
    println!("the largest i32 is {largest_number_i32}");

    let number_list_f32 = vec![3.4, 4.3, 4.2, 3.32, 3.1, 2.23, 2.3];
    let largest_number_f32 = get_largest_f32(&number_list_f32);
    println!("the largest f32 is {largest_number_f32}");

    let largest_number_i32 = get_largest(&number_list_i32);
    let largest_number_f32 = get_largest(&number_list_f32);
    println!("the largest i32 is {largest_number_i32} and f32 is {largest_number_f32}");

    let news_article = NewsArticle {
        headline: String::from("Gachagua Impeached"),
        location: String::from("Nairobi, Kenya"),
        author: String::from("Samson Rapando"),
        content: String::from("This is some dummy content"),
    };
    println!(
        "article summary: {} {}",
        news_article.summarize(),
        news_article.read_more()
    );
    println!("article        : {}", news_article.all());
    traits::notify(&news_article);

    let tweet = Tweet {
        username: String::from("_rapando"),
        content: String::from("This is a sample tweet"),
        reply: false,
        retweet: true,
    };

    println!(
        "\ntweet summary: {} {}",
        tweet.summarize(),
        tweet.read_more()
    );
    println!("tweet        : {}", tweet.all());
    traits::notify(&tweet);
}
