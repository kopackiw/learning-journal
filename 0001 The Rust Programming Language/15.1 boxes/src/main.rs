use crate::List::{Cons, Nil};
use crate::Nat::{Suc, Zero};
use crate::Tree::{Leaf, Node};

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>),
}

impl List {
    fn to_vector(&self) -> Vec<i32> {
        match self {
            Nil => vec![],
            Cons(v, b) => {
                let mut result = b.to_vector();
                result.insert(0, *v);

                result
            }
        }
    }
}

#[derive(Debug)]
enum Nat {
    Zero,
    Suc(Box<Nat>),
}

impl Nat {
    fn to_integer(&self) -> i32 {
        match self {
            Zero => 0,
            Suc(b) => b.to_integer() + 1,
        }
    }
}

#[derive(Debug)]
enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?} is equivalent for {:?}", list, list.to_vector());

    let two = Suc(Box::new(Suc(Box::new(Zero))));
    println!("{:?} is equivalent for {}", two, two.to_integer());

    let tree: Tree<i32> = Node(
        Box::new(Leaf(5)),
        Box::new(Node(Box::new(Leaf(10)), Box::new(Leaf(12)))),
    );
    println!("{:?}", tree);
}
