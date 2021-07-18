extern crate ureq;
extern crate select;
extern crate colored;

// select
use select::document::Document;
use select::predicate::{Class,Name};
use select::node::Node;
// colored
use colored::*;

fn main() {
    let phoronix_articles = Article::get_articles();
    for article in phoronix_articles {
        println!("title: {}", article.title.red().bold());
        println!("link: {}{}", "https://www.phoronix.com/".blue().underline(), article.link.blue().underline());
        println!("details: {}", article.details.cyan());
        println!("summary: {}\n", article.summary.green());
    }
}

#[allow(dead_code)]
fn open_testing() -> &'static str {
    include_str!("phoronix.html")
}

fn get_phoronix_page() -> Result<String, ureq::Error> {
    let body = ureq::get("https://www.phoronix.com/").call()?.into_string()?;
    Ok(body)
}

struct Article {
    title: String,
    link: String,
    details: String,
    summary: String,
}

impl Article {
    fn get_articles() -> Vec<Article> {
        let data = get_phoronix_page().unwrap();
            Document::from(data.as_str())
                .find(Name("article"))
                .map(|node| Article::new(&node))
                .collect()
    }

    fn new(node: &Node) -> Article {
        let header = node.find(Name("a")).next().unwrap();
        let link = header.attr("href").unwrap();
        let mut details = node.find(Class("details")).next().unwrap().text();
        if details.contains("Add A Comment") {
            details = details.replace("Add A Comment", "0 Comments");
        }
        let summary = node.find(Name("p")).next().unwrap();
        Article{
                title: header.text(),
                link: String::from(link),
                details: details,
                summary: summary.text()
            }
    }
}