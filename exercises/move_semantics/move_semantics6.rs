// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    // get_char(data); // data remove
    get_char(data.clone());

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    // 由于 to_uppercase() 方法接受 &self 参数，它会返回一个新的 String 实例，而不会修改原始的字符串
    let after_data = data.to_uppercase();
    println!("{}", data);
}




