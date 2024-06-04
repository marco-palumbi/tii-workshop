use p22::calc::*;

#[test]
fn it_works() {
    let result = celsius2farenheit(3);
    assert_eq!(result, 37);
}
#[test]
fn also_this_works() {
    let result = celsius2farenheit(6);
    assert_eq!(result, 42);
}
#[test]
fn test1_farenheit2celsius() {
    let result = farenheit2celsius(42);
    assert_eq!(result, 5)
}
#[test]
fn test0_fibonacci_loop() {
    let result = fibonacci_loop(0);
    assert_eq!(result, 0)
}
#[test]
fn test1_fibonacci_loop() {
    let result = fibonacci_loop(1);
    assert_eq!(result, 1)
}
#[test]
fn test2_fibonacci_loop() {
    let result = fibonacci_loop(2);
    assert_eq!(result, 1)
}
#[test]
fn test3_fibonacci_loop() {
    let result = fibonacci_loop(19);
    assert_eq!(result, 4181)
}
#[test]
fn test0_fibonacci_rec() {
    let result = fibonacci_rec(0);
    assert_eq!(result, 0)
}
#[test]
fn test1_fibonacci_rec() {
    let result = fibonacci_rec(1);
    assert_eq!(result, 1)
}
#[test]
fn test2_fibonacci_rec() {
    let result = fibonacci_rec(2);
    assert_eq!(result, 1)
}
#[test]
fn test3_fibonacci_rec() {
    let result = fibonacci_rec(19);
    assert_eq!(result, 4181)
}
