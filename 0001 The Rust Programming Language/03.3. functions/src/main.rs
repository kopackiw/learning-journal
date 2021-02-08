fn main() {
    println!("Hello, world!");

    {
        // println!("another_function {}", another_function); // cannot hack this :D
    }
    {
        // let a = fn a() {} // no anonymous?
    }
    let q = {
        let x = 3;
        x + 1
    };

    println!("q: {}", q);
}

fn _another_function() -> () {
    println!("Another function.");
}

fn _get_function() {
    // return another_function; meh :(
}
