use std::marker::PhantomData;

// Marker type parameter pattern
struct Cat<Breed> {
    name: String,
    breed: PhantomData<Breed>,
}

struct Aegean {}
impl Cat<Aegean> {
    fn breed_name(&self) -> &str {
        "Aegan"
    }
}
struct Bengal {}
impl Cat<Bengal> {
    fn breed_name(&self) -> &str {
        "Bengal"
    }
}
struct Chartreux {}
impl Cat<Chartreux> {
    fn breed_name(&self) -> &str {
        "Chartreux"
    }
}
struct Kinkalow {}
impl Cat<Kinkalow> {
    fn breed_name(&self) -> &str {
        "Kinkalow"
    }
}

pub fn run() {
    let cat: Cat<Aegean> = Cat {
        name: "Niaw".into(),
        breed: PhantomData,
    };
    println!("{}", cat.breed_name());
}
