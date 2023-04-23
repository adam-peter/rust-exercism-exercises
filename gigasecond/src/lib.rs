use time::{ext::NumericalDuration, PrimitiveDateTime};
use time_macros::{date, time};

// Returns a DateTime one billion seconds after start.

fn main() {
    let starting_date = PrimitiveDateTime::new(date!(2015 - 1 - 24), time!(22:00));
    let after_date = after(starting_date);
    println!("{}", after_date);
}

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let gigasecond = 1_000_000_000;
    start.saturating_add((gigasecond).seconds())
}
