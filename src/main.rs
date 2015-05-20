#![feature(box_syntax)]

fn main() {
    // We begin by getting the second (on windows, it would be the first) command line argument.
    // Thing is, the user may or may not have supplied us with any arguments, so the result of this
    // expression is not a `String`, but an `Option<String>`.
    let value = std::env::args().nth(1);
    let convert_and_double = compose(convert, double);

    match convert_and_double(value) {
        Some(n) => println!("{}", n),
        None => println!("No value"),
    }
}

fn convert(n: Option<String>) -> Option<f32> {
    n.and_then(|n| n.parse().ok())
}

fn double(n: Option<f32>) -> Option<f32> {
    n.map(|n| n * 2.0)
}

// This function accepts any two functions and composes them into a single function accepting the
// input type for the first function and returning the output type of the second function.
fn compose<'f, T1, T2, T3, F1, F2>(a: F1, b: F2) -> Box<Fn(T1) -> T3 + 'f>
    where   F1: Fn(T1) -> T2 + 'f,
            F2: Fn(T2) -> T3 + 'f
{
    box move |input| b(a(input))
}
