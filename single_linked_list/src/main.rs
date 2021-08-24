use core::fmt::Display;

#[derive(Debug)]
struct SingleLinkedList<T> {
    // We need to own the Box here since this is the variable that refers to the allocated value,
    // As we keep allocating via Box::new(Node::new()), we need to move the value into some variable.
    head: Option<Box<Node<T>>>,
    len: u32,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T :Display + Clone> Node<T> 
{
    fn new(value: T) -> Self {
        Node{data : value,
             next : None}
    }
}

impl<T : Display + Clone> SingleLinkedList<T> {
    fn new() -> Self {
        SingleLinkedList{head : None, 
                         len : 0}
    }

    fn push(&mut self, val: T) {
    	match self.head.as_mut() {
	    Some(mut node) => {
	    	let mut count = self.len;

		while count > 1 {
		    node = (*node).next.as_mut().unwrap();
		    count = count - 1;
		}

		(*node).next = Some(Box::new(Node::new(val)));
	    },
            None => {
		//println!("Pushing first value {}", val);
		self.head = Some(Box::new(Node::new(val)));
	    },
	}
	self.len = self.len + 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
  
       let mut node = self.head.as_mut().unwrap();
       let mut count = self.len;

       while count > 2 {
           if (*node).next.is_none() { 
               break;
           }
           node = (*node).next.as_mut().unwrap();
           count = count - 1;
       } 
       let result : T;
       if (*node).next.is_none() {
           result = (*node).data.clone();
       } else {
           result = (*node).next.as_ref().unwrap().data.clone();
       }
       (*node).next = None;
       self.len = self.len - 1;
       return Some(result)
    }

    fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }

        let mut node = self.head.as_ref().unwrap();
        let mut count = self.len;

        while count > 1 {
            node = (*node).next.as_ref().unwrap();
            count = count - 1;
        }

        return Some(&((*node).data))
    }
    
    fn is_empty(&self) -> bool {
         if self.len == 0 {
	     return true;
         }
         return false;
    }

    fn print_list(&self) {
    	match self.head.as_ref() {
	    Some(mut temp) => {
		println!("Current list: ");
                let mut count = self.len;
                loop {
		    println!("Node: {}", (*temp).data);
                    count = count - 1;
		    if count == 0 {
			break;
		    }
		    temp = (*temp).next.as_ref().unwrap();
		}
	    },

            None => {println!("The list is empty");},
	}
    }

    fn rev(self) -> SingleLinkedList<T> {
        let vec_list : Vec<T>= self.into();
        SingleLinkedList::from(vec_list)
    }
}

impl<T: Clone + Display> From<Vec<T>> for SingleLinkedList<T> {
    fn from(input: Vec<T>) -> Self {
         let mut list = SingleLinkedList::new();
         // iter - iterates over &T
         // iter_mut - iterates over &mut T
         // into_iter - iterates over T
         for it in input.iter() {
             list.push(it.clone());
         }
         list
    }
}

impl<T: Clone + Display> Into<Vec<T>> for SingleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
       let mut res = vec![];
       while let Some(t) = self.pop().take() {
          res.push(t);
       }
       res
    }
}

fn main() {
    let mut list = SingleLinkedList::new();
    list.print_list();

    list.push(1);
    list.push(2);
    list.push(3);
    list.print_list();
    let x = list.peek();
    match x {
        Some(&x) => {println!("peeked: {}", x)},
        None => {println!("List was empty");},
    };

    list.pop();
    list.pop();
    list.print_list();
    
    let y = list.peek(); 
    match y {
        Some(&x) => {println!("peeked: {}", x)},
        None => {println!("List was empty");},
    };
    
    list.push(4);
    list.push(5);
    list.print_list();

    let new_list = list.rev();
    new_list.print_list();
}

