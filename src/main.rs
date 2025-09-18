use rustaurant::{AnyEdible, Category::*, Rustaurant, edible};
use tokio::{
    join,
    time::{Duration, Sleep, sleep},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let full_menu: Vec<AnyEdible> = vec![
        // MEALS
        edible(
            "Cheeseburger - A beef patty with cheese, lettuce, and tomato.",
            Meal,
            35,
        ),
        edible("Club Sandwich - A club sandwich.", Meal, 15),
        edible("Soup - A warm liquid with spices and noodles.", Meal, 20),
        //DRINKS
        edible("Cola - A refreshing caramel flavoured drink.", Drink, 7),
        edible("Lemonade - Freshly squeezed sour fruit water", Drink, 9),
        edible("Water - Human sustainability liquid.", Drink, 2),
        // DESSERTS
        edible("Ice cream - Frozen sugar ", Dessert, 12),
        edible("Fondant - A chocolate cake in French", Dessert, 15),
        edible(
            "Cotton Candy - Furry sugar (Available in white, green, blue, and pink).",
            Dessert,
            6,
        ),
    ];
    let my_rustaurant = Rustaurant::new(&full_menu);
}
