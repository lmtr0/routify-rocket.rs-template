use rocket::{Request, request::{FromRequest, Outcome}};

pub struct AuthGuard {
    pub refresh_token: String,
    // pub access_token: String,
    // pub user: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Use Outcome::Forward and Option<AuthGuard> to routes that don't need authentication upfront but may need later
        if let Some(cookie) = req.cookies().get("rt") {
            log::error!("Auth Cookie found");
         
            // Todo: Request User from database

            // reequest.state();
            Outcome::Success(AuthGuard {
                refresh_token: cookie.value().to_string()
            })
        }
        else {
            Outcome::Success(AuthGuard {
                refresh_token: "".to_string(),
            })
        }
    }
}
