pub struct Queue<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        let items = Vec::new();
        Queue { items }
    }

    pub fn push(&mut self, item: T) -> T {
        &self.items.push(item.clone());
        item
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.items.len() > 0 {
            let item = self.items[0].clone();
            self.items.remove(0);
            return Some(item);
        }
        None
    }
}

pub struct Stack<T> {
    items: Vec<T>,
    capacity: Option<usize>,
}

impl<T> Stack<T> {
    pub fn new(capacity: Option<usize>) -> Self {
        if let Some(capacity) = capacity {
            let items = Vec::with_capacity(capacity);
            return Stack {
                items,
                capacity: Some(capacity),
            };
        }
        let items = Vec::new();
        Stack {
            items,
            capacity: None,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(capacity) = self.capacity {
            if self.items.len() < capacity {
                self.items.push(item);
                Ok(())
            } else {
                Err(format!(
                    "[StackOverflow]: Stack has reached its capacity {}",
                    capacity
                )
                .into())
            }
        } else {
            self.items.push(item);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
