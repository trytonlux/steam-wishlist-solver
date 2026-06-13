use std::fmt::Display;

use tabled::{
    Table,
    settings::{
        Color, Panel, Style,
        object::Rows,
        themes::{BorderCorrection, Colorization},
    },
};

use crate::wishlist::{Game, GameList};

#[derive(PartialEq, Debug)]
// Options for sorting wishlist
pub enum SortStrategy {
    Cheapest,
    Expensive,
}

pub struct Cart {
    items: Vec<Game>,
    total_cost: f32,
    strategy: SortStrategy,
    only_discounts: bool,
}

impl Display for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = match self.strategy {
            SortStrategy::Cheapest => "Sorted by Lowest Price",
            SortStrategy::Expensive => "Sorted by Highest Price",
        };

        let header = match self.only_discounts {
            true => format!("{} (Only Discounts)", header),
            false => header.to_string(),
        };

        let mut table = Table::new(&self.items);
        table
            .with(Style::modern())
            .with(Colorization::exact([Color::BOLD], Rows::first()))
            .with(Panel::header(header))
            .with(Panel::footer(format!("Total Cost: {}", &self.total_cost)))
            .with(BorderCorrection::span());
        write!(f, "{table}")
    }
}

// Return maximum list of items that fit within given budget.
// Supports multiple methods for sorting the list before greedily pulling items.
pub fn grab_max_items(
    wishlist: &GameList,
    budget: f32,
    strategy: SortStrategy,
    only_discounts: bool,
) -> Cart {
    let mut sorted_wishlist = match only_discounts {
        true => wishlist
            .iter()
            .filter(|game| game.discount != 0)
            .cloned()
            .collect(),
        false => wishlist.clone(),
    };

    sorted_wishlist.sort_by(|a, b| a.price.total_cmp(&b.price));

    if strategy == SortStrategy::Expensive {
        sorted_wishlist.reverse();
    }

    let mut total_cost = 0_f32;
    let mut cart = GameList::new();

    for game in sorted_wishlist {
        let cost = game.price;
        if total_cost + cost <= budget {
            total_cost += cost;
            cart.push(game.clone());
        } else {
            break;
        }
    }

    Cart {
        items: cart,
        total_cost,
        strategy,
        only_discounts
    }
}
