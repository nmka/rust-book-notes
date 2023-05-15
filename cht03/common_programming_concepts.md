# Common programming concepts

### Introduction

In this chapter we will discuss common programming concepts not unique to rust

`Keywords` are set of words used by rust language only. Some of them don't have a functionality bus still reserved for future

### Variables and mutability

Variable: are immutable by default. If you want to mutate it you have to use `let mut variable_name` when you declare

Constants: they set on build not on runtime. they are valid for inter time of program runtime

Shadowing: when using same name for variable. But you have to use let in front of it to used

```
let x = 5;

let x = x + 1
```


### Data types

Two subset of data types. Scalar and compound

Scalar: value represents single unit like long, int, float, char, byte etc
int: 8 - 128 If integer overflow happens on release build it will turn :u8 `257 -> 1`
float: 32-64 f32 is single precision f64 is double percision floats. float represents according to IEEE-745
numeric operation: +-*%/

Compound:

compound types can group multiple values into one type.

tuple type: tuple is general way of grouping together number of values into one compound type

```
 let tup: (i32, f32, u8) = (-1421, 32.12, 400);
```
You can access tuple values like this `tup.0 || tup.1`

Array type: unlike tuple every element of array should be same type. Arrays in rust have fixed length

declaring array:

```
    let a = [0,1,2,3,4,5];

    let months = ["string1", "String2"];

    let with_type: [i32; 5] = [1,2,3,4,5]
```
An array isn’t as flexible as the vector types

```
    let a: [3, 5];
    // is same as
    a = [3,3,3,3,3];

```

### functions

very similar to other languages.
last line is return but you shouldn't use ; on *
```
//    example

let x = {
    let y = 3;
    3+x
}
```

### comments

use `//` to comments

### control flow


