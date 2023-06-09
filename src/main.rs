use std::env;
use std::process;

use minigrep::Config;


fn main() {
let args: Vec<String> = env::args().collect(); // collect the args from console

let config = Config::build(&args).unwrap_or_else(|err| {
    println!("err with arugments {}", err);
    process::exit(1);
});

println!("Searchterm {}", config.query);
println!("file {}", config.file_path);

if let Err(e) = minigrep::run(config) {
    println!("application error {e}");
    process::exit(1);
}

}
