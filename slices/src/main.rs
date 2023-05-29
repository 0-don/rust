#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("Slice: {:?}", slice);

    let mut colors = ["red", "green", "blue", "yellow"];
    update_colors(&mut colors[2..4]);
    println!("Colors: {:?}", colors);
}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "orange";
    colors_slice[1] = "purple";
}
