struct Company {
    name: String,
    ceo: Option<String>,
}
fn get_current_datetime() -> String {
    "2024-01-27T23:11:23".to_string()
}
impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            name => Some(name.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}
fn main() {
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let results: Vec<Result<String, String>> = company_vec
        .iter()
        .map(|company| {
            company.get_ceo().ok_or_else(|| {
                let err_message = format!("No CEO found for {}", company.name);
                println!("{err_message} at {}", get_current_datetime());
                err_message
            })
        })
        .collect();
	
    results
        .iter()
        .filter(|res| res.is_ok())
        .for_each(|res| println!("{res:?}"));
}
