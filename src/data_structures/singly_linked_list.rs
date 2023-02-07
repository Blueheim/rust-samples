pub struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

pub struct SinglyLinkList<T> {
    head: ListItem<T>,
}

impl<T> ListItem<T> {
    pub fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }
    pub fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next)
        } else {
            None
        }
    }
    pub fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }
    pub fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> SinglyLinkList<T> {
    pub fn new(data: T) -> Self {
        SinglyLinkList {
            head: ListItem::new(data),
        }
    }
    pub fn append(&mut self, data: T) {
        let mut tail = self.head.mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
    }
    pub fn head(&self) -> &ListItem<T> {
        &self.head
    }
}

pub fn run() {
    let mut list = SinglyLinkList::new("head");
    list.append("middle");
    list.append("tail");
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }
}