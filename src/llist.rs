use std::boxed::Box;

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        List::<T> { head: None }
    }

    pub fn push(&mut self, new_value: T) {
        let new_elem = Some(Box::new(Node::<T> {
            value: new_value,
            next: self.head.clone(),
        }));
        self.head = new_elem;
    }

    pub fn pop(&mut self) -> Option<T> {
        match &self.head {
            None => None,
            Some(thebox) => {
                let theval = thebox.value.clone();

                let headref = &mut self.head;
                let mut boxclone = headref.clone().unwrap();
                let nextref = &mut boxclone.as_mut().next;
                *headref = nextref.clone();
                *nextref = None;

                Some(theval)
            }
        }
    }

    pub fn for_each(&mut self, foreach_func: fn(&mut T)) {
        let mut current = &mut self.head;
        while current.is_some() {
            let node = current.as_mut().unwrap().as_mut();
            foreach_func(&mut node.value);

            current = &mut node.next;
        }
    }
}
