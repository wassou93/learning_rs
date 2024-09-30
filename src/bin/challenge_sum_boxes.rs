/* YOUR CODE GOES HERE */

use std::{cmp::Ordering, ops::Add};


fn sum_boxes<T: Add<Output = T>>(val1: Box<T>, val2: Box<T>) -> Box<T> {
    Box::new(*val1 + *val2)
}

fn boxes_equals<T: PartialEq>(val1: Box<T>, val2: Box<T>) -> Box<bool> {
    if *val1 == *val2 { Box::new(true) } else { Box::new(false) }
}

fn boxes_cmp<T: PartialOrd>(val1: Box<T>, val2: Box<T>) -> Box<Option<Ordering>> {
    Box::new((*val1).partial_cmp(&*val2))
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one.clone(), two.clone()), 3);

    let pi = Box::new(std::f64::consts::PI);
    let e = Box::new(std::f64::consts::E);
    assert_eq!(*sum_boxes(pi.clone(), e.clone()), std::f64::consts::PI + std::f64::consts::E);

    assert_eq!(*boxes_equals(one.clone(), two.clone()), false);
    assert_eq!(*boxes_equals(pi.clone(), e.clone()), false);

    assert_eq!(boxes_cmp(one, two).unwrap(), Ordering::Less);
    assert_eq!(boxes_cmp(pi, e).unwrap(), Ordering::Greater);

    println!("Tests passed!");
}