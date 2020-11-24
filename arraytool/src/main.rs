extern crate array_tool;

use array_tool::vec::Intersect;

fn main() {
    let arr1 = vec![0, 0, 0, 0, 5, 5, 5, 5, 0, 0, 0, 0];
    let arr2 = vec![1, 2, 3, 4, 5, 6, 1, 1, 2, 3, 4, 5];
    let result = arr2.intersect(arr1);

    println!("{:?}", result);
}
