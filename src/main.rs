
struct Node<T>{
    value:T,
    next_node: Option<Box<Node<T>>>,
}
impl<'a,T: std::fmt::Display> Node<T>{
    fn mut_incr(&mut self) -> Option<&mut Node<T>>{
        if self.next_node.is_some(){
            return Some(&mut *(self.next_node.as_mut().unwrap()));}
            return None;
    }
    fn incr(&self) -> Option<&Node<T>>{
        //make sure it is a valid node to return
        if self.next_node.is_some(){
            return Some(&*(self.next_node.as_ref().unwrap()));
        }
        return None
    }
    fn print(&self){
        print!("{}",self.value);
    }
    fn println(&self){
        println!("{}",self.value);
    }
    fn add_node(&mut self,value: T){
        let new_node = Node{value,next_node:None};
        //should transfer ownership
        self.next_node =  Some(Box::new(new_node));
    }
}
struct StaticLinkedList<T>{
    root:Option<Node<T>>,
    size:u32,
}
impl<'a,T: std::fmt::Display> StaticLinkedList<T>{
    fn push(&mut self,val:T){
        if self.root.is_none(){
          self.root = Some(Node{value:val,next_node:None});
          self.size = 1;
        }else{
          //loops through linked list and adds node to end
          let mut node = self.root.as_mut().unwrap();
          while node.mut_incr().is_some(){
              node = node.mut_incr().unwrap();
          }
          node.add_node(val);
          self.size+=1;
        }
      }
    fn print(&self){
        print!("[");
        let mut binding = Some(self.root.as_ref().unwrap());
        while binding.is_some(){
            //print out this value then increment if increment not none then print out ,
            binding.unwrap().print();
            
            binding = binding.unwrap().incr();
            if binding.is_some(){
                print!(",");
            }
        }
        println!("]");
    }
}
fn main() {
    let node2 = Node{value:2,next_node:None};
    let mut node1 = Node{value:1,next_node:Some(Box::new(node2))};

    //try printing out the value of node2 using node 1.
    //first increment node1. his should return an option with a node pointer
    //then unwrap the node ref and try printing
    node1.incr().unwrap().println();

    //should add a 3rd node with the value of 5
    node1.mut_incr().unwrap().add_node(5);
    //should print out the new value
    node1.incr().unwrap().incr().unwrap().println();
    //i know want to make it easier to mutate these values.

    //we will now move node 1 into the linked list
    let mut new_list = StaticLinkedList::<i32>{root:None,size:0};
    new_list.push(5);
    new_list.push(7);
    new_list.push(8);
    println!("the size of the linked list is: {}",new_list.size);
    //should print out all values
    new_list.print();
    println!("is this on a new line")
}
