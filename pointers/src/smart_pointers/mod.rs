
pub fn smart_pointer() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y)
}


pub enum List {
    Const(i32, Box<List>),
    Nil,
}