use std::fmt;

struct Array<T> {
    data: [T; 1024]
}

impl<T: fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

fn main(){
    // create array with 100 zeroes
    let arr = [0; 10];
    println!("{:?}", arr);

    //print bigger arrays this way instead of using &
    let array = Array { data: [0; 1024] };
    println!("{:?}", array);

}