pub struct QueryRoot;
use async_graphql::Context;

use crate::{
    dbs::mongo::DataSource,
    users::{self, models::User},
    util::constant::GqlResult,
};

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::all_users(db).await
    }

    async fn get_user_by_email(&self, ctx: &Context<'_>, email: String) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        users::services::get_user_by_email(db, &email).await
    }
}
