const _A: u8 = 2; // globals const ok

//  let b: u8 = 2; // globals let not ok -- nice

fn main() {
    {
        let _x = 6;
        // x = 5; // This is error; nice.
        let _x = 5; // But you can redeclare it :(
    }

    {
        let _x = 5;
        let _x = _x;

        let mut _y = 6;
        let _y = _y;

        // So you can redeclare with opposite mutability setting.
    }
    {
        let _x = 5;
        // const y: u8 = x; // does not work, not sure why
    }
}
