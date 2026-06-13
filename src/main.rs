use std::path::Path;

use crate::solver::SortStrategy;

mod solver;
mod wishlist;

fn main() {
    let wishlist = wishlist::from_file(Path::new("./wishlist.json"));
    let budget = 100_f32;

    let cheapest = solver::grab_max_items(&wishlist, budget, SortStrategy::Cheapest, false);
    let cheapest_only_discounts =
        solver::grab_max_items(&wishlist, budget, SortStrategy::Cheapest, true);
    let expensive = solver::grab_max_items(&wishlist, budget, SortStrategy::Expensive, false);
    let expensive_only_discounts =
        solver::grab_max_items(&wishlist, budget, SortStrategy::Expensive, true);

    println!("{cheapest}\n");
    println!("{cheapest_only_discounts}\n");
    println!("{expensive}\n");
    println!("{expensive_only_discounts}");
}
