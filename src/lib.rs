use core::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Eq;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub struct Observation<'a, T>
where
    T: 'a + Eq + Hash + ?Sized,
    &'a T: Borrow<T>,
{
    item: &'a T,
    recorder: Rc<RefCell<HashSet<&'a T>>>,
}

impl<'a, T> Debug for Observation<'a, T>
where
    T: 'a + Eq + Hash + ?Sized + Debug,
    &'a T: Borrow<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item)
    }
}

impl<'a, T> Observation<'a, T>
where
    T: Eq + Hash + ?Sized,
    &'a T: Borrow<T>,
{
    pub(crate) fn new(recorder: Rc<RefCell<HashSet<&'a T>>>, item: &'a T) -> Self {
        recorder.borrow_mut().insert(item);
        Self { item, recorder }
    }
}

impl<'a, T> Drop for Observation<'a, T>
where
    T: Eq + Hash + ?Sized,
    &'a T: Borrow<T>,
{
    fn drop(&mut self) {
        self.recorder.borrow_mut().remove(self.item);
    }
}

/**
Observer records observations of values of type T. It reports
whether an item was seen before.

This is useful when implementing an algorithm that must
ensure items are encountered only once.

Observations are scoped; when they fall out of scope,
the Observer forgets about them.
```
use forgetful::Observer;
let observer = Observer::new();
{
    let observation = observer.notice("foo").expect("never seen before");
    // While 'observation' is in scope, subsequent calls to notice return None.
    assert!(observer.notice("foo").is_none());
}
// Now that 'observation' is out of scope, this will return Some(Observation).
assert!(observer.notice("foo").is_some());
```
*/
pub struct Observer<'a, T>
where
    T: 'a + Eq + Hash + ?Sized,
    &'a T: Borrow<T>,
{
    recorder: Rc<RefCell<HashSet<&'a T>>>,
}

impl<'a, T> Default for Observer<'a, T>
where
    T: 'a + Eq + Hash + ?Sized,
    &'a T: Borrow<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Debug for Observer<'a, T>
where
    T: 'a + Eq + Hash + ?Sized + Debug,
    &'a T: Borrow<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        RefCell::borrow(&self.recorder).fmt(f)
    }
}

impl<'a, T> Observer<'a, T>
where
    T: 'a + Eq + Hash + ?Sized,
    &'a T: Borrow<T>,
{
    pub fn new() -> Self {
        Self {
            recorder: Default::default(),
        }
    }

    pub fn notice(&self, item: &'a T) -> Option<Observation<'a, T>> {
        if RefCell::borrow(&self.recorder).contains(item) {
            None
        } else {
            Some(Observation::new(Rc::clone(&self.recorder), item))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forgets_immediately_with_no_observer() {
        let o = Observer::new();
        assert!(o.notice(&1).is_some());
        assert!(o.notice(&1).is_some());
    }

    #[test]
    fn does_not_forget_if_observer_in_scope() {
        let o = Observer::new();
        let _g = o.notice(&1);
        assert!(o.notice(&1).is_none());
    }

    #[test]
    fn unique_items_noticed_independently() {
        let o = Observer::new();
        let g1 = o.notice(&1);
        let g2 = o.notice(&2);
        assert!(g1.is_some());
        assert!(g2.is_some());
    }

    #[test]
    fn nested_scopes() {
        let o = Observer::new();
        {
            let g1 = o.notice(&1);
            assert!(g1.is_some());
            assert!(o.notice(&1).is_none());
            {
                let g2 = o.notice(&2);
                assert!(g2.is_some());
                assert!(o.notice(&1).is_none());
                assert!(o.notice(&2).is_none());
                {
                    let g3 = o.notice(&3);
                    assert!(g3.is_some());
                    assert!(o.notice(&1).is_none());
                    assert!(o.notice(&2).is_none());
                    assert!(o.notice(&3).is_none());
                }
                assert!(o.notice(&3).is_some());
            }
            assert!(o.notice(&2).is_some());
            assert!(o.notice(&3).is_some());
        }
        assert!(o.notice(&1).is_some());
        assert!(o.notice(&2).is_some());
        assert!(o.notice(&3).is_some());
    }
}
