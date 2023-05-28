use a_ntree::Node;

#[test]
fn adding() {
    let root = Node::new(10);
    {
        let node = Node::new(20);
        let _ = root.add_child(&node);
        // 1 in curly braces (node variable)
        // 1 in root as child in children
        assert_eq!(node.rc_count(), 2);
    }
    assert_eq!(root.rc_count(), 1);
}

#[test]
fn get_root() {
    let root = Node::new(10);
    {
        let _got = root.get_root();
        assert_eq!(root.rc_count(), 2);
    }
    assert_eq!(root.rc_count(), 1);
}

#[test]
fn deleting() {
    let root = Node::new(10);
    let a = Node::new(30);

    {
        let child = Node::new(20);
        let _ = child.add_child(&a);
        let _ = root.add_child(&child);
    }

    assert_eq!(a.rc_count(), 2);
    root.remove_node(&20);
    assert_eq!(a.rc_count(), 1);
}