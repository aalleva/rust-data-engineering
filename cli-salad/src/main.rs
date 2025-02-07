use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    author = "Andres Alleva", 
    version = "1.0",
    about = "Number of fruits to include in the salad", 
    long_about = None
)]

struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    let fruit_salad = create_fruit_salad(num_fruits);

    // Print
    println!("Created fruit salad with {} fruits: {:?}", num_fruits, fruit_salad);
}
