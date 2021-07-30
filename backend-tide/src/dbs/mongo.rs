use crate::util::constant::CONFIG;
use mongodb::{options::ClientOptions, Client, Database};

pub struct DataSource {
    client: Client,
    pub db: Database,
}

#[allow(dead_code)]
impl DataSource {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> DataSource {
        let mut client_options = ClientOptions::parse(CONFIG.get("MONGODB_URI").unwrap())
            .await
            .expect("Failed to parse options!");

        client_options.app_name = Some("tide-graphql-mongodb".to_string());

        let client = Client::with_options(client_options).expect("Failed to init database!");

        let db = client.database(CONFIG.get("MONGODB_NAME").unwrap());

        DataSource {
            client: client,
            db: db,
        }
    }
}
