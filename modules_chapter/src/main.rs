extern crate communicator;

use a::series::of;

fn main(){
    communicator::client::connect();
    of::nested_modules();
}


pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules(){}
        }
    }
}

