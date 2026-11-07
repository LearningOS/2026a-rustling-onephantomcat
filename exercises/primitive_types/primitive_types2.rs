// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let letter: char = '中';
    println!("字符：{}", letter);

    let data = [10, 20, 30, 40, 50];
    let middle: &[i32] = &data[1..4];
    println!("中间三项：{:?}", middle);

    let info = ("Rust", 2026, true);
    let (name, year, ready) = info;
    println!("{} / {} / {}", name, year, ready);
    println!("元组的第二项：{}", info.1);

    let x: i32 = 5;
    let half: f64 = (x as f64) / 2.0;
    println!("除法结果：{}", half);
}
