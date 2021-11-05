use std::collections::HashMap;

use rocket::http::Cookie;


pub struct CookieJar {
    jar: HashMap<String, Cookie<'static>>,
    pub removed_cookies: Vec<String>,
    pub added_cookies: Vec<String>,
}

impl CookieJar {
    pub fn new (jar: HashMap<String, Cookie<'static>>) -> Self {
        CookieJar {
            jar,
            removed_cookies: Vec::new(),
            added_cookies: Vec::new(),
        }
    }

    pub fn add(&mut self, cookie: &Cookie<'static>)  {
        self.added_cookies.push(cookie.to_string());
        self.jar.insert(cookie.name().to_string(), cookie.clone());
    }

    pub fn remove(&mut self, cookie: &String)  {
        let cookie_to_remoe = self.get(cookie).unwrap();

        self.removed_cookies.push(cookie_to_remoe.to_string());
        if self.jar.contains_key(cookie) {
            self.jar.remove(cookie).expect("Failed to remove cookie");
        }
    }

    pub fn get(&mut self, name: &String) -> Option<Cookie<'static>>  {
        if !self.jar.contains_key(name) {
            return None;
        }

        Some(self.jar[name].clone())
    }
}
