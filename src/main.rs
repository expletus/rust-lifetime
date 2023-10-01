fn main() {
    let mut str_split = stringsplit::StrSplit::new(
        "Hello! My Name is Nikhil! And I am here to learn Rust! Stop me if you can",
        "!",
    );
    while let Some(fragment) = str_split.next() {
        println!("{}", fragment);
    }
}
