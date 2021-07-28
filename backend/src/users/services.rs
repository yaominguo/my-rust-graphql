use crate::util::constant::CONFIG;
use async_graphql::{Error, ErrorExtensions};
use futures::StreamExt;
use mongodb::Database;

use super::models::User;

pub async fn all_users(db: Database) -> std::result::Result<Vec<User>, async_graphql::Error> {
    let coll = db.collection(CONFIG.get("MONGODB_USERS").unwrap());
    let mut users: Vec<User> = vec![];
    let mut cursor = coll.find(None, None).await.unwrap();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let user = bson::from_bson(bson::Bson::Document(document)).unwrap();
                users.push(user);
            }
            Err(error) => Err(Error::new("6-all-users")
                .extend_with(|_, e| e.set("details", format!("Error to find doc: {}", error))))
            .unwrap(),
        }
    }

    if users.len() > 0 {
        Ok(users)
    } else {
        Err(Error::new("6-all-users").extend_with(|_, e| e.set("details", "No records")))
    }
}
