extern crate hyper;
extern crate select;
extern crate colored;

use select::document::Document;
use select::predicate::{Class,Name};
use select::node::Node;

use colored::*;

fn main() {
    let phoronix_articles = Article::get_articles();
    for article in phoronix_articles {
        println!("title: {}", article.title.red().bold());
        println!("link: {}", article.link.blue().underline());
        println!("details: {}", article.details.cyan());
        println!("summary: {}\n", article.summary.green());
    }
}

fn open_testing() -> &'static str {
    include_str!("phoronix.html")
}

struct Article {
    title: String,
    link: String,
    details: String,
    summary: String,
}

impl Article {
    fn get_articles() -> Vec<Article> {
        Document::from(open_testing())
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