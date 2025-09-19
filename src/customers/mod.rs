use super::Rustaurant;
use coin_flip::{Coin, flip_coin};
use rand::prelude::*;

pub struct Customer {
    pub count: u32,
    pub balance: u16,
    pub orders: [Option<u32>; 3],
}

impl Customer {
    pub fn place_orders(&mut self, resto: &mut Rustaurant) {
        let mut rng = rand::rng();
        for i in 0..3 {
            self.handle_order(i, resto, &mut rng);
        }
    }

    fn handle_order(&mut self, i: usize, resto: &mut Rustaurant, rng: &mut ThreadRng) {
        let wants = Self::wants_edible();
        let menu = match i {
            0 => resto.meal_menu(),
            1 => resto.drink_menu(),
            _ => resto.dessert_menu(),
        };
        if !wants || menu.is_empty() {
            return;
        }

        let e = rng.random_range(0..menu.len());
        let (edible, category) = menu[e].extract();
        let order = resto.take_order(edible.name(), self.count, category);
        self.orders[i] = Some(order.count);
    }

    pub fn pay_check(&mut self, resto: &mut Rustaurant) {
        let total = {
            let mut sum = 0;
            for o in self.orders {
                if let Some(count) = o {
                    sum += resto.get_edible(count).current_price();
                }
            }
            sum
        };
    }

    fn wants_edible() -> bool {
        flip_coin(&mut |_| {}) == Coin::Heads
    }
}
