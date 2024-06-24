pub struct CircularBuffer<T> {
    /// Using Option leads to less efficient memory layout, but
    /// it allows us to avoid using `unsafe` to handle uninitialized
    /// mempory ourselves.
    data: Vec<Option<T>>,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);
        Self { data, start: 0, end: 0 }
    }

    fn is_empty(&self) -> bool {
        self.start == self.end && self.data[self.start].is_none()
    }

    fn is_full(&self) -> bool {
        self.start == self.end && self.data[self.start].is_some()
    }

    fn inc_start(&mut self) {
        self.start += 1;
        self.start %= self.data.len();
    }

    fn inc_end(&mut self) {
        self.end += 1;
        self.end %= self.data.len();
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer)
        }
        self.data[self.end] = Some(element);
        self.inc_end();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer)
        }
        let res = self.data[self.start].take().expect("uninitialized memory :-(");
        self.inc_start();
        Ok(res)
    }

    pub fn clear(&mut self) {
        self.data.fill_with(|| None);
        self.start = 0;
        self.end = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        self.data[self.end] = Some(element);
        if self.start == self.end {
            self.inc_start();
        }
        self.inc_end();
    }
}
