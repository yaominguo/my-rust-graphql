pub mod queries;
use crate::gql::queries::QueryRoot;
use crate::State;
use async_graphql::{
    http::{playground_source, receive_json, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use tide::{http::mime, Body, Request, Response, StatusCode};

pub async fn build_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub async fn graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    let gql_res = schema.execute(receive_json(req).await?).await;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&gql_res)?);

    Ok(res.into())
}

pub async fn graphiql(_: Request<State>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(playground_source(GraphQLPlaygroundConfig::new("graphql")));
    res.set_content_type(mime::HTML);

    Ok(res.into())
}
