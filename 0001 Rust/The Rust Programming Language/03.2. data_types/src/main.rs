fn main() {
    {
        let mut x: u8 = 255;
        // x += 1; // check for overflow, nice :)
    }
    {
        let tup = (500, 6.4, 1);
        let (_, y, _) = tup; // _ can be written more than once, nice again

        println!("The value of y is: {}", y);
    }
    {
        let longTuple = (
            500, 6.4, 1, 6.4, 1, 6.4, 1, 6.4, 1, 6.4, 1, 6.4, 1, 6.4, 1, 6.4, 1, 6.4, 1, 6.4, 1,
            6.4, 1, 6.4, 1,
        ); // curious whether there is a limit
    }
    {
        let a = [1, 2, 3, 4, 5]; // fixed length (same as tuple), but required one typ
        let b: [i8; 5] = [1, 2, 3, 4, 5]; // the length of array specified is strange at first, but understandable
        let c = [3; 5]; // kinda strange; can be misunderstand with type annotation; `List.repeat 5 3` is more familiar to me
    }
    {
        let a = [1, 2, 3, 4, 5];
        let index = 10;
        // let element = a[index]; // nice again! but is fixed size, so curious how it is done in vector
    }
}
