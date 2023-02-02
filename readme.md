# Basics
## Strings
Difference between `String` and `&str`:
* `String`
  * Resizable
  * Implemented as a vector
  * Constructed with `String::from("popcorn")`
* `&str`
  * String literal, a representation of u8 values.
  * Can't be modified
  * More memory efficient
    
In regards to ownership, using a `String` indicates that we "own" the piece of memory and can modify it. With `&str`, we're dealing with a pointer to memory space.

When creating functions, use `&str` as a param type if we want to only read the value, use `String` if we can to modify it.

## Ownership
The following code works because we create a new `&str` (a reference to a string slice), and assign it to a new var `y`,  
giving us two pointers to the same underlying value.
```rust
fn main() {
    let x = "hello";
    let _y = x;
    
    println!("{}", x);
}
```
Next, we deal with an actual `String` instead of a reference to a string, so Rust enforces the **single-ownership principle**.  
Transferring x to y means x goes out of scope, it will not exist anymore.
```rust
fn main() {
    let x = String::from("hello");
    let y = x;
    
    println!("{}", x);
}
```

Example with functions:
First, we deal with a mutable reference `String`, using the `push_str` method to return it, and assign to a new var `a`.  
We retain ownership of `address`.
```rust
fn main() {
    let address = String::from("Street 1");

    let a = add_postal_code(address);

    println!("{}", a);
}

fn add_postal_code(mut address: String) -> String {
    address.push_str(", 1234 Kingston");
    address
}
```

In this example, the ownership of `address` passed to the function is out of scope, this prints out an empty value.
```rust
fn main() {
    let address = String::from("Street 1");

    let a = add_postal_code(address);

    println!("{:?}", a);
}

fn add_postal_code(mut address: String) {
    address.push_str(", 1234 Kingston");
}
```

🔑 We can use the `to_string` method to convert a `&str` to `String`