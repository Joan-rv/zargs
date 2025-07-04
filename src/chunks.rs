pub struct Chunks<I: Iterator> {
    iterator: I,
    buffer: Vec<I::Item>,
    chunk_size: usize,
}

impl<I: Iterator> Iterator for Chunks<I> {
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.len() < self.chunk_size {
            if let Some(item) = self.iterator.next() {
                self.buffer.push(item)
            } else {
                break;
            }
        }
        if self.buffer.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.buffer))
        }
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
            buffer: Vec::new(),
            chunk_size,
        }
    }
}
