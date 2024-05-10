struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}
impl<'a> Adventurer<'a> {
    fn take_damage1(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}
impl Adventurer<'_> // same as above
{
    fn take_damage2(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

fn main() {}
