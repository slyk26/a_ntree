# a_ntree
Simple Tree datatype in Rust with a parent reference.

## Limitations
 This datatype has the following limitations:
* Each `Node` is unique
* A `Node` cannot have children of different types
* not threadsafe 

## Example
```rust
use a_ntree::Node;

fn main() {
    let root = Node::new(10);

    root.add_child(&Node::new(20));
    root.add_leaf(30);
    root.add_leaf(40);

    root.find(&30).unwrap().add_leaf(21);
}
```
creates this tree:
``` notrun
             Node(10)
           /    |    \
    Node(20) Node(30) Node(40)
                | 
             Node(21)
```