use std::fmt::{Display, Formatter};

/**
Handle-based array with free-list and alive iteration.

Notes:
- Index 0 will be used to check whether if it is valid；null will be `get_handle().idx == 0`
- `alive_iter/alive_iter_mut` will filter handle with idx==0, then you can iterate item which is valid
# example
```ignore
use std::fmt::{Display, Formatter};
use handle_rs::{Handle, HandleArray, IHandleArrayItem};

pub fn handle_array_sample(){
    //new HandleArray
    let mut ha = HandleArray::<Item>::new(1000);
    //add_item
    let handle0 = ha.add_item(Item {value:0.1,handle:Handle::default()});
    let handle1 = ha.add_item(Item {value:0.2,handle:Handle::default()});
    let handle2 = ha.add_item(Item {value:0.3,handle:Handle::default()});
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
    let handle3 = ha.add_item(Item {value:0.5,handle:Handle::default()});
    let handle4 = ha.add_item(Item {value:0.6,handle:Handle::default()});
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

```
*/

//region Handle
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Handle{
    pub index: usize,
    pub generation: usize,
}

impl Display for Handle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[index:{},gen:{}]", self.index, self.generation)
    }
}

impl Default for Handle{
    fn default() -> Self {
        Handle{ index:0, generation:0}
    }
}

//endregion

//region HandleArray
pub trait IHandleArrayItem{
    fn get_handle(&self)->Handle;
    fn set_handle(&mut self,handle:Handle);
}

pub struct HandleArray<T:IHandleArrayItem>
{
    data: Vec<T>,
    free_list:Vec<Handle>,
    alive:usize,
}

impl<T:IHandleArrayItem + Default> HandleArray<T>{
    pub fn new(capacity:usize) -> HandleArray<T>{
        HandleArray{
            data:Vec::with_capacity(capacity),
            free_list:Vec::with_capacity(capacity),
            alive:0,
        }
    }
    pub fn add_item(&mut self, item:T)-> Handle{
        let mut item = item;
        if self.free_list.len() > 0 {
            let handle = self.free_list.pop().unwrap();
            item.set_handle(handle);
            self.data[handle.index]=item;
            self.alive += 1;
            return handle;
        }

        if self.alive == 0{
            //推一个空的进去，哨兵位
            self.data.push(T::default());
        }
        let h = Handle{ index:self.alive+1, generation:0};
        item.set_handle(h);
        self.data.push(item);
        self.alive += 1;
        h
    }
    pub fn remove_item(&mut self, handle:Handle){
        //todo 如果在范围内直接移除
        self.data[handle.index] = T::default();
        let mut handle = handle;
        handle.generation += 1;
        self.free_list.push(handle);
        self.alive -=1;

    }

    pub fn get(&self,handle: Handle)->&T{
        &self.data[handle.index]
    }

    pub fn get_mut(&mut self,handle: Handle)->&mut T{
        &mut self.data[handle.index]
    }

    pub fn iter(&self) -> std::slice::Iter<'_,T>{
        self.data.iter()
    }


    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_,T>{
        self.data.iter_mut()
    }

    //利用切片构造
    pub fn alive_iter(&self)->HandleArrayIter<'_,T>{
        HandleArrayIter{
            data:&self.data,
            index:0,
        }
    }

    pub fn alive_iter_mut(&mut self)->HandleArrayIterMut<'_,T>{
        HandleArrayIterMut{
            data:self.data.iter_mut(),
            index:0,
        }
    }


}

pub struct HandleArrayIter<'a,T:IHandleArrayItem>
{
    data:&'a [T],
    index:usize,
}

//利用IterMut可写迭代构造
pub struct HandleArrayIterMut<'a,T:IHandleArrayItem>
{
    data:std::slice::IterMut<'a, T>,
    index:usize,
}

impl<'a,T:IHandleArrayItem> Iterator for HandleArrayIter<'a,T>{
    type Item = (usize,&'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            let i = self.index;
            self.index += 1;
            if i== 0{
                //跳过哨兵
                continue;
            }

            let item = &self.data[i];
            //默认handle的index为0
            if item.get_handle().index == 0 {
                continue;
            }
            return Some((i,item));
        }
        None
    }
}


impl<'a, T:IHandleArrayItem> Iterator for HandleArrayIterMut<'a, T>{
    type Item = (usize,&'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.data.next(){
            let i = self.index;
            self.index += 1;
            if i== 0{
                continue;
            }
            if item.get_handle().index == 0{
                continue;
            }
            return Some((i,item));
        }
        None
    }
}
//endregion


