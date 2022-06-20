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

