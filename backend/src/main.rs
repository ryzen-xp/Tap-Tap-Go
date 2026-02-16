use std::env;

mod server;

fn main() {

    let url = env::var("DATABASE_URL").expect("Failed to read DATABASE URL!!");

     
}
