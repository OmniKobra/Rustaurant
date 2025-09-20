mod calendar;
mod customers;
mod menu;
mod orders;
mod resto;
mod state;

pub use calendar::*;
pub use customers::*;
pub use menu::*;
pub use orders::*;
pub use resto::*;
pub use state::*;

#[cfg(test)]
mod tests {
    use core::borrow;
    use std::{sync::Arc, time::Duration};

    use tokio::{join, sync::Mutex, time::sleep};

    use super::*;
    #[test]
    pub fn run_sim() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Could not create runtime");
        rt.block_on(async {
            let dummy_state0 = Arc::new(Mutex::new(State::new()));
            let dummy_state1 = Arc::clone(&dummy_state0);
            let t1 = tokio::spawn(async move {
                loop {
                    {
                        let mut state = dummy_state1.lock().await;
                        state.advance();
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            });
            let t2 = tokio::spawn(async move {
                let mut dummy_menu: Vec<AnyEdible> = vec![
                    edible("Dummy Meal - A dummy meal.", Category::Meal, 35),
                    edible("Dummy Drink - A dummy drink.", Category::Drink, 9),
                    edible("Dummy Dessert - A dummy dessert.", Category::Dessert, 12),
                ];
                loop {
                    {
                        let state = dummy_state0.lock().await;
                        for e in &mut dummy_menu {
                            e.extract().0.update_price(state.value());
                            println!("{e}");
                        }
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            });
            join!(t1, t2);
        });
    }
}
