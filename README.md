# Rust Course

- [Assembly Line](./assembly-line/)

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


- [Health Statistics](./health-statistics/)
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
- [Poker](./poker)
- [Resistor Color](./resistor-color)
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
- [Reverse String](./reverse-string)
```rust
pub fn reverse(input &str) -> String {
  let mut reverse = String::new();

  for character in input.chars() {
    reverse.insert(0, character);
  }

  reverse
}
```
- [Semi Structured Logs](./semi-structured-logs)
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
- [Sublist](./sublist)
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
- [Gigasecond](./gigasecond)
```rust
use time::{Duration, PrimitiveDateTime};

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    // Add one gigasecond (1,000,000,000 seconds) to the input date and time
    let one_gigasecond = Duration::seconds(1_000_000_000);
    
    start + one_gigasecond
}
```