use front_area::hosting;
mod front_area;

mod back_of_house;

pub fn deliver_order() {}

fn main() {
    let _x = hosting::add_to_waitlist();

    front_area::eat_at_restaurant();

    println!("Hello, world!");
}
