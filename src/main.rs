mod solver;

fn main() {
    let wishlist = vec![
        11.49, 27.99, 13.09, 27.99, 21.99, 14.99, 29.99, 10.99, 12.99, 26.99, 26.99, 8.79, 21.99,
        21.99, 8.99, 12.99, 38.99, 28.99, 21.99, 8.99, 25.99, 16.99, 17.49, 16.99, 36.06, 8.99,
        4.87, 34.99, 3.99, 4.24, 19.99, 21.99, 13.19, 9.99, 25.99, 25.99, 25.99, 12.76, 24.99,
        22.79, 19.49, 10.99, 28.99, 25.99, 10.80, 25.99, 10.99, 10.99, 22.79, 21.99, 21.99, 14.24,
        22.79, 32.99, 32.99, 9.99, 7.79, 25.99, 21.99, 21.99, 25.99, 14.99, 79.99, 21.99, 22.79,
        25.98, 23.99,
    ];
    let budget = 100_f64;

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
