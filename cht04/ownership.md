# Understanding ownership

Rust unique rust feature that is most different from other languages. It is the reason rust is memory safe.


## Owernship

Ownership is set of rule how rust manages its memory. Rust memory is managed through a system of ownership with set of rules that compiler checks. If any of rules breaks program won't compile. None of features of ownership will slow down your program while it's running

### Stack and heap

stack and heap

main purpose of ownership is to manage heap data.

keeping tack of what codes using what data on heap, minimizing amount of duplicatge data on heap, and removing used data is problem ownership adresses


### Ownership rules

Each value of rust has owner
There can only one owner at time
When owner get out of scope value will be dropped

### Variable scope

```
{
    let x = '11';
    //here x is valid
}
//here x will out of scope

```

### String type


String literals are convenient but they aren't suitable for every situation. One reason is they are immutable.

```
let mut s = String::from("Hello!");
s.push_str(" world");
println!("{}", s);

```
Differences on how these types deal with memory string can mutated and literals can not.

### Memory and allocation

We can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

* The memory must be requested from the memory allocator at runtime.
* We need a way of returning this memory to the allocator when we’re done with our String.

Rust memory is automatically returned to variable owns it out of scope. When variable out of scope it calls special function `drop` its where author of variable can put code to return the memory.

### Value and data interacting with moves

Multiple variables can interact with the same data in different ways in Rust.

scalar types will be copied when move;

but compound types will not copied it will points to same memory;

when we do
```
let x = String::from("Text");
let y = x;

println!("{}",x);
```

it will throw error because x is invalidated when it move to y;

Rust never automatically deep copy your data. (Its expensive)

### Variable and data interacting with clone

If we do want to deeply copy the heap data of the string, not just stack data. we can use clone method.


### Stack only data copy

all scalar types implemented copy method.

i6-i128, f32-f64, bool, tuple (i32, i32)

but tuple (i32, string) does not implement copy

### OwnerShip and functions

passing variable to function will move or copy and its similar to assigning value to variable

### Return value scopes

Returning values can transfer ownership

## Reference and borrowing


