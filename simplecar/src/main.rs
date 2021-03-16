use util::print_vector_i32;
use util::imply;

fn main() {
    // let v = vec![1, 2, 3, 4];
    // print_vector_i32(&v);
    let v1 = vec![2, 3, 4, 5];
    let v2 = vec![1, 2, 3, 4];

    println!("{}", imply(&v1, &v2));
}
