use std::fmt::{Display, Formatter};
use handle_rs::{Handle, HandleArray, IHandleArrayItem};

#[test]
pub fn handle_array_sample(){
    //new HandleArray
    let mut ha = HandleArray::<Item>::new(1000);
    //add_item
    let handle0 = ha.add_item(Item {value:0.1,handle:Handle::default()});
    let _ = ha.add_item(Item {value:0.2,handle:Handle::default()});
    let _ = ha.add_item(Item {value:0.3,handle:Handle::default()});
    println!("{}",ha.get(handle0));
    //alive_iter_mut
    println!("--change value--");
    for turtle in ha.alive_iter_mut() {
        turtle.1.value += 0.1;
    }
    //alive_iter
    for turtle in ha.alive_iter() {
        println!("{}", turtle.1);
    }
    //remove_item
    ha.remove_item(handle0);
    println!("--remove now---");
    for turtle in ha.alive_iter() {
        println!("alive {}", turtle.1);
    }
    println!("--add again---");
    let _ = ha.add_item(Item {value:0.5,handle:Handle::default()});
    let _ = ha.add_item(Item {value:0.6,handle:Handle::default()});
    for turtle in ha.alive_iter() {
        println!("alive {}", turtle.1);
    }
}



#[derive(Debug)]
struct Item{
    pub value:f64,
    pub handle:Handle,
}

impl IHandleArrayItem for Item {
    fn get_handle(&self) -> Handle {
        self.handle
    }

    fn set_handle(&mut self, handle: Handle) {
        self.handle = handle;
    }
}

impl Default for Item{
    fn default() -> Self {
        Item{
            value : 0.0,
            handle : Handle{index:0,generation:0},
        }
    }
}

impl Display for Item{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"value : {} , handle:{}",self.value,self.handle)
    }
}