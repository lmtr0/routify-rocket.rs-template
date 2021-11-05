use std::collections::HashMap;

use rocket::{Request, request::{FromRequest, Outcome}};


pub struct RequestInfo {
    pub headers: HashMap<String, String>,
}



#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestInfo {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let reqinfo = RequestInfo {
            headers: HashMap::new(),
        };

        for header in req.headers().iter() {
            println!("{}", &header);
        }

        
        Outcome::Success(reqinfo)
    }
}