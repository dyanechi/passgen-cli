use expanders::enum_with_props;

enum_with_props! {
    pub Animal {
        A(String::from("Dog"), 3usize),
        B(String::from("Cat"), 7usize),
    }
}

fn main() {
    let animal_a = Animal::A;
    let animal_b = Animal::B;

    println!("Animal A: {:?}", animal_a.props());
    println!("Animal B: {:?}", animal_b.props());
}