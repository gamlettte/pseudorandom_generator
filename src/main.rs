mod pseudorandom_generator;

fn main() {
    println!("Hello, world!");
    let mut generator = pseudorandom_generator::Generator::new(33);
    let a = generator.get_random_value();
    let mut count = 0;
    while generator.get_random_value() != a {
        println!("{}", a);
        count += 1;
    }
    println!("{}", count);
}
