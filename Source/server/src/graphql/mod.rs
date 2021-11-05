use std::{
    sync::{Arc, Mutex},
    collections::HashMap,
};

use juniper::{EmptySubscription, RootNode};
use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use mongodb::Database;
use rocket::{State, http::{ContentType, Cookie, Status}, response::content::Html};


mod context;
mod query;
mod mutation;
mod cookies;
mod altair;
// mod subscription;

pub use self::cookies::CookieJar;
pub use self::query::Query;
pub use self::mutation::Mutation;
pub use self::context::Ctx;
pub use super::guards::{AuthGuard};
// pub use subscription::Subscription;


pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Ctx>>;

#[rocket::get("/playground")]
pub fn gql_playground_handler() ->(Status, (ContentType, &'static str)) {
    (Status::Ok, (ContentType::HTML, altair::SOURCE))
}

#[rocket::get("/graphiql")]
pub fn gql_graphiql_handler() -> Html<String> {
    juniper_rocket::graphiql_source("/api/gql", None)
}

#[rocket::post("/gql", data = "<request>")]
pub async fn gql_handler(
    request: GraphQLRequest,
    // db: &State<Arc<Database>>,
    schema: &State<Schema>,
    
    auth: AuthGuard,
    req_cookies: &rocket::http::CookieJar<'_>,
) -> GraphQLResponse {
    let mut jar: HashMap<String, Cookie<'static>> = HashMap::new();
    // let db = Arc::clone(db);

    // gets the cookies to the request
    for cookie in req_cookies.iter() {
        jar.insert(cookie.name().to_string(), cookie.clone());
    }


    let context = Ctx {
        cookiejar: Mutex::new(CookieJar::new(jar)),
        // db,
        user_id: auth.refresh_token,
    };

    let response = request.execute(schema, &context).await;

    // add and remove the cookies that the graphql server processed
    let cookies = context.cookiejar.lock().unwrap();

    for cookie in &cookies.added_cookies {
        req_cookies.add(Cookie::parse(cookie.clone()).unwrap())
    }
    
    for cookie in &cookies.removed_cookies {
        println!("removing from request {}", cookie);
        req_cookies.remove(Cookie::parse(cookie.clone()).unwrap());
        println!("removed from request");
    }

    // return the response
    response
}


pub fn generate_schema() -> Schema {
    Schema::new(
        Query, 
        Mutation, 
        EmptySubscription::new()
    )
}
