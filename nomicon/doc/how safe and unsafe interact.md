# how safe and unsafe interact

The unsafe keyword has two uses: to declare the existence of contracts the compiler can't 
check, and to declare that a programmer has checked that these contracts have been upheld.

This is the balance between Safe and Unsafe Rust. The separation is designed to make using 
Safe Rust as ergonomic as possible, but requires extra effort and care when writing Unsafe 
Rust. The rest of this book is largely a discussion of the sort of care that must be 
taken, and what contracts Unsafe Rust must uphold.

unsafe는 safe하게 만들기위한 노력의 표식이다. 



