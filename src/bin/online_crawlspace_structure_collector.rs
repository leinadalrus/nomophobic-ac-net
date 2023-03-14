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

pub struct DocumentCrawler<T> {
    t: T,
    crawling_concurrency: usize,
    processing_concurrency: usize,
}

#[rocket::async_trait]
pub trait CrawlerHandler<Item> {
    fn name(&self) -> String;
    fn url(&self) -> Vec<String>;
    fn scrape(
        &self, url: &str,
    ) -> Result<
        (&Item, std::string::String),
        Box<(dyn std::error::Error + 'static)>,
    >; // (Vec<Item>, Vec<String>)
    fn process(
        &self, item: Item,
    ) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>>;
}

impl<Item> CrawlerHandler<Item> for DocumentStructure<Item> {
    fn name(&self) -> String { "".to_owned() }

    fn url(&self) -> Vec<String> { vec!["".to_owned()] }

    fn scrape(
        &self, url: &str,
    ) -> Result<
        (&Item, std::string::String),
        Box<(dyn std::error::Error + 'static)>,
    > {
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

impl<X> DocumentCrawler<X> {
    pub /* async */ fn run<T: Send + 'static>(
        self: &Self, handler: Arc<dyn CrawlerHandler<T>>,
    ) {
        let mut visited_urls = std::collections::HashSet::<String>::new();
        let crawling_concurrency = self.crawling_concurrency;
        let crawling_queue_capacity = crawling_concurrency * 512;
        let processing_concurrency = self.processing_concurrency;
        let processing_queue_capacity = processing_concurrency * 8;
        let active_crawlers = Arc::new(AtomicUsize::new(0));

        let (urls_to_visit_x1, urls_to_visit_y1) =
            tokio::sync::mpsc::channel::<T>(crawling_queue_capacity);
        let (items_x1, items_y1) =
            tokio::sync::mpsc::channel::<T>(processing_queue_capacity);
        let (new_urls_x1, mut new_urls_y1) =
            tokio::sync::mpsc::channel::<T>(crawling_queue_capacity);
        let barrier_x1 = Arc::new(Barrier::new(1));

        for url in handler.url() {
            visited_urls.insert(url.clone());
            // let _ = urls_to_visit_x1.send().await;
        }

        loop {
            if new_urls_y1.try_recv().ok().is_some() {
                for url in handler.url() {
                    visited_urls.insert(url.clone());
                    // let _ = urls_to_visit_x1.send().await;
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        // let _ = urls_to_visit_x1.send(url).await;
                    }
                }
            }

            if new_urls_x1.capacity() == crawling_queue_capacity
                && urls_to_visit_x1.capacity() == crawling_queue_capacity
                && active_crawlers.load(Ordering::SeqCst) == 0
            {
                // no more work, we leave
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(8))/* .await */;
        }
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
