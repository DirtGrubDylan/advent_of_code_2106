extern crate day_9;

fn main() {
    let new_m = day_9::load_messages_from_file(
        r"C:\Users\dylan\Documents\advent_of_code_2106\python\advents\data\day9_input.txt")
        .unwrap();

    println!("{}", new_m[0].decompress_v1().len());
}
