use std::path::PathBuf;

use crate::solver::SortStrategy;

use clap::{Command, CommandFactory, Parser};
use clap_complete::{Shell, generate};

mod solver;
mod wishlist;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Generate shell completions. Default to current shell
    #[arg(long)]
    #[clap(value_enum)]
    completions: Option<Shell>,
    /// Path to JSON wishlist file to load
    #[clap(default_value = "./wishlist.json")]
    wishlist_file: PathBuf,
    /// Spending limit for games
    #[arg(long, short)]
    #[clap(default_value_t = 100_f32)]
    budget: f32,
    /// How to sort the wishlist before adding games to cart
    #[arg(long, short)]
    #[clap(value_enum)]
    #[clap(default_value_t = SortStrategy::Cheapest)]
    sorting: SortStrategy,
    /// Whether to include games not discounted
    #[arg(long, short)]
    include_non_discounted: bool,
}

fn generate_completions(shell: Shell, cmd: &mut Command) {
    generate(
        shell,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    )
}

fn main() {
    let args = Cli::parse();

    if let Some(shell) = args.completions {
        generate_completions(shell, &mut Cli::command());
    }

    let wishlist = wishlist::from_file(&args.wishlist_file);
    let budget = args.budget;
    let sort_strategy = args.sorting;
    let only_discounts = !args.include_non_discounted;

    let cart = solver::grab_max_items(&wishlist, budget, sort_strategy, only_discounts);
    println!("{cart}");
}
