# Nand2Tetris

Do programmers using a high-level language need to know anything about underlying low-level details? They do not specialize in kernel or embedded programming, so why bother? If they will be assigned a task to create a computer from basic logic gates, will they prevail?

## Goal of the project

1. To gain knowledge how computer works at a hardware level;
2. To learn how the software interacts with the hardware.

## Project resources

1. Noam Nisan, Shimon Schocken, _Build a Modern Computer from First Principles: From Nand to Tetris_
   - view [part 1](https://www.coursera.org/learn/build-a-computer) & [part 2](https://www.coursera.org/learn/nand2tetris2) on Cursera
2. Noam Nisan, Shimon Schocken, _The Elements of Computing Systems: Building a Modern Computer from First Principles_ (The MIT Press, 2005)
   - [view the book on Amazon](https://www.amazon.com/Elements-Computing-Systems-Building-Principles/dp/0262640686/ref=ed_oe_p)

## Project journey

That's it, my first journey!

### 0. Introduction

<details>
   <summary>Abstraction and implementation</summary>

#### Abstraction and implementation

> Printing "Hello World" on the screen actually involves setting a bunch of pixels on your screen to be lighter or darker. You have to put the pixels that are lighter in a very special order to somehow represent the letter H and then the letter E. How did it happen? [...] The "how" is called an implementation and the "what" is an abstraction. [...] Due to abstraction, we can separate concerns. When we can separate, we can forget a lot of details about implementation. You can repeat that many times in many multiple layers of abstraction, one above the other.

The multiple levels of abstraction idea is explained very well by this quote.

> So here we are at the very low level of everything in, in applied computer science. And this actually is not computer science. This is electrical engineering and solid state physics. And all sorts of things that neither Norm and I understand much about. And therefore, we're going to obstruct the way of this hardware and focus instead on the most elementary logic gate that we can think of, which is called NAND.

Great reference to previous part with abstraction, when we need to abstract over electrical engineering stuff.

</details>

### 1. Boolean functions and gate logic

<details>
   <summary>State representation</summary>

#### State representation

> You've probably all heard that computers internally only have 0s and 1s. It's simplest to have only two possible values that you need to maintain.

##### N = 0

Considering a zero-element state representation is not practical. It cannot be instantiated, as the state is not representable by definition. The equivalence in programming languages is `void` or `never`. Mathematically it is an empty set (`{}`).

##### N = 1

Considering a one-element state leads to confusion. It has one member and the information can be saved, but the meaning cannot be obtained. It is a similar concept to a set containing one element - `{ () }`. In programming languages it's called a `unit` or `()`.

Example:

1. There is a board with all facts about **existing** personal relationships **we know**, represented in a `<person1><person2>: ()` manner.
2. Person A is in relationship with person B, denoted as `AB : ()` inscription on the board.
3. **We also know** that person C **is not** in relationship with person D.
4. If we try to denote it on the relationship board, we are facing with lack of "other representative" which can deny being in relationship. On the other hand, if we just skip this piece of information, we are rejecting **a fact that we know** about the world.

Using a one-element state to represent a more complex world is not enough.

##### N = 2

True and false, one and zero, yin and yang - possible representations of state which is able to describe all world around us in a precise way. Remember `boolean`?

##### N > 2

True, false, and maybe? Zero, one, or a half? The state containing more elements is more precise, but as higher the abstraction (dimension) goes, the implementation (with our current technology) becomes more complex.

</details>

<details>
   <summary>Boolean expressions</summary>

#### Boolean expressions

`Boolean` is a set with two elements: `{ True, False }`. The elements of the set are the simplest values and all operations can be evaluated either to `True` (`1`) or `False` (`0`).

A function is a transformation of an input into an output e.g. `AND`, `OR`, and `NOT`.

> Once we have functions, we can start combining them.

Example of operation composition from the course:

```text
  1 AND (0 OR (NOT (1)))
= 1 AND (0 OR 0)
= 1 AND 0
= 0
```

It's true (pun!) with boolean world, but try to imagine a function composition with one function returning `void` element and the second expecting a `boolean` value on input.

_My opinion is that programmers always should be aiming to "process" an input into the output by function composition._

</details>

<details>
   <summary>Boolean algebra</summary>

#### Boolean algebra

> In its most general form, algebra is the study of mathematical symbols and the rules for manipulating these symbols.

Relaying on the cite above, we can assume that algebra is all around us, as all civilization is based on symbols. They might be letters, digits etc. All what's needed to build or to share knowledge.

##### Commutative law

In algebra there may exist some laws. Binary function like `AND` and `OR` have some really nice trait - they are commutative. The order of operands does not matter.

`x OR y = y OR x`
`x AND y = y AND x`

Worth to remember that it's a trait of a single function rather than a whole universe. In the realm of Rational numbers, addition is commutative, and division is not.

##### Associative law

Another real cool law is associativity. It can be remembered as "I do not need parenthesis".

`x AND (y AND z) = (x AND y) AND z`
`x OR (y OR z) = (x OR y) OR z`

##### Distributive law

Third law one can apply to boolean algebra is distributive law.
`x OR (y AND z) = (x OR y) AND (x OR z)`
`x AND (y OR z) = (x AND y) OR (x AND z)`

##### De Morgan laws

Should sound familiar to all CS student which took Logics 101.

`NOT(x AND y) = NOT(x) OR NOT(y)`
`NOT(x OR y) = NOT(x) AND NOT(y)`

</details>
