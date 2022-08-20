use core::panic;
use::std::fs::File;
use::std::io::ErrorKind;
use::std::io;

pub fn a() {

    let f = File::open("Hello.text").expect("Failed to open hello.text");

    let f = File::open("Hello.text").unwrap();

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref err) if err.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.text"){
    //             Ok(file) => file,
    //             Err(err) => {
    //                 panic!("tried to create file but there was a problem: {:?}", err)
    //             }
    //         }

    //     },
    //     Err(err) => panic!("There was a problem opening the file: {:?}", err)
    // };
}


pub fn red_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.text")?;
    Ok(s)
}


