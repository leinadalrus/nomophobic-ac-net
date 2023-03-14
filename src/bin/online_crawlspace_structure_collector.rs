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

#[derive(Debug, Clone)]
pub struct Window(pub String);
#[derive(Debug, Clone)]
pub struct Document(pub String);
#[derive(Debug, Clone)]
pub struct Head(pub String);
#[derive(Debug, Clone)]
pub struct Body(pub String);
#[derive(Debug, Clone)]
pub struct Forms(pub String);

#[derive(Debug, Clone)]
pub struct DocumentStructure<T> {
    element: T,
    window: Window,
    document: Document,
    head: Head,
    body: Body,
    forms: Forms,
}

#[rocket::async_trait]
pub trait CrawlerHandler<Item> {
    fn name(&self) -> String;
    fn url(&self) -> Vec<String>;
    fn scrape(
        &self, url: &str,
    ) -> Result<(&Item, std::string::String), Box<(dyn std::error::Error + 'static)>>; // (Vec<Item>, Vec<String>)
    fn process(
        &self, item: Item,
    ) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>;
}

impl<Item> CrawlerHandler<Item> for DocumentStructure<Item> {
    fn name(&self) -> String { "".to_owned() }

    fn url(&self) -> Vec<String> { vec!["".to_owned()] }

    fn scrape(
        &self, url: &str,
    ) -> Result<(&Item, std::string::String), Box<(dyn std::error::Error + 'static)>> {
        let t = &self.element;
        let s = self.url().into_iter().collect();

        Ok((t, s))
    }

    fn process(
        &self, item: Item,
    ) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
        Ok(())
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
