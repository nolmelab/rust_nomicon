# what unsafe rust can do 

The only things that are different in Unsafe Rust are that you can:

- Dereference raw pointers
- Call unsafe functions (including C functions, compiler intrinsics, and the raw allocator)
- Implement unsafe traits
- Mutate statics
- Access fields of unions

UB (Undefined Behavior)를 피하기 위한 것이다. 

Unlike C, Undefined Behavior is pretty limited in scope in Rust. All the core language 
cares about is preventing the following things:

- Dereferencing (using the * operator on) dangling or unaligned pointers (see below)
- Breaking the pointer aliasing rules
- Calling a function with the wrong call ABI or unwinding from a function with the wrong 
  unwind ABI.
- Causing a data race
- Executing code compiled with target features that the current thread of execution does not support
- Producing invalid values (either alone or as a field of a compound type such as enum/struct/array/tuple):
  - a bool that isn't 0 or 1
  - an enum with an invalid discriminant
  - a null fn pointer
  - a char outside the ranges [0x0, 0xD7FF] and [0xE000, 0x10FFFF]
  - a ! (all values are invalid for this type)
  - an integer (i*/u*), floating point value (f*), or raw pointer read from uninitialized 
    memory, or uninitialized memory in a str.
  - a reference/Box that is dangling, unaligned, or points to an invalid value.
  - a wide reference, Box, or raw pointer that has invalid metadata:
  - dyn Trait metadata is invalid if it is not a pointer to a vtable for Trait that 
    matches the actual dynamic trait the pointer or reference points to
  - slice metadata is invalid if the length is not a valid usize (i.e., it must not be 
    read from uninitialized memory)
  - a type with custom invalid values that is one of those values, such as a NonNull that 
    is null. (Requesting custom invalid values is an unstable feature, but some stable 
    libstd types, like NonNull, make use of it.)


aliasing: 
- variables and pointers alias if they refer to overlapping regions of memory.


