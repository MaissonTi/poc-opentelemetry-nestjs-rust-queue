use opentelemetry::{propagation::Extractor, propagation::Injector};
use std::collections::HashMap;

pub struct HeaderExtractor<'a>(&'a HashMap<String, String>);

impl<'a> HeaderExtractor<'a> {
    pub fn new(map: &'a HashMap<String, String>) -> Self {
        HeaderExtractor(map)
    }
}

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }
    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect()
    }
}

pub struct HeaderInjector<'a>(pub &'a mut HashMap<String, String>);

impl<'a> Injector for HeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key.to_string(), value);
    }
}


