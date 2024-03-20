# 2 A Back Stack
## 2. 1Layout
One important thing to keep in mind during defining of Linked Lists
is the notion of a null-pointer-optimization. If we have an enum
which has a null-ptr value (for example `Empty`) then Rust can be
more efficient about memoery layout, because this `Empty` enum
value is represented by all zeroes.

Something like that, I'm not entirely clear on it yet.

If we have a Struct with exactly one Data type, we have a zero-
cost abstraction as that Struct will have the size of its one data
type. For example this allows us to make a list public without
having to make the nodes public by wrapping them with a public struct

