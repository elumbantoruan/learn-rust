use crate::garden::fruits::Oranges;
use crate::garden::vegetables::Asparagus;
use crate::pond::fish::Fish;

pub mod garden;
pub mod pond;

fn main() {
    let plan = Asparagus{};
    println!("I'm growing {:?}", plan);

    let fruit = Oranges{};
    println!("I'm also growing {:?}", fruit);

    let fish = Fish{};
    println!("I also have {:?} in my backyard", fish)
}
