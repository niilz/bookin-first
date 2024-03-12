use std::error::Error;

use reqwest::{
    cookie::{CookieStore, Jar},
    Url,
};

pub trait Cookie {
    fn read_cookie(&self, domain: &str) -> Result<String, Box<dyn Error>>;
}

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
