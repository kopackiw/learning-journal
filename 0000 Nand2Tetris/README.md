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

Great reference to previous part with abstraction, when one need to abstract over electrical engineering stuff.

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

<details>
   <summary>Building simple logic gates</summary>

#### Building simple logic gates

One gate to rule them all - let me introduce a `NAND` gate. It's a building block for all other gates. Starting only with the `NAND` gate, it is the first task in this course to implement `NOT`, `AND`, and `OR` gates.

##### Implementing NOT gate

Let's start with a description of both `NAND` and `NOT` gate.

| a   | b   | NAND(a,b) |
| --- | --- | --------- |
| 0   | 0   | 1         |
| 0   | 1   | 1         |
| 1   | 0   | 1         |
| 1   | 1   | 0         |

| a   | NOT(a) |
| --- | ------ |
| 0   | 1      |
| 1   | 0      |

An interface of the `NOT` gate requires 1 input, but the `NAND` gate requires two of them. The signal must be therefore split into two inputs. The `NAND` gate looks like this:

| a   | b (=a) | NAND(a,b) |
| --- | ------ | --------- |
| 0   | 0      | 1         |
| 1   | 1      | 0         |

Both tables (`NOT(a)` and `NAND(a,b=a)`) are equal now.

##### Implementing AND gate

As there are already `NOT` and `NAND` gates available, a the `AND` gate can be build by using double negation law (`NOT(NOT(a)) == a`). Therefore: `AND(a,b) = NOT(NOT(AND(a))) = NOT(NAND(a))`.

##### Implementing OR gate

Again, one of laws can be used to obtain an `OR` gate. Starting from de Morgan law: `NOT(x AND y) = NOT(x) OR NOT(y)`, let's introduce `a = NOT(x)` and `b = NOT(y)`, hence `NOT(NOT(a) AND NOT(b)) = a OR b`. It can be simplified to `NAND(NOT(a), NOT(b)) = a OR b`.

</details>

<details>
   <summary>Building more complex logic gates</summary>

#### Building more complex logic gates

The goal of this subchapter is to build working `XOR`, `MUX`, and `DMUX` gates using all gates build previously.

##### Implementing XOR gate

The `XOR` gate evaluates to `1` only if the operands have opposite values.

| a   | b   | XOR(a, b) |
| --- | --- | --------- |
| 0   | 0   | 0         |
| 0   | 1   | 1         |
| 1   | 0   | 1         |
| 1   | 1   | 0         |

To be evaluated to `1`, the second row (`NOT(a) AND b`) or third row (`a AND NOT(b)`) must be evaluated. Hence:

```text
XOR(a,b) = (NOT(a) AND b) OR (a AND NOT(b)) // (a AND NOT(b)) = c
XOR(a,b) = (NOT(a) AND b) OR c // using distributive law
XOR(a,b) = (NOT(a) OR c) AND (b OR c)
XOR(a,b) = (NOT(a) OR (a AND NOT(b))) AND (b OR (a AND NOT(b))) // using distributive law twice
XOR(a,b) = ((NOT(a) OR a) AND (NOT(a) OR NOT(b))) AND ((b OR a) AND (b OR NOT(b)))
XOR(a,b) = (1 OR (a NAND b)) AND ((b OR a) AND 1)
XOR(a,b) = (a NAND b)) AND (b OR a)
```

They must not be both ones (`a NAND b`) and they must evaluate to 1 (`b OR a`).

##### Implementing MUX gate

A multiplexer chip is responsible for switching between two signals based on provided flag `s`.

| s   | MUX(a,b,s) |
| --- | ---------- |
| 0   | a          |
| 1   | b          |

It can either (`OR`) return `a` when the signal is 0 (`NOT(s)`) or return `b` when the signal is 1 (`s`). So the following expressions implements the `MUX` interface.

`(NOT(s) AND a) OR (s AND b)`

##### Implementing DMUX gate

A demultiplexer chip is the reverse of the previous one. Based on provided flag `s`, it channels provided input onto one of two outputs.

| s   | DMUX(a,s) |
| --- | --------- |
| 0   | [a, 0]    |
| 1   | [0, a]    |

Similarly to the previous gate, the input must be paired with both `s` and `NOT(s)`. Due to the law of excluded middle, one of the values (`s` or `NOT(s)`) must be true. The "truthy" one will keep the value `a` after being paired, the "falsy" one will evaluate to `0`. To sum up, following code is a demultiplexer:
`[x, y] = [NOT(s) AND a, s AND a]`

</details>

<details>
   <summary>Building 16-bit variants gates</summary>

#### Building 16-bit variants gates

`NOT16`, `AND16`, `OR16` are pretty straightforward to build. All pair of bits must be "notted", "anded", or "ored" together and pass to the output. I was hoping that `MUX16` will be a small challenge, but it works similar to the gates above.

</details>

<details>
   <summary>Building multi-way variants gates</summary>

#### Building multi-way variants gates

In this subchapter there will be introduced new multi-way gates.

##### Implementing Or8Way gate

Exercises rather simply - one need to fold / reduce all bits with `OR` function. Done with 7 `OR` gates.

##### Implementing Mux4Way16 gate

Four inputs and two bits acting like flags. I've come with an idea of splitting incoming signals into pairs (based on the flag on a given index) and "MUXing" them. Then, the "winners" will be "MUXed" together with flags coming from the other index. During implementation, I've made a mistake with indexes (it was indexed from right-most bit and not left-most one).

##### Implementing Mux8Way16 gate

