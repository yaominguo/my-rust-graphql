use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub username: String,
}

// 使用简单对象类型async_graphql::SimpleObject后则不再需要以下的getter、setter方法
// #[async_graphql::Object]
// impl User {
//     pub async fn id(&self) -> ObjectId {
//         self._id.clone()
//     }

//     pub async fn email(&self) -> &str {
//         self.email.as_str()
//     }

//     pub async fn username(&self) -> &str {
//         self.username.as_str()
//     }
// }

// 还是可以用以下方法进行个别字段特殊定义
#[async_graphql::ComplexObject]
impl User {
    pub async fn info(&self) -> String {
        let mut info = String::new();
        info.push_str(&self.username);
        info.push_str("<");
        info.push_str(&self.email);
        info.push_str(">");

        info
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct NewUser {
    pub email: String,
    pub username: String,
}
