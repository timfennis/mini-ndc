# Mini NDC

A very minimal programming language that can do simple floating point math. The purpose of this language is
to give a demonstration of an even further simplified interpreter from on the book [Crafting Interpreters](https://craftinginterpreters.com).

The language supports:
 * Floating point literals
 * Addition, subtraction, multiplication, division (left associative)
 * Exponentiation (right associative)
 * Grouped expresions using parentheses

 ## Example

 You can pass the program as the first argument:

 ```bash
mini-ndc "5 + 5 * 3"
Result: 20
```
