mod server_configuration;

fn main() {
    let configuration = server_configuration::init();
    println!("Hello, world!");
    println!("{:?}", configuration);
}