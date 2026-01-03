struct SeaCreature {
  animal_type: String,
  name: String,
  arms: i32,
  legs: i32,
  weapon: String,
}

fn main() {
  let ferris = SeaCreature {
    animal_type: String::from("crab"),
    name: String::from("Ferris"),
    arms: 2,
    legs: 4,
    weapon: String::from("claw"),
  };

  println!(
    "{} is a {}. They have {}, arms, {} legs and their weapon is {}",
    ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon);
}
