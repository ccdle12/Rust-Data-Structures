# LinkedList

## Data Structure

A data structure that has a `list` of nodes referencing the next node in memory 
forming a list like structure.

## Rust

- `Rc` - Reference Counted Data Structure for a single threaded reference

Same as a C++ shared pointer, used to keep a reference of an object in-memory.

When the last reference is dropped, the object is dropped from memory.

- `RefCell` - Allows a mutable object using multiple shared references. Not thread safe.

# Low Level Explanations

Explain acces to the refcell via rc.

## Accessing the NodeRef<T>

So each Node holds a reference to a "next" node.

This is aliased as a `NodeRef<T>`.

The `NodeRef<T>` is really `Option<Rc<RefCell<Node<T>>>>`.

To access the Node, we need to unwrap the option, call `borrow_mut()` which is
a function trait implementation and that will return the inner value Node<T>.

# When to use LinkedLists?

Pros:
- Dynamically sized list, we don't have to predefine the size of the list.
- No shifting is required
- Queues and Stacks use LinkedLists

Cons:
- Getting a node by index is inefficient - O(n)
- Heap allocation
