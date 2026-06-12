use std::path::Path;

mod solver;
mod wishlist;

fn main() {
    let wishlist = wishlist::from_file(Path::new("./wishlist.json"));
    let budget = 100_f32;

    println!("Sorted by lowest price");
    let cheapest = solver::grab_max_items(&wishlist, budget, solver::SortStrategy::Cheapest);

    println!("{} items for {}", cheapest.items.len(), cheapest.total_cost);
    println!("Full cart: {:#?}", cheapest.items);
    println!();

    println!("Sorted by highest price");
    let expensive = solver::grab_max_items(&wishlist, budget, solver::SortStrategy::Expensive);

    println!("{} items for {}", expensive.items.len(), expensive.total_cost);
    println!("Full cart: {:#?}", expensive.items);
    println!();
}
