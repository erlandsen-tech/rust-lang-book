fn main() {
    let number_list = vec![34, 50, 25, 10, 65];
//    println!("The largest number is {}", largest(&number_list));

    let point = Point {x: 3.4, y: 2.8};
    println!("Distance from origin if 3.4, 2.8: {}" , 
            point.distance_from_origin()); 
}

struct Point<T> {
    x: T,
    y: T,
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}
