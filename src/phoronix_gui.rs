
extern crate gtk;
extern crate gdk;
extern crate pango;

use crate::article::Article;
use crate::homepage;
use gtk::prelude::*;

fn generate_article_widgets(container: &gtk::Box, articles: &Vec<Article>) {
    for article in articles {
        let url = format!("https://phoronix.com/{}", article.link);
        let title_and_url = gtk::LinkButton::with_label(&url, &article.title);
        title_and_url.set_halign(gtk::Align::Start);
        title_and_url.set_widget_name("titleandurl");

        // Details of the article inside of a gtk::TextView
        let details = gtk::TextView::new();
        details.set_left_margin(10);
        details.set_right_margin(10);
        details.set_editable(false);
        details.buffer().unwrap().set_text(&article.details);
        details.set_widget_name("details");

        // Summary of the article inside of a gtk::TextView
        let summary = gtk::TextView::new();
        summary.set_wrap_mode(gtk::WrapMode::Word);
        summary.set_left_margin(10);
        summary.set_right_margin(10);
        summary.set_pixels_above_lines(10);
        summary.set_pixels_below_lines(10);
        summary.set_editable(false);
        summary.buffer().unwrap().set_text(&article.summary);
        summary.set_widget_name("summary");

        // Attach the title+url, details and summary to the article_box
        container.add(&title_and_url);
        container.add(&details);
        container.add(&summary);
        container.add(&gtk::Separator::new(gtk::Orientation::Horizontal));
    }
} 

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Phoronix Reader");
    let (width, height) = (800,600);
    window.set_default_size(width, height);
    window.connect_key_press_event(move |_,key| {
        match key.keyval() {
            gdk::keys::constants::Escape => gtk::main_quit(),
            _ => ()
        }
        Inhibit(false)
    });

    let articles = Article::get_articles(homepage::online().unwrap());

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_window.set_min_content_width(600);    
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    generate_article_widgets(&container, &articles);
    scrolled_window.add(&container);
    window.add(&scrolled_window);
    window.show_all();                                                                              
}

pub fn phoronix_gui() {
    // Create a new application
    let app = gtk::Application::new(Some("com.yoshilab.PhoronixReader"), Default::default());
    app.connect_activate(|app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        // Load the CSS file
        let style = include_bytes!("style.css");
        provider.load_from_data(style).expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        on_activate(app);
    });
    app.run();
}