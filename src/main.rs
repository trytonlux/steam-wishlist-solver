#[derive(PartialEq, Debug)]
// Options for sorting wishlist
enum WishlistSortStrategies {
    Cheapest,
    Expensive,
}

// Finds the most amount of items that fit in a given budget by simply just itterating the list.
// Supports multiple methods for sorting the list before greedily pulling items.
fn max_items_greedy(wishlist: &[f64], budget: f64, strategy: WishlistSortStrategies) {
    let mut sorted_wishlist = wishlist.to_vec();
    sorted_wishlist.sort_by(f64::total_cmp);

    if strategy == WishlistSortStrategies::Expensive {
        sorted_wishlist.reverse();
    }

    let mut total_cost = 0_f64;
    let mut cart = Vec::<f64>::new();

    for cost in sorted_wishlist {
        if total_cost + cost <= budget {
            total_cost += cost;
            cart.push(cost);
        } else {
            break;
        }
    }

    println!("Wishlist sorted by {:#?}", strategy);
    println!("{} items bought for {}", cart.len(), total_cost);
    println!("Full cart: {:#?}", cart);
}

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

    max_items_greedy(&wishlist, budget, WishlistSortStrategies::Cheapest);
    max_items_greedy(&wishlist, budget, WishlistSortStrategies::Expensive);
}
