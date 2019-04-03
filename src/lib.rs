mod forward_list;
use forward_list::ForwardList;
use std::mem::swap;
use std::iter::{FromIterator, IntoIterator};

#[derive(Debug)]
pub enum PairingHeap<T: PartialOrd> {
  Empty,
  Head(ParingNode<T>)
}
use PairingHeap::*;

#[derive(Debug)]
pub struct ParingNode<T: PartialOrd> {
  val: T,
  children: ForwardList<ParingNode<T>>,
}

impl <T: PartialOrd> ParingNode<T> {
  pub fn add_child(&mut self, n: ParingNode<T>) {
    self.children.push_back(n);
  }
}

impl<T: PartialOrd> PairingHeap<T> {
  pub fn get_minium(&self) -> Option<&T> {
    match self {
      Empty => None,
      Head(node) => Some(&node.val)
    }
  }

  pub fn merge(&mut self, other: PairingHeap<T>) {
    if let Head(ref mut lnode) = self {
      if let Head(mut rnode) = other {
        if lnode.val > rnode.val {
          swap(lnode, &mut rnode);
        }
        lnode.add_child(rnode);
      }
    } else {
      *self = other;
    }
  }

  pub fn merge_pairs(mut l: ForwardList<ParingNode<T>>) -> Self {
    loop {
      if let Some(mut first) = l.pop_front_node() {
        if let Some(mut second) = l.pop_front_node() {
          if first.val.val > second.val.val {
            swap(&mut first, &mut second);
          }
          first.val.add_child(second.val);
          l.push_back_node(first);
        } else {
          return Head(first.val)
        }
      } else {
        return Empty
      }
    }
  }

  pub fn pop_min(&mut self) -> Option<T> {
    let mut other = Empty;
    swap(self, &mut other);
    if let Head(node) = other {
      *self = PairingHeap::merge_pairs(node.children);
      Some(node.val)
    } else {
      None
    }
  }

  pub fn insert(&mut self, mut t: T) {
    if let Head(node) = self {
      if t < node.val {
        swap(&mut node.val, &mut t);
      }
      node.children.push_back(ParingNode{val: t, children: ForwardList::default()});
    } else {
      *self = Head(ParingNode{val: t, children: ForwardList::default()})
    }
  }

  pub fn of(t: T) -> Self {
    Head(ParingNode{
      val: t,
      children: ForwardList::default()
    })
  }

  pub fn new() -> Self {
    Self::default()
  }
}

impl<T: PartialOrd> Default for PairingHeap<T> {
  fn default() -> Self {
    Empty
  }
}

pub struct IntoIter<T: PartialOrd> {
  heap: PairingHeap<T>
}

impl<T: PartialOrd> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.heap.pop_min()
  }
}

impl<T: PartialOrd> IntoIterator for PairingHeap<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;
  #[inline]
  fn into_iter(self) -> IntoIter<T> {
    IntoIter { heap: self }
  }
}

impl<T: PartialOrd> FromIterator<T> for PairingHeap<T> {
  fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = T> {
    PairingHeap::merge_pairs(iter.into_iter().map(|val| ParingNode{val, children: ForwardList::default()}).collect())
  }
}

 #[test]
 fn test() {
   let mut a = PairingHeap::default();
   let mut b = PairingHeap::default();
   a.insert(5);
   a.insert(2);
   a.insert(3);
   b.insert(7);
   b.insert(1);
   b.insert(4);
   a.merge(b);
   assert!(a.get_minium().unwrap() == &1);
   let x: Vec<_> = a.into_iter().collect();
   assert!(x == vec![1,2,3,4,5,7]);
 }
