# SegmentTree.rs
Generic segment tree implementation in Rust.

## Generic?
This segment tree allows the user to define it's "aggregation" operation. In other words, the tree's internal nodes can be the result of any operation between its children, e.g.: sum, max, min, min & max, max and second max, etc...

See the `examples` folder to learn how to implement an `Aggregator`.
