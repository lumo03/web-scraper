use crate::helper_methods::fetch_recipes;

mod helper_methods;
mod tests;

#[tokio::main]
async fn main() {
    let recipes = fetch_recipes().await;
    println!("{:?}", recipes);
}
