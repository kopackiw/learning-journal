fn main() {
    {
        struct Point<T> {
            x: T,
            y: T,
        }
        // it does not need to be named after generics in struct definition
        // we need to specify Point<U> as Point is unary type constructor
        impl<U> Point<U> {
            fn x(&self) -> &U {
                &self.x
            }
        }
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let _integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        let char = Point { x: 'x', y: 'y' };

        println!("{}", char.x());
        // println!("{}", char.distance_from_origin()); // handled for non-f32
        println!("{}", float.distance_from_origin());
    }
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T> Point<T, T> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
