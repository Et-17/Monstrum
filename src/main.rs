mod random;

fn main() {
    let mut gen = random::Generator::new(7, 10);
    for _ in 0..100 {
        println!("{:x}", gen.next());
    }
}
