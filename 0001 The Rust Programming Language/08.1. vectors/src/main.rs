fn main() {
    {
        let mut v: Vec<i8> = Vec::new();

        v.push(4);
    }
    {
        let mut v = get_simple_vector();

        for i in &mut v {
            *i += 50;
        }
    }
}

fn get_simple_vector() -> Vec<i32> {
    vec![1, 2, 3]
}
