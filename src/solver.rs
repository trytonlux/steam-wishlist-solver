#[derive(PartialEq, Debug)]
// Options for sorting wishlist
pub enum SortStrategy {
    Cheapest,
    Expensive,
}

pub struct Cart {
    pub items: Vec<f32>,
    pub total_cost: f32,
}

// Return maximum list of items that fit within given budget.
// Supports multiple methods for sorting the list before greedily pulling items.
pub fn grab_max_items(wishlist: &[f32], budget: f32, strategy: SortStrategy) -> Cart {
    let mut sorted_wishlist = wishlist.to_vec();
    sorted_wishlist.sort_by(f32::total_cmp);

    if strategy == SortStrategy::Expensive {
        sorted_wishlist.reverse();
    }

    let mut total_cost = 0_f32;
    let mut cart = Vec::<f32>::new();

    for cost in sorted_wishlist {
        if total_cost + cost <= budget {
            total_cost += cost;
            cart.push(cost);
        } else {
            break;
        }
    }

    Cart {
        items: cart,
        total_cost,
    }
}
