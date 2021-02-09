use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, From, To>
where
    T: Fn(From) -> To,
{
    calculation: T,
    dictionary: HashMap<From, To>,
}

impl<T, From, To> Cacher<T, From, To>
where
    T: Fn(From) -> To,
    From: Eq + Hash + Copy,
    To: Copy,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            dictionary: HashMap::new(),
        }
    }

    fn value(&mut self, arg: From) -> To {
        match self.dictionary.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.dictionary.insert(arg, v);

                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let mut expensive_closure_2 = Cacher::new(|string_slice: &String| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        string_slice.chars().count()
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
