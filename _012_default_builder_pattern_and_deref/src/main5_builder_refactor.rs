#![allow(dead_code)]

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
    can_use: bool,
}
#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
            can_use: true,
        }
    }
}
impl Character {
    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self.can_use = false;
        self
    }
    fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self.can_use = false;
        self
    }
    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.can_use = false;
        self
    }
    fn build(mut self) -> Result<Character, String> {
        if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
            self.can_use = true;
            Result::Ok(self)
        } else {
		Result::Err("Could not create character. Characters must have:
	1) Height below 200
	2) Weight below 300
	3) A name that is not Smurf (that is a bad word)"
                .to_string())
        }
    }
}
fn main() {
    let character_with_smurf = Character::default().name("Lol I am Smurf!!").build();
    let character_too_tall = Character::default().height(400).build();
    let character_too_heavy = Character::default().weight(500).build();
    let okay_character = Character::default()
        .name("Billybrobby")
        .height(180)
        .weight(100)
        .build();
    let character_vec = vec![
        character_with_smurf,
        character_too_tall,
        character_too_heavy,
        okay_character,
    ];
    for character in character_vec {
        match character {
            Ok(character) => println!("{character:?}\n"),
            Err(err_info) => println!("{err_info}\n"),
        }
    }
}
