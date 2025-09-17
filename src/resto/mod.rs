use super::{AnyEdible, Category};
pub struct Rustaurant<'a> {
    full_menu: &'a Vec<AnyEdible>,
    meal_menu: Vec<&'a AnyEdible>,
    drink_menu: Vec<&'a AnyEdible>,
    dessert_menu: Vec<&'a AnyEdible>,
}

impl<'a> Rustaurant<'a> {
    pub fn new(full_menu: &'a Vec<AnyEdible>) -> Self {
        Self {
            full_menu,
            meal_menu: filter_menu(full_menu, Category::Meal),
            drink_menu: filter_menu(full_menu, Category::Drink),
            dessert_menu: filter_menu(full_menu, Category::Dessert),
        }
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
