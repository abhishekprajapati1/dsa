mod algorithms;
fn main() {
    println!("Performing linear search.");
    let pos =  algorithms::search::linear_search([12, 21, 22, 40], 400);
    algorithms::search::show_pos_msg(pos);

    println!("Performing binary search");
    let pos = algorithms::search::binary_search([10, 12, 15, 16, 17, 18, 19, 20, 21], 120);
    algorithms::search::show_pos_msg(pos);
}
