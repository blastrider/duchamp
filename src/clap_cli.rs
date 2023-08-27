use clap::Parser;
///ça s'utilise comme ça maintenat pas comme dans le bouquin.
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    firstname: String,
    #[arg(short, long)]
    lastname: String,
    #[arg(short, long)]
    age: u16,


    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {} {}, you are {} yo !", args.firstname, args.lastname, args.age)
    }
}
