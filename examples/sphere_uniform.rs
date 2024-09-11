use rayonetta::utils::random_uniform;

fn main() {
    for _ in 0..10000 {
        let v = random_uniform();
        println!("{}", v);
    }
}
