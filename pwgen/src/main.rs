use clap::Parser;
use passwords::PasswordGenerator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 25)]
    length: usize,
    #[arg(long, default_value_t = true)]
    no_numbers: bool,
    #[arg(long, default_value_t = true)]
    no_symbols: bool,
    #[arg(long, default_value_t = false)]
    no_lowercase_letters: bool,
    #[arg(long, default_value_t = true)]
    no_uppercase_letters: bool,
    #[arg(
        long,
        default_value_t = true,
        help = "Include similar characters like 'l', '1' and  'I'"
    )]
    include_similar_chars: bool,
    #[arg(long, default_value_t = true, help = "Spaces in the password")]
    spaces: bool,
}

fn main() {
    let args = Args::parse();
    let pg = PasswordGenerator {
        length: args.length,
        numbers: args.no_numbers,
        lowercase_letters: args.no_lowercase_letters,
        uppercase_letters: args.no_uppercase_letters,
        symbols: args.no_symbols,
        spaces: args.spaces,
        exclude_similar_characters: args.include_similar_chars,
        strict: false,
    };
    println!("{}", pg.generate_one().unwrap());
}
