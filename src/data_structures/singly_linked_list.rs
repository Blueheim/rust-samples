struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

struct SinglyLinkList<T> {
    head: ListItem<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }
    fn next(&self) -> Option<&Self> {
        if let Some(next) = &self.next {
            Some(next)
        } else {
            None
        }
    }
    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }
    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> SinglyLinkList<T> {
    fn new(data: T) -> Self {
        SinglyLinkList {
            head: ListItem::new(data),
        }
    }
    fn append(&mut self, data: T) {
        let mut tail = self.head.mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
    }
    fn head(&self) -> &ListItem<T> {
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
