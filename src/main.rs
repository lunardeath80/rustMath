mod prime;

fn main() {
    for i in 0..100 {
        println!("{}: has pf: {:?}", i, prime::prime_factorise(i));
    }
}
