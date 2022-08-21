use std::mem;
pub fn yes(){
    let mut list = LinkedList::new("This");
    println!("{:?}", list.head);

    list.push("is");
    println!("{:?}", &list)
}

#[derive(Debug)]
pub struct LinkedListNode< T> {
    data: T,
    next: Link<T>
}

type Link<T> = Option<Box<LinkedListNode<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn head(&self) -> &T {
        match &self.head {
            None => {
                panic!("Head of empty new_list")
            }

            Some(node) => {
               &node.data
            }
        }
    }

    pub fn head_option(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
   }

    pub fn new(new_data: T) -> LinkedList<T> {
        let new_node = LinkedListNode{
            data: new_data,
            next: None,
        };
        LinkedList{
            head: Some(Box::new(new_node))
        }
    }

    pub fn push(&mut self, new_data: T) {
       let new_node = LinkedListNode { 
           data: new_data, 
           next: mem::replace(&mut self.head, None), 
       }; 

       self.head = Some(Box::new(new_node)) 
    }

    pub fn last(&self) -> &LinkedListNode<T> {
        match &self.head {
           None => {
               panic!("Empty list")
           }
           Some(node) => {
             node.last() 
        }
    }
    }
    pub fn last_option(&self) -> Option<&LinkedListNode<T>> {
        match &self.head {
           None => {
               None
           }
           Some(node) => {
             Some(node.last())
          }
        }
    }
    /*
    pub fn append(&mut self, new_data: T) {
        if self.head.is_some() {
            let new_node = LinkedListNode{
                data: new_data,
                next: None,
            };
            self.head.unwrap


        } else {
                 let new_node = LinkedListNode{
                    data: new_data,
                    next: None
                };
                self.head = Some(Box::new(new_node))           
        }
  }
  */
     /* pub fn map<V>(&self,f: fn(T) -> V) -> LinkedList<V> {
       match self.head {
          case  
       } 
     }
     */
}

impl<T> LinkedListNode<T> {
    /*
    pub fn append(&mut self, new_data: T) {
        if self.next.is_none() {
            let new_node = LinkedListNode{
                data: new_data,
                next: None 
            };
            self.next= Some(Box::new(new_node));

            } else {
                self.next.unwrap().append(new_data)
            }
    }
*/
    pub fn last(&self) -> &LinkedListNode<T> {
        match &self.next {
            None => self,
            Some(node) => node.last()
        }
    }
}

#[cfg(test)]
mod linked_link_tests {
    use super::*;

    fn e1() -> String {"this".to_string()}
    fn e2() -> String {"is".to_string()}
//    fn e3() -> String {"great".to_string()}

    #[test]
    fn new_list_and_head(){
        let list = LinkedList::new(e1());
        assert_eq!(list.head(), &e1())
    }
    
    #[test]
    fn head_option_some(){
        let list = LinkedList::new(e1());
        assert_eq!(list.head_option(), Some(&e1()))
    }

    #[test]
    fn head_option_none(){
        let list: LinkedList<String> = LinkedList{
            head: None 
        };
        assert_eq!(list.head_option(), None)
    }

    /*
    #[test]
    fn append_to_list(){
        let mut list = LinkedList::new(e1());
        list.append(e2());
        println!("{:?}", list)
    }
    */
}

