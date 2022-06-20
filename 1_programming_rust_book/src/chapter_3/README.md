# Basic Types

## Types

### Integers
```rust
// Signed integers
i8
i16
i32
i64
```

```rust
// Unsigned integers
u8 
u16
u32
u64
```

```rust
/// Signed and unsigned integers, 
/// the same size as an address on the machine. 
/// (32, 64 bits)

isize, usize
```

### Floating points

```rust
/// Floating point numbers, 
/// single and double precision

f32, f64
```

### Boolean
```rust
/// Boolean
bool
```

### Tuple

```rust
/// Tuple: mixed types allowed
(char, u8, i32)
```

```rust
/// "Unit" (Empty) tuple
()
```

### Struct

```rust
/// Named-field struct
struct S {
    x: f32, 
    y: f32
}
```

```rust
/// Tuple-like struct

struct T(i32, char)
```

```rust
/// Unit-like struct, has no fields
struct E;
```


### Enums
```rust
/// Enumeration, algebraic data type
enum Attend {
    OnTime,
    Late(u32)
}
```

### Box
```rust
/// Box: owning pointer to value in heap
Box<Attend>
```

### Shared, mutable
```rust
/// Shared and mutable references: nonowning
/// pointers that must not outlive their referent
&i32, 
&mut i32
```

### String
```rust
/// UTF-8 string, dynamically sized
String

/// Reference to str: nonowning pointer to
/// UTF-8 text
&str
```

### Arrays
```rust
/// Array, fixed length; elements all of same type
[f64; 4]
[u8; 256]
```

### Vector
```rust
/// Vector, varying length; elements all of same
/// type
Vec<f64>
```

```rust
/// Reference to slice: reference to a portion of
/// an array or vector, comprising pointer and
/// length.
&[u8]
&mut [8]
```

### Traits
```rust
/// Trait object: reference to any value that
/// implements a given set of methods.
&Any,
&mut Read
```

### Points
```rust
/// Pointer to function
fn(&str, usize) -> isize
```

### Closure
```rust
/// (Closure types have nowritten form)
```