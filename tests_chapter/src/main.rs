
fn main() {
    println!("Hello, world!");

  


}


#[cfg(test)]
pub mod tests {
   
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    // #[test]
    // // fn another() {
    // //     panic!("Make it fail")
    // // }

    #[derive(Debug)]
    pub struct Rectangle {
        length: u32,
        width: u32
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 10, width: 8};
        let smaller = Rectangle { length: 8, width: 7};

        assert!(!smaller.can_hold(&larger))


    }

}

