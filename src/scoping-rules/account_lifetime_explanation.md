# Understanding Lifetime Bounds in the Account Struct

## The Account Struct Definition

```rust
#[derive(PartialEq, Eq, Hash, Debug)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}
```

## Explanation of Lifetime Bounds

The `Account` struct uses a lifetime parameter `'a` which is applied to both string references (`&'a str`). Here's what this means:

1. **Lifetime Parameter Declaration**: `<'a>` declares a lifetime parameter named 'a' for the struct.

2. **Reference Fields with Lifetime Annotation**: Both `username` and `password` are references to string slices (`&str`) with the lifetime `'a`. This means:
   - Both references must be valid for at least the lifetime `'a`
   - The struct itself cannot outlive the lifetime `'a`
   - Both fields must have the same lifetime `'a`

3. **Purpose of These Lifetime Bounds**:
   - They ensure that the references in the struct remain valid as long as the struct itself is used
   - They prevent the struct from outliving the data it references (which would create dangling references)
   - They allow the compiler to verify memory safety at compile time

## Comparison with Ref Struct from lifetime_bounds.rs

```rust
struct Ref<'a, T: 'a>(&'a T);
```

The `Ref` struct has a more complex lifetime bound:
- It has a lifetime parameter `'a` and a generic type parameter `T`
- The bound `T: 'a` means "any references in `T` must outlive `'a`"

In contrast, the `Account` struct directly uses `&'a str` for its fields, which is a simpler case where the references themselves have the lifetime `'a`.

## Usage in the Code

In the hashmap.rs file, the Account struct is used as a key in a HashMap:

```rust
type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;
```

The `try_logon` function also uses the same lifetime parameter:

```rust
fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    // ...
    let logon = Account {
        username,
        password,
    };
    // ...
}
```

This ensures that:
1. The string references passed to the function have the same lifetime as the references in the Accounts HashMap
2. The Account struct created inside the function has the same lifetime as its fields
3. All references remain valid throughout the function's execution

## Conclusion

The lifetime bounds in the Account struct ensure memory safety by guaranteeing that the struct doesn't outlive the string references it contains. This is a fundamental aspect of Rust's ownership system that prevents dangling references and related memory safety issues.