Concept identically with the previous one. The difference is we group not two of inputs, but four of them, and take two winners to `MUX16`. With right indexing in mind, I've accomplished it on a first attempt. Alternative implementation is to use first 4 `MUX16` on 4 pairs, and then run `MUX4WAY16` with four "winners".

##### Implementing DMux4Way gate

The trick of "combining" values with the selector on the corresponding index does the trick. Since there were two control flags, they had to be "anded" respectively to return `1` for one selector and `0` for the other.

##### Implementing DMux8Way gate

I've stuck. Tried to do `DMux4Way` twice (on (a,b,c,d) and (e,f,g,h) outputs) and then override `a` and `e` outputs with `DMUX`. It is not allowed. I've looked at implementation in `DMux4Way` and thought it will be too cumbersome to how so many `AND` gates. I've known that I've missed something. After looking on the Internet, I've spot a solution similar to my original concept, but the one difference was that `DMUX` was on first line rather than on last one. Due to that, we can pass input to proper `a` or `e` channel. I've updated this solution as well as `DMux4Way`.

</details>

### 2. Boolean arithmetic and ALU

<details>
   <summary>Binary numbers</summary>

#### Binary numbers

Humans are used to decimal system when counting. But there are other ones! We use full 60 when talking about a minute or hour. Half an hour is not 5 minutes, but 30. As there are a lot of resources talking about binary numbers, I will not do the same here. Finishing with some tip - a person can count up to 1 023 with 10 fingers just by using binary system.

</details>

<details>
   <summary>Binary addition</summary>

#### Binary addition

On the way to ALU!

##### Implementing HalfAdder

`HalfAdder` produces a tuple of sum of two inputs, as well as carry from that addition. When adding two bits, the result will be `1` if and only if these two bits are opposite signs, so `XOR` gate must be used. The carry will be present if and only if both two bits will be equal to `1`, so `AND` gate will be used.

##### Implementing FullAdder

Let use `HalfAdder` with two first inputs. Returned value of `carry` must be remembered. Returned value of `sum` must be pinned with a third input into `HalfAdder`. The resulting `sum` is the final one. One needs to check whether first `OR` second `carry` has return `1` - this will be final `carry`.

##### Implementing Add16

Addition of 16 bits in pretty straightforward as all we need is to pin together 16 bits with full adders, moving `carry` from right to left, bit by bit, throwing away `carry` first from the left.

</details>

<details>
   <summary>Negative numbers</summary>

#### Negative numbers

At the beginning, the concept with 2's complement system bothered me. But when it was presented as follows `... 6, 7, -8 (8), -7 (9), -6 (10), ...`.

Task: add (-2) and (-3).
(-2) represents the same as 16 - 2 = 14, so `1110`.
(-3) represents the same as 16 - 3 = 13, so `1101`.

```math
  1110_2
+ 1101_2
= 1011_2
```

`1011` represents 11 (when unsigned), but also 16 - 11 = -5 (when signed).
The idea of adding two number that will result in overflow is beautiful!

##### Implementing Inc16

Initial idea of writing 16 `FullAdder`s I throw into a bin. I did not know the syntax for expressing a bus with only the rightmost bit set to one, so I've searched and found. Armed with a new syntax knowledge, I'm moving further!

</details>

<details>
   <summary>Arithmetic logic unit</summary>

### Arithmetic logic unit

I've always been curious about how the ALU processes all those function, which tends to be stored "as bits". How a sequence of bits can manipulate another?

This table shown below blew my mind! This sequence of bits is just a set of control flags which composed in a specific way results in a given function!

<img width="200" alt="Screenshot 2021-01-29 at 21 57 54" src="https://user-images.githubusercontent.com/26244440/106326794-59aee400-627d-11eb-996e-bd887214c2ea.png">

`zx` - omit x input and pass 0
`nx` - negate x input
`f` - do logical AND (when 0) or ADD (when 1)

### Implementing ALU

After some minutes of self-doubt, I was enlightened! It's `MUX16` everywhere! I must do both operations (e.g. logic sum and logic and) and then check via MUX which result pass further. I've had a problem with outputting control flags due to syntax error e.g. using sub bus of internal node is not allowed:

```text
   Mux16(a=afterFunctionCheck, b=negatedOutput, sel=no, out=result);

   Or8Way(in=resultRight, out=isFirstSevenBitsNotZeroes);
   Not(in=isFirstSevenBitsNotZeroes, out=isFirstSevenBitsZeroes);
   Or8Way(in=resultLeft, out=isLastSevenBitsNotZeroes);
   Not(in=isLastSevenBitsNotZeroes, out=isLastSevenBitsZeroes);

   And(a=isFirstSevenBitsZeroes, b=isLastSevenBitsZeroes, out=zr);
   And(a=result[15], b=true, out=ng);
   Or16(a=result, b=false, out=out);
```

Again, after checking on the Internet, I found the correct syntax and adjust solution to:

```text
   Mux16(a=afterFunctionCheck, b=negatedOutput, sel=no, out=out, out[8..15]=resultLeft, out[0..7]=resultRight, out[15]=ng);

   Or8Way(in=resultRight, out=isFirstSevenBitsNotZeroes);
   Not(in=isFirstSevenBitsNotZeroes, out=isFirstSevenBitsZeroes);
   Or8Way(in=resultLeft, out=isLastSevenBitsNotZeroes);
   Not(in=isLastSevenBitsNotZeroes, out=isLastSevenBitsZeroes);
   And(a=isFirstSevenBitsZeroes, b=isLastSevenBitsZeroes, out=zr);
```

</details>
