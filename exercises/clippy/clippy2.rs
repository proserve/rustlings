// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM DONE

fn main() {
    let res = 42;
    let option = Some(12);
    // for x in option {
    //     res += x;
    // }
    println!("{}", res + option.unwrap());
}
