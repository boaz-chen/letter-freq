use letter_freq::logic;

fn main() {
    const TEXT: &str = include_str!("../benches/input");

    let res = logic::count_letters_parallel(TEXT, 5);
    println!("{:#?}", res);
}
