use mongodb::options::ClientOptions;

use crate::error::{Error, ErrorKind};

const MONGODB_URI: &str = "MONGODB_URI";

pub struct MongoDbConfig {
    client_options: ClientOptions,
}

impl MongoDbConfig {
    pub fn client_options(self) -> ClientOptions {
        self.client_options
    }
}

pub async fn try_generate_config(uri: String) -> Result<MongoDbConfig, Error> {
    let client_options = match ClientOptions::parse(uri).await {
        Ok(client_options) => client_options,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::AutoConfigFailure,
                format!("failed to parse mongodb_uri: {:?}", error),
            ));
        }
    };

    Ok(MongoDbConfig { client_options })
}
