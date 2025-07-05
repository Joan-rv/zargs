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
                return self
                    .buffer
                    .take()
                    .and_then(|v| (!v.is_empty()).then_some(v));
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
        assert_ne!(chunk_size, 0);
        Chunks {
            iterator: self,
            buffer: Some(Vec::new()),
            chunk_size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ChunkIterator;

    #[test]
    fn basic_functionality() {
        let input = vec![1, 2, 3, 4];
        let output: Vec<_> = input.into_iter().chunks(2).collect();
        assert_eq!(output, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn incomplete_chunk() {
        let input = vec![1, 2, 3];
        let output: Vec<_> = input.into_iter().chunks(2).collect();
        assert_eq!(output, vec![vec![1, 2], vec![3]]);
    }
}
