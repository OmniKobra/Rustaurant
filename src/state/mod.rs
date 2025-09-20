use std::time::Duration;

use super::*;
use coin_flip::*;
use rand::prelude::*;
use tokio::time::sleep;

pub struct State {
    current_val: u8, // percent
    calendar: Calendar,
    is_paused: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_val: 100,
            calendar: Calendar::default(),
            is_paused: false,
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn advance(&mut self) {
        if !self.is_paused() {
            self.calendar.advance(600);
            self.fluctuate();
        }
        let start = self.calendar.start_seconds();
        let rn = self.calendar.time();
        // checks if the current month is over and await continue
        if rn.timestamp() == start + (86400 * 30) {
            self.pause();
            println!("ONE MONTH HAS PASSED");
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    fn fluctuate(&mut self) {
        let flip = flip_coin(&mut |_| {});
        println!("Before fluctuation: {} &", self.current_val);
        self.current_val = {
            let mut rng = rand::rng();
            // max should always be between 1 and 10 due to reason explained below
            let val: u8 = rng.random_range(0..=10); // max should be 10 due to cascade depreciation in 1 year sim going below zero
            if flip == Coin::Heads {
                // anything above 12 would overflow the max u8 as well in case of cascade appreciation.
                println!("The currency has depreciated by {val}%");
                // depreciation
                self.current_val + val
            } else {
                println!("The currency has appreciated by {val}%");
                // appreciation
                self.current_val - val
            }
        };
        println!("After fluctuation: {} %\n", self.current_val);
    }

    pub fn value(&self) -> u8 {
        self.current_val
    }

    pub fn calendar(&self) -> &Calendar {
        &self.calendar
    }
}
