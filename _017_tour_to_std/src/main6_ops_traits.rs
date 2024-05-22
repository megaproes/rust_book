use std::fmt;
use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    // this is called an associated type—a type that “goes together” with a trait. In this case, it’s another Point.
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}
impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}
impl Add for Country {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, other.name),
            population: self.population + other.population,
            gdp: self.gdp + other.gdp,
        }
    }
}
impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

fn main() {
    let nauru = Country::new("Nauru", 12_511, 133_200_000);
    let vanuatu = Country::new("Vanuatu", 219_137, 956_300_000);
    let micronesia = Country::new("Micronesia", 113_131, 404_000_000);
    println!("{}", nauru);
    let nauru_and_vanuatu = nauru + vanuatu;
    println!("{nauru_and_vanuatu}");
    println!("{}", nauru_and_vanuatu + micronesia);
}
