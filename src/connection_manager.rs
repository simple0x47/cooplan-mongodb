use mongodb::Client;
use mongodb::error::Error;

use crate::config::mongodb_config::MongoDbConfig;

pub struct MongoDbConnectionManager {
    client: Client,
}

//TODO: Implement connection failure handling
impl MongoDbConnectionManager {
    pub fn try_new(mongodb_config: MongoDbConfig) -> Result<MongoDbConnectionManager, Error> {
        let client = match Client::with_options(mongodb_config.client_options()) {
            Ok(client) => client,
            Err(error) => {
                return Err(error);
            }
        };

        Ok(MongoDbConnectionManager { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
