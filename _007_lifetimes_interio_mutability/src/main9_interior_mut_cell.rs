use std::cell::Cell;
#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>, // Cell works for all types, but it works best for simple Copy types because it gives values, not references
}

struct some{
	sad: Cell<String>,
}
impl PhoneModel {
    fn make_not_on_sale(&self) {
        self.on_sale.set(false);
    }
}

fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };
    super_phone_3000.make_not_on_sale();
    println!("{super_phone_3000:#?}");
    
    let mut some1 = some{sad: Cell::new("value".to_string())};
    println!("{}", some1.sad.get_mut())
}
