# LinkedList

## Data Structure

A data structure that has a `list` of nodes referencing the next node in memory 
forming a list like structure.

## Rust

- `Rc` - Reference Counted Data Structure for a single threaded reference

Same as a C++ shared pointer, used to keep a reference of an object in-memory.

When the last reference is dropped, the object is dropped from memory.

- `RefCell` - Allows a mutable object using multiple shared references. Not thread safe.

# TODO

Explain acces to the refcell via rc.
