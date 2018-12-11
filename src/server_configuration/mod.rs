extern crate serde_derive;
extern crate dotenv;
extern crate envy;

pub mod server_configuration {

    #[derive(serde_derive::Deserialize, Debug)]
    pub struct Configuration {
        pub database_url: String,
        pub temp_folder: String
    }

    pub fn init() -> Configuration {
        dotenv::dotenv().expect("Failed to read .env file");
        match envy::from_env::<Configuration>() {
            Ok(config) => config,
            Err(e) => panic!(e)
        }
    }
}