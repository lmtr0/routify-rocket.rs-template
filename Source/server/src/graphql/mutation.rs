use super::Ctx;

use juniper::{GraphQLInputObject};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub struct Mutation;

#[juniper::graphql_object(Context = Ctx)]
impl Mutation {
    async fn createSomething(context: &Ctx, input: CustomInput) -> String {
        let mut input = input;
        let col = context.db.collection::<CustomInput>("accounts");
        input._id = Uuid::new_v4().to_string();
        let res = col.insert_one(input, None).await;
        
        if let Err(e) = res {
            log::error!("Error while creating document {}", e);
            String::from("databse error")
        }
        else {
            String::from("Hello From Mutation")
        }

    }
}

#[derive(GraphQLInputObject, Serialize, Deserialize)]
pub struct CustomInput {
    pub _id: String,
    pub hello: String,
}