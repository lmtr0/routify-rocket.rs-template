use rocket::http::{Cookie, SameSite};

use super::Ctx;

pub struct Query;

#[juniper::graphql_object(Context = Ctx)]
impl Query {
    pub async fn api_version() -> String {
        "v1.0.0".to_string()
    }

    pub async fn add_cookie(ctx: &mut Ctx, key: String, value: String) -> String {

        ctx.cookiejar.lock().expect("Failed to lock cookies").add( &Cookie::build(key, value).http_only(false).expires(None).secure(true).same_site(SameSite::Lax).finish());
        "DONE".to_string()
    }

    pub async fn verify_cookie(ctx: &mut Ctx, key: String, value: String) -> bool {

        let mut cookies = ctx.cookiejar.lock().expect("Failed to lock cookies");
        if let Some(cookie) = cookies.get( &key) {
            if cookie.value() == value.as_str() {
                return true;
            }
            
            return false;
        }
        false
    }

    pub async fn remove_cookie(ctx: &mut Ctx, key: String) -> bool {

        let mut cookies = ctx.cookiejar.lock().expect("Failed to lock cookies");
        cookies.remove(&key);
        true
    }
}