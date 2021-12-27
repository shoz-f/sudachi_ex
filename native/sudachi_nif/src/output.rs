use sudachi::prelude::{MorphemeList, SudachiResult};

pub trait SudachiOutput<T> {
    fn write(&mut self, morphemes: &MorphemeList<T>) -> SudachiResult<()>;
    fn flush(&mut self);
}
