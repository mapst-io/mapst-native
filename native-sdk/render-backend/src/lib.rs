use geometry;
use geometry::Point;
use std::any::Any;


fn test() {
    let point: Point<i64> = Point::max();
    let d = point.clone();
    let k = point;
    let s = Point::new(111, 2222);
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
