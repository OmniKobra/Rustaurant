use rand::Rng;

use super::{AnyEdible, Category, Customer, Edible, Order, OrderStatus};
use std::collections::HashMap;

struct Account {
    balance: u32,
    revenue: u32,
    expenses: u32,
    monthly_expenses: u32,
    pub order_count: u32,
    pub customer_count: u32,
}

impl Account {
    pub fn new() -> Self {
        Self {
            balance: 0,
            revenue: 0,
            expenses: 0,
            monthly_expenses: 10_000,
            customer_count: 0,
            order_count: 0,
        }
    }
}

pub struct Rustaurant<'a> {
    meal_menu: Vec<&'a AnyEdible>,
    drink_menu: Vec<&'a AnyEdible>,
    dessert_menu: Vec<&'a AnyEdible>,
    orders: HashMap<u32, Order<'a>>,
    customers: Vec<Customer>,
    account: Account,
}

impl<'a> Rustaurant<'a> {
    pub fn new(full_menu: &'a Vec<AnyEdible>) -> Self {
        Self {
            meal_menu: filter_menu(full_menu, Category::Meal),
            drink_menu: filter_menu(full_menu, Category::Drink),
            dessert_menu: filter_menu(full_menu, Category::Dessert),
            orders: HashMap::new(),
            customers: Vec::new(),
            account: Account::new(),
        }
    }

    pub fn find_edible(&'a self, name: &str, category: Category) -> &'a AnyEdible {
        let finder = |list: &'a Vec<&AnyEdible>| -> &'a AnyEdible {
            &list.iter().find(|f| f.extract().0.name() == name).unwrap()
        };
        match category {
            Category::Meal => finder(&self.meal_menu),
            Category::Drink => finder(&self.drink_menu),
            Category::Dessert => finder(&self.dessert_menu),
        }
    }

    fn edible(&self, name: &str, category: Category) -> &dyn Edible {
        self.find_edible(name, category).extract().0
    }

    pub fn receive_customer(&mut self) {
        self.account.customer_count += 1;
        let mut c = Customer {
            count: self.account.customer_count,
            balance: rand::rng().random_range(50..=120),
            orders: [None; 3],
        };
        c.place_orders(self);
        self.customers.push(c);
    }

    pub fn take_order(&mut self, name: &'a str, customer: u32, category: Category) -> &Order<'a> {
        self.account.order_count += 1;
        let count: u32 = self.account.order_count;
        self.orders.insert(
            count,
            Order {
                name,
                count,
                customer,
                category,
                status: OrderStatus::Idle,
                ..Default::default()
            },
        );
        self.account.expenses += (self.edible(name, category).current_price() / 2) as u32;
        &self.orders.get(&count).unwrap()
    }
    pub fn get_order(&self, count: u32) -> &Order<'a> {
        &self.orders.get(&count).unwrap()
    }

    pub fn get_edible(&self, order_count: u32) -> &dyn Edible {
        let Order { name, category, .. } = *self.get_order(order_count);
        self.edible(name, category)
    }

    pub fn take_payment(&mut self, amount: u32) {
        self.account.revenue += amount;
    }

    pub fn net_revenue(&self) -> i32 {
        self.account.revenue as i32 - self.account.expenses as i32
    }

    pub fn meal_menu(&self) -> &Vec<&'a AnyEdible> {
        &self.meal_menu
    }

    pub fn drink_menu(&self) -> &Vec<&'a AnyEdible> {
        &self.drink_menu
    }

    pub fn dessert_menu(&self) -> &Vec<&'a AnyEdible> {
        &self.dessert_menu
    }
}

pub fn filter_menu(full_menu: &Vec<AnyEdible>, category: Category) -> Vec<&AnyEdible> {
    full_menu
        .iter()
        .filter(|e| match e {
            AnyEdible::AnyMeal(_) => category == Category::Meal,
            AnyEdible::AnyDrink(_) => category == Category::Drink,
            AnyEdible::AnyDessert(_) => category == Category::Dessert,
        })
        .collect::<Vec<&AnyEdible>>()
}
