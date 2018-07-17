extern crate blockcounter;

fn main() {
    let lines = vec![
        "   // comment",
        "// comment",
        "   non-comment",
        "non-comment",
        "//",
        "",
        "non-comment",
    ];
    let text = {
        let mut text = String::new();
        for line in lines {
            text += line;
            text += "\n"
        }
        text
    };
    println!("{}", text);
    println!("===");
    let comments = vec!["//".to_string()];
    for block in blockcounter::Blocks::new_with_comments(1, text.as_bytes(), &comments) {
        println!("{}", blockcounter::clean(&block));
        println!("=========");
    }
}
