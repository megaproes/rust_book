struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: bool,
}
impl PhoneModel {
    /// Returns the method one of this [`PhoneModel`].
    fn method_one(&self) {}
    fn method_two(&self) {}
}
fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: true,
    }; // what if i want to change only some of the data like on_sale? 
    // if i do super_phone_3000 as 'mut' then i will change all the data inside, but i want only some
}
