fn main() {
    let mut s = "hello";
    // s.push_str(", world!"); // This will not work as in example below. &str doesn't have push_str() method

    {
        let d: &str = "scoped hello";
    }

    println!("{}", s);
    // println!("{}", d) <- This will not run.

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}
