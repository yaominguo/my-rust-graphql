use crate::gql::{build_schema, graphiql, graphql};

mod gql;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let schema = build_schema().await;
    let state = State { schema: schema };
    let mut app = tide::with_state(state);

    app.at("graphql").post(graphql);
    app.at("graphiql").get(graphiql);

    app.listen(format!("{}:{}", "127.0.0.1", "8080")).await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    >,
}
