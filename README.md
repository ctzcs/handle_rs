# handle_rs

Handle-based array with free-list and alive iteration.

## Features
- Sentinel at index `0` to represent invalid/null handle
- Free-list reuse to avoid reallocations
- `alive_iter` / `alive_iter_mut` filter out invalid entries for safe iteration

## Usage
```rust
use std::fmt::{Display, Formatter};
use handle_rs::{Handle, HandleArray, IHandleArrayItem};

#[derive(Debug)]
struct Item {
    pub value: f64,
    pub handle: Handle,
}

impl IHandleArrayItem for Item {
    fn get_handle(&self) -> Handle { self.handle }
    fn set_handle(&mut self, handle: Handle) { self.handle = handle; }
}

impl Default for Item {
    fn default() -> Self {
        Item { value: 0.0, handle: Handle { index: 0, generation: 0 } }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "value: {} , handle: {}", self.value, self.handle)
    }
}

fn main() {
    let mut ha = HandleArray::<Item>::new(1000);
    let h0 = ha.add_item(Item { value: 0.1, handle: Handle::default() });
    let _ = ha.add_item(Item { value: 0.2, handle: Handle::default() });
    println!("{}", ha.get(h0));

    for (_, item) in ha.alive_iter_mut() {
        item.value += 0.1;
    }

    for (_, item) in ha.alive_iter() {
        println!("alive {}", item);
    }

    ha.remove_item(h0);
    for (_, item) in ha.alive_iter() {
        println!("alive {}", item);
    }
}
```