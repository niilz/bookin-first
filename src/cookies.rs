#[cfg(not(target_family = "wasm"))]
use reqwest::cookie::{CookieStore, Jar};
use reqwest::Url;
use std::{error::Error, sync::Arc};

pub trait Cookie {
    fn read_cookie(&self, domain: &str) -> Result<String, Box<dyn Error>>;
}

#[cfg(not(target_family = "wasm"))]
impl Cookie for Jar {
    fn read_cookie(&self, domain: &str) -> Result<String, Box<dyn Error>> {
        let cookie_domain = Url::parse(domain)?;
        Ok(self
            .cookies(&cookie_domain)
            .ok_or("no cookie present")?
            .to_str()?
            .to_string())
    }
}

impl<T> Cookie for Arc<T>
where
    T: Cookie,
{
    fn read_cookie(&self, domain: &str) -> Result<String, Box<dyn Error>> {
        self.as_ref().read_cookie(domain)
    }
}
