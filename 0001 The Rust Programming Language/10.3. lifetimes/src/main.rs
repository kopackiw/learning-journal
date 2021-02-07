fn main() {
    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("long string is long");
        let result;
        // {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // invalid lifetime
                                                              // }
        println!("The longest string is {}", result);
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");

        let first_sentence;
        {
            first_sentence = novel.split('.').next().expect("Could not find a '.'");
        }

        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}
