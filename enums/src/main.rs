fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    };

    let home = IpAddr::V4(String::from("127.0.0.1"));

    println!("home is: {:#?}", home);

    enum Option<T> {
        Some(T),
        None,
    }

    enum Coin  {
        Penny,
        Nickel,
        Dime,
        Quarter
    }

    fn value_in_cents(coin: Coin) -> i32 {
        match coin {
            Coin::Penny =>1,
            Coin::Nickel =>5,
            Coin::Dime =>10,
            Coin::Quarter =>25,
            
        }
    }

    println!("value is: {}", value_in_cents(Coin::Quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}