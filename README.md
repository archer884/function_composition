# function_composition
Function composition demonstration from rust programming video #10

This project is intended to demonstrate how to take two functions and stitch them together into one function in Rust. In the video, I also discuss why what looks like a bare function is subject to a lifetime like any other struct or value: underneath the hood, Rust creates functions like `let f = |n: u32| n * 2` by transforming them into structs implementing a single function. Any environment variables closed over by the function (which is a closure!) will be included as fields in this struct.

Cool, huh?
