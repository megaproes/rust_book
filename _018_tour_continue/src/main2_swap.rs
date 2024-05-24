use std::mem;
#[derive(Debug)]
struct Ring {
    owner: String,
    former_owners: Vec<String>,
}
impl Ring {
    fn switch_owner_to(&mut self, name: &str) {
        if let Some(position) = self.former_owners.iter().position(|n| n == name) {
            mem::swap(&mut self.owner, &mut self.former_owners[position])
        } else {
            println!("Nobody named {name} found in former_owners, sorry!");
        }
    }
}
fn main() {
    let mut one_ring = Ring {
        owner: "Frodo".into(),
        former_owners: vec!["Gollum".into(), "Sauron".into()],
    };
    println!("Original state: {one_ring:?}");
    one_ring.switch_owner_to("Gollum");
    println!("{one_ring:?}");
    
    one_ring.switch_owner_to("Sauron");
    println!("{one_ring:?}");
    one_ring.switch_owner_to("Billy");
    println!("{one_ring:?}");
}
