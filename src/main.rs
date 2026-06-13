use std::path::Path;

mod solver;
mod wishlist;

fn main() {
    let wishlist = wishlist::from_file(Path::new("./wishlist.json"));
    let budget = 100_f32;


    let cheapest = solver::grab_max_items(&wishlist, budget, solver::SortStrategy::Cheapest);
    let expensive = solver::grab_max_items(&wishlist, budget, solver::SortStrategy::Expensive);

    println!("{cheapest}\n");
    println!("{expensive}");
}
