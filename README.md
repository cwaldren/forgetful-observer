## forgetful observer

This crate allows you to track items seen during execution of an algorithm using [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization).

An observation of a particular item is represented by an `Obervation<T>`. When this object falls
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
it notices in a `HashSet`. 

Upon the `Observation`'s destruction, the item reference is removed from the set. 