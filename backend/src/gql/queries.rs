pub struct QueryRoot;
use async_graphql::Context;

use crate::{
    dbs::mongo::DataSource,
    users::{self, models::User},
};

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn all_users(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<User>, async_graphql::Error> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::all_users(db).await
    }
}
