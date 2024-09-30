/* YOUR CODE GOES HERE */

use std::ops::Add;


fn sum_boxes<T: Add<Output = T>>(val1: Box<T>, val2: Box<T>) -> Box<T> {
    Box::new(*val1 + *val2)
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(std::f64::consts::PI);
    let e = Box::new(std::f64::consts::E);
    assert_eq!(*sum_boxes(pi, e), std::f64::consts::PI * std::f64::consts::E);
}