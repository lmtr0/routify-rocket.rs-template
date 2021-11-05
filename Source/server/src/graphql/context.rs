use std::sync::{Mutex, 
    // Arc
};
// use mongodb::Database;

use super::CookieJar;

pub struct Ctx {
    pub cookiejar: Mutex<CookieJar>,
    // pub db: Arc<Database>,
    pub user_id: String,
}

impl<'a> juniper::Context for Ctx {}