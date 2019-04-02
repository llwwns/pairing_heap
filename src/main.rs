mod forward_list;
use forward_list::ForwardList;
use std::mem::swap;

#[derive(Debug)]
enum Heap<T: PartialOrd> {
  Empty,
  Head(ParingNode<T>)
}
use Heap::*;

#[derive(Debug)]
struct ParingNode<T: PartialOrd> {
  val: T,
  children: ForwardList<ParingNode<T>>,
}

impl <T: PartialOrd> ParingNode<T> {
  fn add_child(&mut self, n: ParingNode<T>) {
    self.children.push_back(n);
  }
}

impl<T: PartialOrd> Heap<T> {
  fn get_minium(&self) -> Option<&T> {
    match self {
      Empty => None,
      Head(node) => Some(&node.val)
    }
  }

  fn merge(&mut self, other: Heap<T>) {
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

  fn merge_pairs(mut l: ForwardList<ParingNode<T>>) -> Self {
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

  fn pop_min(&mut self) -> Option<T> {
    let mut other = Empty;
    swap(self, &mut other);
    if let Head(node) = other {
      *self = Heap::merge_pairs(node.children);
      Some(node.val)
    } else {
      None
    }
  }

  fn insert(&mut self, mut t: T) {
    if let Head(node) = self {
      if t < node.val {
        swap(&mut node.val, &mut t);
      }
      node.children.push_back(ParingNode{val: t, children: ForwardList::default()});
    } else {
      *self = Head(ParingNode{val: t, children: ForwardList::default()})
    }
  }

  fn of(t: T) -> Self {
    Head(ParingNode{
      val: t,
      children: ForwardList::default()
    })
  }
}

impl<T: PartialOrd> Default for Heap<T> {
  fn default() -> Self {
    Empty
  }
}

impl<T: PartialOrd> Iterator for Heap<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.pop_min()
  }
}



 #[test]
 fn test() {
   let mut a = Heap::default();
   let mut b = Heap::default();
   a.insert(5);
   a.insert(2);
   a.insert(3);
   b.insert(7);
   b.insert(1);
   b.insert(4);
   a.merge(b);
   println!("{:?}", a);
   assert!(a.get_minium().unwrap() == &1);
   let x: Vec<_> = a.collect();
   println!("{:?}", x);
   assert!(x == vec![1,2,3,4,5,7]);
 }

fn main() {
  let mut a = ForwardList::default();
  let mut b = ForwardList::default();
  a.push_back(5);
  a.push_back(2);
  a.push_front(3);
  b.push_front(7);
  b.push_back(1);
  b.push_back(4);
  a.append(&mut b);
  println!("{:?}", a);
  println!("{:?}", b);
}
