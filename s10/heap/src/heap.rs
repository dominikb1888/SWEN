use crate::person::Person;

#[derive(Debug)]
pub struct PersonHeap {
    data: Vec<Person>,
}

impl PersonHeap {
    pub fn new() -> Self {
        PersonHeap { data: Vec::new() }
    }

    pub fn insert(&mut self, person: Person) {
        self.data.push(person);
        self.sift_up(self.data.len() - 1);
    }

    pub fn delete(&mut self) -> Option<Person> {
        if self.data.is_empty() {
            return None;
        }

        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let max = self.data.pop();
        self.sift_down(0);
        max
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] > self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.data.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < len && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < len && self.data[right] > self.data[largest] {
                largest = right;
            }

            if largest != idx {
                self.data.swap(idx, largest);
                idx = largest;
            } else {
                break;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

