use pairing_heap::PairingHeap;

fn main() {
  let x = vec![5,2,1,7,4,6,9,2];
  let h: PairingHeap<_> = x.into_iter().collect();
  let y: Vec<_> = h.into_iter().collect();
  println!("{:?}", y);
}
