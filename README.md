# a_ntree
Simple Tree datatype in Rust with a parent reference.

## Limitations
 This datatype has the following limitations:
* Each Value inside a `Node` is unique
* A `Node` cannot have children of different types

## Example
```rust norun ignore
use a_ntree::Node;

let root = Node::new(10);

root.add_child(&Node::new(20));
root.add_leaf(30);
root.add_leaf(40);

root.find(&30).unwrap().add_leaf(21);
```
creates this tree:
```norun
             Node(10)
           /    |    \
    Node(20) Node(30) Node(40)
                | 
             Node(21)
```