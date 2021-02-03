const a: u8 = 2; // globals const ok

//  let b: u8 = 2; // globals let not ok -- nice

fn main() {
    {
        let x = 6;
        // x = 5; // This is error; nice.
        let x = 5; // But you can redeclare it :(
    }

    {
        let x = 5;
        let mut x = x;

        let mut y = 6;
        let y = y;

        // So you can redeclare with opposite mutability setting.
    }
    {
        let x = 5;
        // const y: u8 = x; // does not work, not sure why
    }
}
