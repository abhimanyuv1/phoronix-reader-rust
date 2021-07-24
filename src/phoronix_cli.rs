// colored
use colored::*;

use crate::article::Article;
use crate::homepage;

fn print_colored() {
    let data = homepage::online().unwrap();
    let phoronix_articles = Article::get_articles(data);
    for article in phoronix_articles {
        println!("title: {}", article.title.red().bold());
        println!("link: {}{}", "https://www.phoronix.com/".blue(), article.link.blue());
        println!("details: {}", article.details.cyan());
        println!("summary: {}\n", textwrap::fill(article.summary.as_str(), 80).green());
    }
}

fn print_no_color() {
    let data = homepage::online().unwrap();
    let phoronix_articles = Article::get_articles(data);
    for article in phoronix_articles {
        println!("title: {}", article.title);
        println!("link: {}{}", "https://www.phoronix.com/", article.link);
        println!("details: {}", article.details);
        println!("summary: {}\n", textwrap::fill(article.summary.as_str(), 60));
    }
}

pub fn print(color: bool) {
    if color {
        print_colored();
    } else {
        print_no_color();
    }
}