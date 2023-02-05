use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Zijia Jiang", about = "A calculator")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Zijia Jiang")]
    Calculate { operator: char, a: i32, b: i32 },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Calculate { operator, a, b }) => {
            println!(
                "The result is: {} {} {} = {}",
                a,
                operator,
                b,
                mini3::calculate(a, b, operator)
            );
        }
        None => println!("No command given"),
    }
}
