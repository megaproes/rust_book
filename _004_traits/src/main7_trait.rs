use std::fmt::Debug;
trait MonsterBehavior: Debug {
    fn take_damage(&mut self, damage: i32);
    fn display_self(&self) {
        println!("The monster is now: {self:?}");
    }
}
#[derive(Debug)]
struct Monster {
    health: i32,
}
impl MonsterBehavior for Monster {
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}
#[derive(Debug)]
struct Wizard {
    health: i32,
}
#[derive(Debug)]
struct Ranger {
    health: i32,
}
trait FightClose {
    fn attack_with_sword<T: MonsterBehavior>(&self, opponent: &mut T) {
        println!("You attack with your sword!");
        opponent.take_damage(10);
        opponent.display_self();
    }
    fn attack_with_hand<T: MonsterBehavior>(&self, opponent: &mut T) {
        println!("You attack with your hand!");
        opponent.take_damage(2);
        opponent.display_self();
    }
}
impl FightClose for Wizard {}
impl FightClose for Ranger {}
trait FightFromDistance: Debug {
    fn attack_with_bow<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
        println!("You attack with your bow!");
        if distance < 10 {
            opponent.take_damage(10);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }
    fn attack_with_rock<T: MonsterBehavior>(&self, opponent: &mut T, distance: u32) {
        println!("You attack with a rock!");
        if distance < 3 {
            opponent.take_damage(4);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }
}
impl FightFromDistance for Ranger {}
fn main() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };
    let mut uruk_hai = Monster { health: 40 };
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);
}
