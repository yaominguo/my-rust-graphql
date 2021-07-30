pub mod mutations;
pub mod queries;

use crate::util::constant::CONFIG;
use crate::State;
use crate::{
    dbs::mongo,
    gql::{mutations::MutationRoot, queries::QueryRoot},
};
use async_graphql::{
    http::{playground_source, receive_json, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use tide::{http::mime, Body, Request, Response, StatusCode};

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let mongo_ds = mongo::DataSource::init().await;

    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .finish()
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
    res.set_body(playground_source(GraphQLPlaygroundConfig::new(
        CONFIG.get("GRAPHQL_PATH").unwrap(),
    )));
    res.set_content_type(mime::HTML);

    Ok(res.into())
}
