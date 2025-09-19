mod customers;
mod menu;
mod orders;
mod resto;
mod state;
mod calendar;

pub use customers::*;
pub use menu::*;
pub use orders::*;
pub use resto::*;




#[cfg(test)]
mod tests {
    #[test]
    fn run_sim() {}
}
