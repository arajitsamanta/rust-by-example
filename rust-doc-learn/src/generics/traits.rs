use rust_doc_learn::{Summary,Tweet,NewsArticle};

pub fn traits_ex(){
    println!("\n====== Traits - Shared behavior ======");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    println!("**** Traits as parameter");
    rust_doc_learn::notify(&article);
    rust_doc_learn::notify_same_trait_type(&tweet, &article);
    rust_doc_learn::notify_trait_bound_syntax(&tweet, &tweet);
    rust_doc_learn::notify_trait_bound_with_plus_sign(&tweet);
}


