use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response};

use self::queries::QueryRoot;
use actix_web::{web, HttpResponse, Result};

pub mod mutations;
pub mod queries;

type ActixSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn build_schema() -> ActixSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}
