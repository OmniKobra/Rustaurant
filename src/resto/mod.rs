use super::{AnyEdible, Category, Customer, Edible, Order, OrderStatus};
use std::{collections::HashMap, time::Duration};
pub struct Rustaurant<'a> {
    // full_menu: &'a Vec<AnyEdible>,
    meal_menu: Vec<&'a AnyEdible>,
    drink_menu: Vec<&'a AnyEdible>,
    dessert_menu: Vec<&'a AnyEdible>,
    orders: HashMap<u32, Order<'a>>,
    customer_count: u32,
    customers: Vec<Customer<'a>>,
    order_count: u32,
    revenue: u32,
    expenses: u32,
}

impl<'a> Rustaurant<'a> {
    pub fn new(full_menu: &'a Vec<AnyEdible>) -> Self {
        Self {
            // full_menu,
            meal_menu: filter_menu(full_menu, Category::Meal),
            drink_menu: filter_menu(full_menu, Category::Drink),
            dessert_menu: filter_menu(full_menu, Category::Dessert),
            orders: HashMap::new(),
            customer_count: 0,
            order_count: 0,
            revenue: 0,
            expenses: 0,
            customers: vec![],
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

    pub fn get_edible(&self, name: &str, category: Category) -> &dyn Edible {
        self.find_edible(name, category).extract().0
    }

    pub fn receive_customer(&mut self) -> Customer<'a> {
        self.customer_count += 1;
        Customer {
            count: self.customer_count,
            order: None,
        }
    }

    pub fn take_order(&mut self, name: &'a str, customer: u32, category: Category) {
        self.order_count += 1;
        let count: u32 = self.order_count;
        self.orders.insert(
            customer,
            Order {
                name,
                count,
                customer,
                category,
                status: OrderStatus::Idle,
                ..Default::default()
            },
        );
    }

    pub fn take_payment(&mut self) {}

    pub fn net_revenue(&self) -> i32 {
        self.revenue as i32 - self.expenses as i32
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
