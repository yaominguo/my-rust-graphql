mod dbs;
mod gql;
mod users;
mod util;

use crate::gql::{build_schema, graphiql, graphql};
use crate::util::constant::CONFIG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let schema = build_schema().await;
    let state = State { schema: schema };
    let mut app = tide::with_state(state);

    app.at(CONFIG.get("GRAPHQL_PATH").unwrap()).post(graphql);
    app.at(CONFIG.get("GRAPHIQL_PATH").unwrap()).get(graphiql);

    app.listen(format!(
        "{}:{}",
        CONFIG.get("ADDRESS").unwrap(),
        CONFIG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}
