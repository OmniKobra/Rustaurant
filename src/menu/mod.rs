use std::ops::{Deref, DerefMut};

#[repr(u8)]
#[derive(PartialEq)]
/// Categorizes Edibles into their Categories
pub enum Category {
    Meal = 0,
    Drink = 1,
    Dessert = 2,
}

/// Not essential but good to have
impl TryFrom<u8> for Category {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Category::*;
        match value {
            0 => Ok(Meal),
            1 => Ok(Drink),
            2 => Ok(Dessert),
            _ => Err(()),
        }
    }
}
/// Trait that defines common behaviour between Meals, Drinks, Desserts
pub trait Edible {
    fn name_description(&self) -> &str;
    fn name(&self) -> &str {
        self.splitter().0
    }
    fn description(&self) -> &str {
        self.splitter().1
    }
    fn splitter(&self) -> (&str, &str) {
        self.name_description().split_once(" - ").unwrap()
    }
    fn price(&self) -> u16;
    fn current_price(&self) -> u16;
    fn update_price(&mut self, value: u8);
}
/// Abstract data struct with common fields between any Edible
pub struct EdibleData {
    name_description: &'static str,
    price: u16,
    current_price: u16,
}

impl EdibleData {
    pub fn new(name_description: &'static str, price: u16) -> Self {
        Self {
            name_description,
            price,
            current_price: price,
        }
    }
}

impl Edible for EdibleData {
    fn name_description(&self) -> &str {
        self.name_description
    }
    fn price(&self) -> u16 {
        self.price
    }
    fn current_price(&self) -> u16 {
        self.current_price
    }
    fn update_price(&mut self, value: u8) {
        self.current_price = (self.price as u32 * value as u32 / 100) as u16;
    }
}

/// Wraps Edible structs and categorizes them into Category variants through const Generic
pub struct EdibleItem<T: Edible, const C: u8>(pub T);

impl<T: Edible, const C: u8> EdibleItem<T, C> {
    /// Allows extraction of category of an EdibleItem
    pub fn category() -> Category {
        match C {
            0 => Category::Meal,
            1 => Category::Drink,
            2 => Category::Dessert,
            _ => panic!("invalid category"),
        }
    }
}

impl<T: Edible, const C: u8> Deref for EdibleItem<T, C> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Edible, const C: u8> DerefMut for EdibleItem<T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Edible, const C: u8> std::fmt::Display for EdibleItem<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\nUsual Price: {}\nCurrent Price: {}",
            self.name(),
            self.description(),
            self.price(),
            self.current_price()
        )
    }
}

/// Custom types for corresponding categories
type MealItem = EdibleItem<EdibleData, { Category::Meal as u8 }>;
type DrinkItem = EdibleItem<EdibleData, { Category::Drink as u8 }>;
type DessertItem = EdibleItem<EdibleData, { Category::Dessert as u8 }>;

/// Allows Heterogenous collections and data extraction pattern matching based on category
pub enum AnyEdible {
    AnyMeal(MealItem),
    AnyDrink(DrinkItem),
    AnyDessert(DessertItem),
}

/// Generates AnyEdible variant corrresponding to its category
/// reduces verbosity of instantiating an edible item
pub fn edible(name_description: &'static str, category: Category, price: u16) -> AnyEdible {
    let data = EdibleData::new(name_description, price);
    match category {
        Category::Meal => AnyEdible::AnyMeal(EdibleItem::<EdibleData, 0>(data)),
        Category::Drink => AnyEdible::AnyDrink(EdibleItem::<EdibleData, 1>(data)),
        Category::Dessert => AnyEdible::AnyDessert(EdibleItem::<EdibleData, 2>(data)),
    }
}
