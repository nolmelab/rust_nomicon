# alternative representations

## repr(C)

## repr(transparent)

```rust
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
```
UnsafeCell can be transmuted to T. 



