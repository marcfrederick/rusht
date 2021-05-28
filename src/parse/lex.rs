// String wird in tokens zerlegt - Nr. 1
pub fn tokenize(str_expr: String) -> Vec<String>
{
    str_expr
        .replace("\\(", " ( ")
        .replace("\\)", " ) ")
        //.replace("\\+", " + ")
        //.replace("\\-", " - ")
        //.replace("\\*", " * ")
        //.replace("\\/", " / ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}


// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://doc.rust-lang.org/rust-by-example/macros.html