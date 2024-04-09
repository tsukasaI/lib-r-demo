mod generator;

pub fn print_random_number() {
    let n = generator::gen_rand();
    println!("Random u8: {}", n);
}
