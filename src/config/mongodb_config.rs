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

pub async fn try_generate_config() -> Result<MongoDbConfig, Error> {
    let uri: String = match std::env::var(MONGODB_URI) {
        Ok(uri) => uri,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::AutoConfigFailure,
                format!("failed to read 'mongodb_uri': {}", error),
            ));
        }
    };

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
