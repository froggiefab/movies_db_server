#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod server_configuration;
mod orm;

#[get("/")]
fn index() -> &'static str {
    "Hello world !"
}

fn main() {
    let configuration = server_configuration::init();
    let movie_service = orm::MovieService::new(&configuration);

    let movie1 = orm::Movie {
        id: 1,
        title: String::from("Toxic avengers"),
        synopsis: String::from("Meh"),
        poster: None,
        rating: None
    };
    movie_service.create_movie(&movie1).expect("L'insertion a échouée");
    println!("{:?}", movie_service.get_movies());

    // Démarrage serveur Rocket
    rocket::ignite().mount("/", routes![index]).launch();
}