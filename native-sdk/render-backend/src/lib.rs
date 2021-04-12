use geometry;
use geometry::Point;
use std::any::Any;


fn test() {
    let mut previous: Point<i32> = Point::new(32, 32);
    let next: Point<i32> = Point::new(64, 64);
    previous += &next;
    previous.clone();
}

#[cfg(test)]
mod tests {
    use super::test;

    #[test]
    fn it_works() {
        test();
        assert_eq!(2 + 2, 4);
    }
}
