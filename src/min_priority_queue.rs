pub struct MinPriorityQueue<T>
where
    T: Clone + PartialEq + PartialOrd,
{
    data: Vec<T>,
    mod_count: usize,
}

impl<T> MinPriorityQueue<T>
where
    T: Clone + PartialEq + PartialOrd,
{
    pub fn new() -> MinPriorityQueue<T> {
        MinPriorityQueue {
            data: Vec::<T>::new(),
            mod_count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    #[allow(dead_code)]
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.mod_count += 1;
        for i in (0..(self.data.len() / 2)).rev().into_iter() {
            self.sort_data(i);
        }
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[0].clone())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let last = self.data.len() - 1;
            self.data.swap(0, last);

            let result = self.data.pop();
            self.mod_count += 1;

            for i in (0..(self.data.len() / 2)).rev().into_iter() {
                self.sort_data(i);
            }

            result
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, item: &T) -> bool {
        self.data.contains(item)
    }

    fn sort_data(&mut self, index: usize) {
        if self.data.len() == 1 {
            return;
        }

        let size = self.data.len();
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        let mut next = index;

        if left < size && self.data[left] < self.data[next] {
            next = left;
        }

        if right < size && self.data[right] < self.data[next] {
            next = right
        }

        if next != index {
            self.data.swap(index, next);
            self.sort_data(next);
        }
    }
}
