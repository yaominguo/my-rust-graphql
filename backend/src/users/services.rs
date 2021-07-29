use crate::util::constant::{GqlResult, CONFIG};
use async_graphql::{Error, ErrorExtensions};
use futures::StreamExt;
use mongodb::Database;

use super::models::{NewUser, User};

pub async fn all_users(db: Database) -> GqlResult<Vec<User>> {
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

pub async fn get_user_by_email(db: Database, email: &str) -> GqlResult<User> {
    let coll = db.collection(CONFIG.get("MONGODB_USERS").unwrap());
    let data = coll.find_one(bson::doc! {"email": email}, None).await;

    if let Ok(user_data) = data {
        if let Some(user_info) = user_data {
            let user: User = bson::from_bson(bson::Bson::Document(user_info)).unwrap();
            Ok(user)
        } else {
            Err(Error::new("2-email").extend_with(|_, e| e.set("details", "email not exist")))
        }
    } else {
        Err(Error::new("2-email")
            .extend_with(|_, e| e.set("details", "something wrong with mongodb")))
    }
}

pub async fn new_user(db: Database, mut new_user: NewUser) -> GqlResult<User> {
    let coll = db.collection(CONFIG.get("MONGODB_USERS").unwrap());
    new_user.email = new_user.email.to_lowercase();

    if self::get_user_by_email(db.clone(), &new_user.email)
        .await
        .is_ok()
    {
        Err(Error::new("2-email").extend_with(|_, e| e.set("details", "email already exist")))
    } else {
        let new_user_bson = bson::to_bson(&new_user).unwrap();

        if let bson::Bson::Document(document) = new_user_bson {
            coll.insert_one(document, None)
                .await
                .expect("文档插入mongodb集合时出错");
            self::get_user_by_email(db.clone(), &new_user.email).await
        } else {
            Err(Error::new("3-new_user")
                .extend_with(|_, e| e.set("details", "将BSON对象转换为mongodb文档时出错")))
        }
    }
}
