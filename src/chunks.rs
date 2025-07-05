pub struct Chunks<I: Iterator> {
    iterator: I,
    buffer: Option<Vec<I::Item>>,
    chunk_size: usize,
}

impl<I: Iterator> Iterator for Chunks<I> {
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let buffer = self.buffer.as_mut()?;

        while buffer.len() < self.chunk_size {
            if let Some(item) = self.iterator.next() {
                buffer.push(item)
            } else {
                return self.buffer.take();
            }
        }

        self.buffer.as_mut().map(std::mem::take)
    }
}

pub trait ChunkIterator {
    type Iter: Iterator;
    fn chunks(self, chunk_size: usize) -> Chunks<Self::Iter>;
}

impl<I: Iterator> ChunkIterator for I {
    type Iter = I;
    fn chunks(self, chunk_size: usize) -> Chunks<Self::Iter> {
        Chunks {
            iterator: self,
            buffer: Some(Vec::new()),
            chunk_size,
        }
    }
}
