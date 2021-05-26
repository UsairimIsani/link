use std::any::type_name;

mod graph;
mod tree;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
fn main() {
    println!("Hello, world!");
}
