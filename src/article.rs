// select
use select::document::Document;
use select::predicate::{Class,Name};
use select::node::Node;

pub struct Article {
    pub title: String,
    pub link: String,
    pub details: String,
    pub summary: String,
}

impl Article {
    pub fn get_articles(data: String) -> Vec<Article> {
            Document::from(data.as_str())
                .find(Name("article"))
                .map(|node| Article::new(&node))
                .collect()
    }

    fn new(node: &Node) -> Article {
        let header = node.find(Name("a")).next().unwrap();
        let mut link = String::from(header.attr("href").unwrap());
        if link.starts_with("/") {
            assert_eq!(link.remove(0),'/');
        }
        let mut details = node.find(Class("details")).next().unwrap().text();
        if details.contains("Add A Comment") {
            details = details.replace("Add A Comment", "0 Comments");
        }
        let summary = node.find(Name("p")).next().unwrap();
        Article{
                title: header.text(),
                link: link,
                details: details,
                summary: summary.text()
            }
    }
}