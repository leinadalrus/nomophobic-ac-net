use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

use tokio::sync::Barrier;

pub struct NetworkDomainOperators;
pub struct NetworkDomainDorks;
pub struct SearchEngine;

#[rocket::async_trait]
pub trait CrawlerHandler {
    type Item: std::fmt::Debug + Clone; // maybe trait Sized would do better than Debug?

    fn name(&self) -> String;
    fn url(&self) -> Vec<String>;
    fn scrape(
        &self, url: &str,
    ) -> std::result::Result<
        (Vec<Self::Item>, Vec<String>),
        std::boxed::Box<dyn std::error::Error>,
    >;
    fn process(
        &self, item: Self::Item,
    ) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>;
}

impl CrawlerHandler for SearchEngine {
    type Item: std::fmt::Debug + Clone;

    // maybe trait Sized would do better than Debug?

    fn name(&self) -> String {
        "".to_owned()
    }

    fn url(&self) -> Vec<String> {
        vec!["".to_owned()]
    }

    fn scrape(
        &self, url: &str,
    ) -> std::result::Result<
        (Vec<Self::Item>, Vec<String>),
        std::boxed::Box<dyn std::error::Error>,
    > {
    }

    fn process(
        &self, item: Self::Item,
    ) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl SearchEngine {
    pub async fn init<T: Send + 'static>(
        &self, crawler: Arc<dyn CrawlerHandler<Item = T>>,
    ) {
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn head() -> web_sys::HtmlElement {
    document()
        .head()
        .expect("document should have a head")
        .into()
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn forms() -> web_sys::HtmlCollection { document().forms() }

#[tokio::main]
async fn main() {
    let barrier_1 = Arc::new(Barrier::new(1));
    let barrier_2 = barrier_1.clone();
    let barrier_3 = barrier_1.clone();

    tokio::spawn(async move {
        barrier_2.wait().await;
    });

    tokio::spawn(async move {
        barrier_3.wait().await;
    });

    barrier_1.wait().await;
}
