use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v,
            Err(_) => "Error loading env variable".to_string(),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("nest-mongo");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        let user = Option::expect(
            self.col.insert_one(new_doc, None).await.ok(),
            "Error creating user",
        );

        Ok(user)
    }
    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let _obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": _obj_id};
        let user_detail = Option::expect(
            self.col.find_one(filter, None).await.ok(),
            "Error Gettting User",
        );

        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let _obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": _obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "name": new_user.name,
                "location": new_user.location,
                "title": new_user.title
            }
        };

        let update_doc = Option::expect(
            self.col.update_one(filter, new_doc, None).await.ok(),
            "Error Update",
        );

        Ok(update_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let _obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": _obj_id};
        let user_detail = Option::expect(
            self.col.delete_one(filter, None).await.ok(),
            "Error Delete user",
        );
        Ok(user_detail)
    }
}
