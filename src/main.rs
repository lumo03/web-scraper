use crate::helper_methods::fetch_recipes;

mod helper_methods;
mod tests;

fn main() {
    let recipes = fetch_recipes();
    println!("{:?}", recipes);
}
