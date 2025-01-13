#[macro_use]
extern crate proc_macro_example_derive;

#[allow(dead_code)]
#[derive(Reflective)]
struct Data {
    input: String,
    numbers: [u32; 5],
}

fn main() {
    println!("Fields: {:?}", Data::get_fields());
}
