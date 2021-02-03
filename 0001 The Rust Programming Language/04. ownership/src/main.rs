// Problem submitted: https://users.rust-lang.org/t/how-to-properly-use-dereferencing-mechanism/55033
fn main() {
    fn take_string_ref(ref_to_string: &mut String) {
        // Question 1
        ref_to_string.push_str("without dereferencing |");
        // above is syntax sugar for String::push_str(ref_to_string, "without dereferencing |");

        (*ref_to_string).push_str("| with dereferencing");

        // Question 2
        let s = ref_to_string.to_owned();
        // let q = *ref_to_string; // <- cannot do a dereference
    }

    fn take_int_ref(ref_to_int: &mut i32) {
        // Question 3
        // ref_to_int += 1;
        *ref_to_int += 1;
        println!("-> {}", ref_to_int);
        println!("-> {}", *ref_to_int);
    }

    let mut x = 5;
    let y = &mut x;

    let mut some_string = String::from("");

    take_int_ref(y);
    take_string_ref(&mut some_string);

    println!("value: {}", some_string);
}
