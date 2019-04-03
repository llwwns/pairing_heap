use std::mem::swap;
use std::ptr::NonNull;
use std::fmt;
use std::marker::PhantomData;
use std::iter::{FromIterator, IntoIterator};

pub struct Node<T> {
  pub val: T,
  next: Option<NonNull<Node<T>>>
}

pub struct ForwardList<T> {
  head: Option<NonNull<Node<T>>>,
  tail: Option<NonNull<Node<T>>>,
  marker: PhantomData<Box<Node<T>>>
}

impl<T> Default for ForwardList<T> {
  #[inline]
  fn default() -> Self {
    ForwardList{ head: None, tail: None, marker: PhantomData::default() }
  }
}

impl<T> ForwardList<T> {
  pub fn append(&mut self, other: &mut ForwardList<T>) {
    if let Some(ref mut tail) = self.tail {
      unsafe {
        tail.as_mut().next = other.head;
        self.tail = other.tail;
        other.head = None;
        other.tail = None;
      }
    } else {
      swap(self, other);
    }
  }

  #[inline]
  pub fn front(&self) -> Option<&T> {
    unsafe {
      self.head.as_ref().map(|node| &node.as_ref().val)
    }
  }

  #[inline]
  pub fn front_mut(&mut self) -> Option<&mut T> {
    unsafe {
      self.head.as_mut().map(|node| &mut node.as_mut().val)
    }
  }

  pub fn back(&mut self) -> Option<&T> {
    unsafe {
      self.tail.as_mut().map(|node| &node.as_ref().val)
    }
  }

  pub fn back_mut(&mut self) -> Option<&mut T> {
    unsafe {
      self.tail.as_mut().map(|node| &mut node.as_mut().val)
    }
  }

  pub fn push_front(&mut self, elt: T) {
    if let Some(head) = self.head {
      unsafe {
        let n = Box::new(Node{ val: elt, next: Some(head) });
        let n = Some(NonNull::new_unchecked(Box::into_raw(n)));
        self.head = n
      }
    } else {
      unsafe {
        let n = Box::new(Node{ val: elt, next: None });
        let n = Some(NonNull::new_unchecked(Box::into_raw(n)));
        self.tail = n;
        self.head = n;
      }
    }
  }

  pub fn push_back(&mut self, elt: T) {
    unsafe {
      let n = Box::new(Node{ val: elt, next: None });
      let n = Some(NonNull::new_unchecked(Box::into_raw(n)));
      if let Some(mut tail) = self.tail {
        tail.as_mut().next = n;
        self.tail = n;
      } else {
        self.tail = n;
        self.head = n;
      }
    }
  }

  pub fn push_front_node(&mut self, mut node: Box<Node<T>>) {
    if let Some(head) = self.head {
      node.next = Some(head);
      unsafe {
        self.head = Some(NonNull::new_unchecked(Box::into_raw(node)));
      }
    } else {
      node.next = None;
      unsafe {
        self.head = Some(NonNull::new_unchecked(Box::into_raw(node)));
      }
      self.tail = self.head;
    }
  }

  pub fn push_back_node(&mut self, mut node: Box<Node<T>>) {
    unsafe {
      node.next = None;
      let node = Some(NonNull::new_unchecked(Box::into_raw(node)));
      if let Some(mut tail) = self.tail {
        tail.as_mut().next = node;
        self.tail = node;
      } else {
        self.head = node;
        self.tail = node;
      }
    }
  }

  pub fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
    unsafe {
      if let Some(head) = self.head {
        self.head = head.as_ref().next;
        if self.head.is_none() {
          self.tail = None;
        }
        Some(Box::from_raw(head.as_ptr()))
      } else {
        None
      }
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    self.pop_front_node().map(|node| node.val)
  }

  pub fn iter(&self) -> Iter<T> {
    Iter { current: self.head, marker: PhantomData::default() }
  }

  pub fn iter_mut(&self) -> IterMut<T> {
    IterMut { current: self.head, marker: PhantomData::default() }
  }

  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  pub fn new() -> Self {
    Self::default()
  }
}

pub struct Iter<'a, T> {
  current: Option<NonNull<Node<T>>>,
  marker: PhantomData<&'a Box<Node<T>>>,
}

pub struct IterMut<'a, T> {
  current: Option<NonNull<Node<T>>>,
  marker: PhantomData<&'a mut Box<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<&'a T> {
    if let Some(current) = self.current {
      unsafe {
        let node = &*current.as_ptr();
        self.current = node.next;
        Some(&node.val)
      }
    } else {
      None
    }
  }
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;
  fn next(&mut self) -> Option<&'a mut T> {
    if let Some(current) = self.current {
      unsafe {
        let node = &mut *current.as_ptr();
        self.current = node.next;
        Some(&mut node.val)
      }
    } else {
      None
    }
  }
}

impl<T: fmt::Debug> fmt::Debug for ForwardList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "ForwardList[")?;
      for x in self.iter() {
        write!(f, "{:?}, ", x)?;
      }
      write!(f, "]")?;
      Ok(())
    }
}

impl<T> Drop for ForwardList<T> {
  fn drop(&mut self) {
    while let Some(_) = self.pop_front_node() {};
  }
}

impl<T> FromIterator<T> for ForwardList<T> {
  fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = T> {
      let mut iter = iter.into_iter();
      let mut l = ForwardList::default();
      while let Some(item) = iter.next() {
        l.push_back(item);
      }
      l
  }
}

impl<T: Clone> Clone for ForwardList<T> {
  #[inline]
  fn clone(&self) -> Self {
    self.iter().map(|item| item.clone()).collect()
  }
}

pub struct IntoIter<T> {
  list: ForwardList<T>
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop_front()
  }
}

impl<T> IntoIterator for ForwardList<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;
  #[inline]
  fn into_iter(self) -> IntoIter<T> {
    IntoIter { list: self }
  }
}

impl<'a, T> IntoIterator for &'a ForwardList<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;
  fn into_iter(self) -> Iter<'a, T> {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut ForwardList<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;
  fn into_iter(self) -> IterMut<'a, T> {
    self.iter_mut()
  }
}

#[test]
fn test() {
  let mut a = ForwardList::default();
  let mut b = ForwardList::default();
  a.push_back(5);
  a.push_back(2);
  a.push_front(3);
  b.push_front(7);
  b.push_back(1);
  b.push_back(4);
  a.append(&mut b);
  let n = a.pop_front_node().unwrap();
  a.push_back_node(n);
  {
    let x: Vec<_> = a.iter().collect();
    assert!(x == vec![&5,&2,&7,&1,&4,&3]);
  }
  b = a.clone();
  let mut x: Vec<_> = a.into_iter().collect();
  assert!(x == vec![5,2,7,1,4,3]);
  let x: Vec<_> = (&b).into_iter().collect();
  assert!(x == vec![&5,&2,&7,&1,&4,&3]);
  let x: Vec<_> = (&mut b).into_iter().collect();
  assert!(x == vec![&mut 5,&mut 2,&mut 7,&mut 1,&mut 4,&mut 3]);
}

