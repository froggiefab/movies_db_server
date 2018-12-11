extern crate serde_derive;
extern crate dotenv;
extern crate envy;

pub mod server_configuration {

    #[derive(serde_derive::Deserialize, Debug)]
    pub struct Configuration {
        pub name: String
    }

    pub fn init() -> String {
        dotenv::dotenv().ok();
        String::from("OK !")
    }
}