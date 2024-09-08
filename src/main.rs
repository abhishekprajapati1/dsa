mod bca;
fn main() {
    println!("Performing linear search.");
    let pos =  bca::search::linear_search([12, 21, 22, 40], 400);
    bca::search::show_pos_msg(pos);

    println!("Performing binary search");
    let pos = bca::search::binary_search([10, 12, 15, 16, 17, 18, 19, 20, 21], 120);
    bca::search::show_pos_msg(pos);
}
