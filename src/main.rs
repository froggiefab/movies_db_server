#[macro_use] extern crate diesel;

mod server_configuration;

mod orm;

fn main() {
    let configuration = server_configuration::init();
    let movie_service = orm::MovieService::new(&configuration);
    println!("{:?}", movie_service.getMovies());
}