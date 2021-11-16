//create a linkedList
#[derive(Debug,Clone)]
pub struct LinkedList<T> (Option<(T,Box<LinkedList<T>>)>);

impl <T:PartialOrd + Clone> LinkedList<T> { 
    #[allow(dead_code)]
    fn new() -> Self { 
        LinkedList(None)
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self,data:T) { 
         let t = self.0.take();
         self.0 = Some((data,Box::new(LinkedList(t))));

    }
    #[allow(dead_code)]
    pub fn push_back(&mut self, data:T)  { 
         match self.0 { 
             Some((_,ref mut child)) => child.push_back(data),
             None => self.push_front(data)
         }
     }
     #[allow(dead_code)] 
     pub fn insert_sorted(&mut self, data:T) { 
         // push the the element to the front then sort then 
    
         match self.0 { 
             Some((ref  front, ref  mut back)) =>  { 
               let t = back.clone().0.take().unwrap();
               let f = front.clone();
 if    t.0 < data {
     back.push_front(data.clone());
    // back.insert_sorted(data);
      back.insert_sorted(f);
      
    } 
 
 

             },
             None => self.push_front(data)
         }
     }
}


fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);
    ll.insert_sorted(4);
    println!("{:?}",ll);
}
// challenge:: inserted sorted 