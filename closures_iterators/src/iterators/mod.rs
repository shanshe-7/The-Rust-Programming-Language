
pub fn iterate() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let  v2: Vec<_> =v1.iter().map(|x|x+1).collect();

    for value in v1_iter {
        println!("Value is: {}", value)
    }
}