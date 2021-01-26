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

### 26th January 2021

#### 0. Introduction

> Printing "Hello World" on the screen actually involves setting a bunch of pixels on your screen to be lighter or darker. You have to put the pixels that are lighter in a very special order to somehow represent the letter H and then the letter E. How did it happen? [...] The "how" is called an implementation and the "what" is an abstraction.[...] Due to abstraction, we can separate concerns. When we can separate, we can forget a lot of details about implementation. You can repeat that many times in many multiple layers of abstraction, one above the other.

The multiple levels of abstraction idea is explained very well by this quote.

> So here we are at the very low level of everything in, in applied computer science. And this actually is not computer science. This is electrical engineering and solid state physics. And all sorts of things that neither Norm and I understand much about. And therefore, we're going to obstruct the way of this hardware and focus instead on the most elementary logic gate that we can think of, which is called NAND.

Great reference to previous part with abstraction, when we need to abstract over electrical engineering stuff.
