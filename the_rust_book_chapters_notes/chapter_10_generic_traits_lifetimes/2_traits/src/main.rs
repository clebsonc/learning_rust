// Traits works just like interfaces in Java or abstract base class in python.
// This means that a trait is basically a definition that other types must
// implement its behavior. Example:



pub trait Summary {
    // this is the behavior that a type implementing Summary must have.
    // If it is necessary to have a default version for the behavior, just
    // implement it here. In this example there is no default behavior, so the
    // type that implements Symmary mus implement it.
    fn summarize(&self) -> String;  
}


// Creating a type to use the Summary trait.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing the behavior summarize for for the type NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let article = NewsArticle {
        headline: "Tonight Show".to_string(),
        location: "Belo Horizonte".to_string(),
        author: "The invisible Man".to_string(),
        content: "there was a night with a show. The show happened in the night. It happened tonight.".to_string(),
    };

    println!("Summary:\n {}", article.summarize());
}
