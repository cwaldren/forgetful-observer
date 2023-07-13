## forgetful observer

This crate allows you to track items seen during execution of an algorithm using the RAII technique. 

An observation of a particular item is represented by an object called `Observation`. When it falls
out of scope, the item is forgotten.

This might be useful when implementing a recursive algorithm on a graph that must detect cycles.

Here's an example:

```rust
let observer = Observer::new();
{
    let observation = observer.notice("foo").expect("never seen before");
    // While 'observation' is in scope, subsequent calls to notice return None.
    assert!(observer.notice("foo").is_none());
}
// Now that 'observation' is out of scope, this will return Some(Observation).
assert!(observer.notice("foo").is_some());
```

The `Observer` can track any item that is `Eq + Hash`. For example:
```rust
observer.notice(&42);
```

Internally, `Observer` stores references to the items
it notices in a `HashSet`. Upon destruction of an `Observation`, the item reference is removed from the set. 