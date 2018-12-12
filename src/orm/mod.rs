use crate::server_configuration::Configuration;

use self::schema::movies;
use self::schema::movies::dsl::*;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Queryable, Insertable};
use diesel::pg::PgConnection;
use std::result::Result;

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

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie<'a> {
    pub title: &'a String,
    pub synopsis: &'a String,
    pub poster: Option<&'a String>,
    pub rating: i32
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
    
    pub fn get_movies(&self) -> Vec<Movie> {  
        let connection = establish_connection(self.configuration);
        movies.limit(5).load::<Movie>(&connection).expect("Error loading movies")
    }

    pub fn create_movie(&self, movie: &Movie) -> Result<Movie, Error> {
        let connection = establish_connection(self.configuration);
        let new_movie = NewMovie {
            title: &movie.title,
            synopsis : &movie.synopsis,
            poster: movie.poster.as_ref(),
            rating: movie.rating.unwrap_or(1)
        };
        diesel::insert_into(movies::table).values(&new_movie).get_result(&connection)
    }

}