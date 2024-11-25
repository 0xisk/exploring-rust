## Chapter 4: "Understanding Ownership":
### What is Ownership?
Ownership is a set of rules that governs how Rust manages the memory. We have two locations in the memory any programming language including Rust is using to save the memory, the **Stack** and the **Heap**. 

### Stack VS Heap
The **Stack** holds all the data that are with a known fixed size. Whereas data with unknown size at compilation time will be stored in the **Heap**. The process of adding into the Stack is called **PUSH** and of removing from the Stack is called **POP**, while the process of adding into the Heap is called **ALLOCATING**. Pushing into the stack is always faster than allocating on the heap; because the allocator never has to search for a new location since that location will be always on top of the stack. Therefore, allocating more space on the heap requires more work becuase the allocator must first find a new place in the heap that is big enough for holding the data and then perform bookkeeping for preparing to the next allocation. 
Ownership then is the process of managing the data on the heap. 

### Ownership Rules
1. Each value in Rust must has an Owner. 
2. There can only be one owner at a time. 
3. When the owner goes out of the scope, the value will be dropped. 

### The `String` type
This type manages data allocatied on the heap and is able to store an amount of text that is unknown at the compilation time. 
```rs=
let s = String::from("hello")
```

### Memory and Allocation
The string literals `&str` are fast and efficient than String type because of what discussed earlier. That is why str literals are immutable. 
```rs=
let greeting: &str = "Hello, world!";
```
Unfortunately, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compilation time and whose size might change while running the program.

In order to support a mutable and growable piece of text in String typ, we need to allocate memory on the heap. This means:
- The memory must be requested from the memory allocator at runtime.
- We need to return this memory back to the allocator after the use of that String is done. 

The first is done by us when calling `String::from()`. However, the second part is done by the `Garbage Collector (GC)`, that keeps track of and cleans up memory that isn't being used anymore. 