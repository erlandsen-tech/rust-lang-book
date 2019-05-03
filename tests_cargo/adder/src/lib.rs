#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explore() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn failure_is_always_an_option() {
        assert_eq!(9 / 3, 12);
    }
    #[test]
    fn fail_test() {
        panic!("the disco!");
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 9, height: 10 };
        let smaller = Rectangle { width:  6, height: 8 };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_test() {
        let larger = Rectangle { width: 9, height: 10 };
        let smaller = Rectangle { width:  6, height: 8 };
        assert!(!smaller.can_hold(&larger));
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
