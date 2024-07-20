# Tokens

There are two types of tokens:
* `LParens` for `(`
* `RParens` for `)`

# Nodes

There is one type of node:
* `Unit` for `()`

# Types
## Unit
The unit type is represented by `()`.

## Integers
There are multiple types of integers, they can be signed and unsigned and of
multiples sizes.

# Methods
## Addition `+`

The addition operator takes a variable amount of numbers, from `0` to `*`. It
works on every type of integer number, requiring that all inputs are of the same
type and returning the same type.

When used with `0` arguments, like so `(+)` it will return the `0` for that
type.

When used with `1` argument, `(+ 2)` it returns the value of the number
passed.

When used with `2+` arguments, `(+ 1 2 3)` it returns the addition of all
numbers.
