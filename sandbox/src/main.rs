

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}


struct ImportantExcerpt<'a> {
    part: &'a str,
} 

impl <'a> ImportantExcerpt<'a> {
    fn announce(&self, msg: &str) -> &str {
        println!("Attention: {msg}");
        self.part;
        msg
    }
}


fn main() {

    let x = "first string x";
    let y = "second y";
    let r = longest(x, y);
    println!("The longest str is '{r}'.");

    let book = "This is a book excerpt.";
    let ex = ImportantExcerpt { part: book };

    ex.announce("Hi!.");


}
