pub mod Vector;

#[cfg(test)]
mod tests {
    pub use crate::Vector::Vec2;

    #[test]
    fn vec2_PartialEQ() {
        let x = Vec2::new(0.0, 1.0);
        let y = Vec2::new(0.0, 1.0);

        let z = Vec2::new(0.1, 0.4);
        //Equal
        assert!( x == y);
        assert! (x != z);
    }

    #[test]
    fn vec2_Normalise() {
        let x = Vec2::new(3.0,4.0);
        assert!(x.magnitude() == 5.0);

    }

    #[test]

    fn vec2_ArithmeticOps() {
        let x = Vec2::new(3.0, 4.0);
        let y = Vec2::new(2.0, 3.0);

        let add = x+y;
        let sub = x-y;
        let mul = x.multiply(3.0);

        assert_eq!(add,Vec2::new(5.0, 7.0));
        assert_eq!(sub,Vec2::new(1.0, 1.0));
        assert_eq!(mul, Vec2::new(9.0, 12.0));
    }

    fn vec2_Grid() {
        let x = Vec2::new(3.5, 4.6);

        let x_grid = x.grid();

        assert_eq!(x, Vec2::new(3.0,4.0));
    }
}
