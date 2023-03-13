# Rust Course

## [Lucians Luscious Lasagna](./lucians-luscious-lasagna)
```rust
#![allow(unused)]

mod calc_i32 {
    fn add(a: i32, b: i32) -> i32 { a + b }
    fn sub(a: i32, b: i32) -> i32 { a - b }
    fn mul(a: i32, b: i32) -> i32 { a * b }
    fn div(a: i32, b: i32) -> i32 { a / b }
}

pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {    
     expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
```

## [Assembly Line](./assembly-line/)
```rust
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = if speed <= 4 {
        1.0
    } else if speed <= 8 {
        0.9
    } else {
        0.77
    };
    let production_rate = speed as f64 * 221.0 * success_rate;
    
    production_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production_rate = production_rate_per_hour(speed) / 60.0;
    let working_items = production_rate.floor() as u32;

    working_items
}
```


## [Health Statistics](./health-statistics/)
```rust
#![allow(unused)]

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User { name, age, weight }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}
```
## [Poker](./poker)
```rust
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: BinaryHeap<_> = hands.iter().map(|&s| (PokerHand::parse(s), s)).collect();
    let (winning, s) = hands.pop().unwrap();
    let mut result = vec![s];
    while let Some((value, s)) = hands.pop() {
        if value < winning {
            break;
        }
        result.push(s);
    }
    result
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PokerHand {
    counts: Vec<usize>,
    values: Vec<u8>,
}
fn parse_card(s: &str) -> (u8, u8) {
    let (value, suit) = s.split_at(s.len() - 1);
    (
        match value.parse::<u8>() {
            Ok(v) => v,
            Err(_) => "JQKA".find(value).unwrap() as u8 + 11,
        },
        suit.as_bytes()[0],
    )
}

impl PokerHand {
    fn parse(s: &str) -> Self {
        let (values, suits): (Vec<u8>, Vec<u8>) = s.split_whitespace().map(parse_card).unzip();
        let mut groups = HashMap::<u8, usize>::new();
        for &v in values.iter() {
            *groups.entry(v).or_default() += 1;
        }
        let mut groups: Vec<_> = groups.into_iter().map(|(v, c)| (c, v)).collect();
        groups.sort_unstable_by_key(|&x| Reverse(x));
        let (mut counts, mut values): (Vec<_>, Vec<_>) = groups.iter().copied().unzip();
        if counts.len() == 5 {
            if values == [14, 5, 4, 3, 2] {
                values = vec![5, 4, 3, 2, 1];
            }
            let is_straight = values[0] - values[4] == 4;
            let is_flush = suits[1..].iter().all(|&x| x == suits[0]);
            match (is_straight, is_flush) {
                (true, true) => counts = vec![5],
                (true, false) => counts = vec![3, 1, 2],
                (false, true) => counts = vec![3, 1, 3],
                _ => {}
            }
        }
        Self { counts, values }
    }
}
```

## [Resistor Color](./resistor-color)
```rust
#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}


pub fn color_to_value(color: ResistorColor) -> u32 {
    match color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9,
    }
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from(""),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors = Vec::new();
    colors.push(ResistorColor::Black);
    colors.push(ResistorColor::Brown);
    colors.push(ResistorColor::Red);
    colors.push(ResistorColor::Orange);
    colors.push(ResistorColor::Yellow);
    colors.push(ResistorColor::Green);
    colors.push(ResistorColor::Blue);
    colors.push(ResistorColor::Violet);
    colors.push(ResistorColor::Grey);
    colors.push(ResistorColor::White);
    colors
}
```

## [Reverse String](./reverse-string)
```rust
pub fn reverse(input &str) -> String {
  let mut reverse = String::new();

  for character in input.chars() {
    reverse.insert(0, character);
  }

  reverse
}
```

## [Semi Structured Logs](./semi-structured-logs)
```rust
#![allow(unused)]

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}

pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message).to_string()
}

pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message).to_string()
}

pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message).to_string()
}
```

## [Sublist](./sublist)
```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.is_empty() || b.windows(a.len()).any(|x| x == a)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
```

## [Gigasecond](./gigasecond)
```rust
use time::{Duration, PrimitiveDateTime};

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    // Add one gigasecond (1,000,000,000 seconds) to the input date and time
    let one_gigasecond = Duration::seconds(1_000_000_000);
    
    start + one_gigasecond
}
```

## [Clock](./clock)

```rust
use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes) % (24 * 60);
        let (hours, minutes) = (total_minutes / 60, total_minutes % 60);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
```