// As the name  implies, this function outright takes the value from something and returns it. But
// take() doesn’t drop the existing variable; instead, it leaves its default value in its
// place. And that’s why this function requires the type we are take()-ing from to imple-
// ment Default

use std::mem;
fn main() {
    let mut number_vec = vec![8, 7, 0, 2, 49, 9999];
    let mut new_vec = vec![];

    number_vec.iter_mut().for_each(|number| {
        let taker = mem::take(number);
        new_vec.push(taker);
    });

    println!("{:?}\n{:?}", number_vec, new_vec);
    
    // or we can specify default impl
    let mut bank_of_klezkavania = Bank {
        money_inside: 5000,
        money_at_desk: DeskMoney(500),
    };
    
    let money_stolen = mem::take(&mut bank_of_klezkavania.money_at_desk);
    println!("\n\nStole {} Klezkavanian credits", money_stolen.0);
    println!("{bank_of_klezkavania:?}");
}

#[derive(Debug)]
struct Bank {
    money_inside: u32,
    money_at_desk: DeskMoney,
}
#[derive(Debug)]
struct DeskMoney(u32);
impl Default for DeskMoney {
    fn default() -> Self {
        Self(50)
    }
}
