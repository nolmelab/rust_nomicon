# exotically sized types 

## DST (Dynamically sized tpyes)

There are two major DSTs exposed by the language:

- trait objects: dyn MyTrait
- slices: [T], str, and others

The information that completes a trait object pointer is the vtable pointer. The runtime 
size of the pointee can be dynamically requested from the vtable.

A slice is simply a view into some contiguous storage -- typically an array or Vec. The 
information that completes a slice pointer is just the number of elements it points to.

## Zero sized types (ZST)

흥미로운 사용처가 있다. 컴파일 할 때만 존재하는가? 아마도.


## Empty types 

enum Void {}







