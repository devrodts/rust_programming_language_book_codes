use ferris_says::say; 
use std::io::{BufWriter};
use std::io::stdout;
fn main(){
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width  = message.chars().count();

    let mut writter = BufWriter::new(stdout.lock());
    say(&message, width, &mut writter).unwrap();
}