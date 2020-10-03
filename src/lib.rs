pub mod prime;
pub mod vector;

#[cfg(test)]
mod tests {
    pub use crate::prime;
    pub use crate::vector::Vec2;
    #[test]
    fn vec2_partial_eq() {
        let x = Vec2::new(0.0, 1.0);
        let y = Vec2::new(0.0, 1.0);

        let z = Vec2::new(0.1, 0.4);
        //Equal
        assert!(x == y);
        assert!(x != z);
    }

    #[test]
    fn vec2_normalise() {
        let x = Vec2::new(3.0, 4.0);
        assert!(x.magnitude() == 5.0);
    }

    #[test]

    fn vec2_arithmetic_ops() {
        let x = Vec2::new(3.0, 4.0);
        let y = Vec2::new(2.0, 3.0);

        let add = x + y;
        let sub = x - y;
        let mul = x.multiply(3.0);

        assert_eq!(add, Vec2::new(5.0, 7.0));
        assert_eq!(sub, Vec2::new(1.0, 1.0));
        assert_eq!(mul, Vec2::new(9.0, 12.0));
    }

    #[test]
    fn vec2_grid() {
        let x = Vec2::new(3.4, 4.6);

        assert_eq!(x.grid(), Vec2::new(3.0, 5.0));
    }

    #[test]
    fn vec2_rotate() {}

    #[test]
    fn vec2_prime_sieve() {
        let up_to_ten = prime::prime_sieve(30);
        assert_eq!(up_to_ten, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
