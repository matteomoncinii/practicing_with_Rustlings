fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let name: &str = cat.0;
    let age: f64 = cat.1;

    // Destructuring the tuple.
    // let (name, age) = cat;

    println!("{name} is {age} years old");
}
