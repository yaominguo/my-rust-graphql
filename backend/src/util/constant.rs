use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CONFIG: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "ADDRESS",
            dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!"),
        );
        map.insert(
            "PORT",
            dotenv::var("PORT").expect("Expected PORT to be set in env!"),
        );
        map.insert(
            "GRAPHQL_PATH",
            dotenv::var("GRAPHQL_PATH").expect("Expected GRAPHQL_PATH to be set in env!"),
        );
        map.insert(
            "GRAPHIQL_PATH",
            dotenv::var("GRAPHIQL_PATH").expect("Expected GRAPHIQL_PATH to be set in env!"),
        );
        map.insert(
            "MONGODB_URI",
            dotenv::var("MONGODB_URI").expect("Expected MONGODB_URI to be set in env!"),
        );
        map.insert(
            "MONGODB_NAME",
            dotenv::var("MONGODB_NAME").expect("Expected MONGODB_NAME to be set in env!"),
        );
        map.insert(
            "MONGODB_USERS",
            dotenv::var("MONGODB_USERS").expect("Expected MONGODB_USERS to be set in env!"),
        );
        map
    };
}
