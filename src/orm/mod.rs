
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::server_configuration::Configuration;

mod schema;

fn establish_connection(config: &Configuration) -> PgConnection {
    PgConnection::establish(&config.database_url)
        .expect(&format!("Error connecting to {}", config.database_url))
}

#[derive(Queryable, Debug)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub synopsis: String,
    pub poster: Option<String>,
    pub rating: Option<i32>
}

pub struct MovieService<'a> {
    configuration: &'a Configuration
}

impl<'a> MovieService<'a> {
    
    pub fn new(config: &'a Configuration) -> MovieService {
        MovieService {
            configuration: config
        }
    }
    
    pub fn getMovies(&self) -> Vec<Movie> {  
        let connection = establish_connection(self.configuration);
        schema::movies::table.limit(5).load::<Movie>(&connection).expect("Error loading movies")
    }

}