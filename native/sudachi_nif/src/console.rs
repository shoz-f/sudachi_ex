use sudachi::analysis::morpheme::Morpheme;
use sudachi::analysis::stateless_tokenizer::DictionaryAccess;
use sudachi::prelude::{MorphemeList, SudachiResult};

use super::output::{SudachiOutput};

use std::io::{BufWriter, Write};

type Writer = BufWriter<Box<dyn Write>>;

pub struct Console {
    print_all: bool,
    writer: Writer,
}
impl Console {
    pub fn new(print_all: bool, inner_writer: Box<dyn Write>) -> Self {
        Console {
            print_all,
            writer: BufWriter::new(inner_writer),
        }
    }
}
impl<T: DictionaryAccess> SudachiOutput<T> for Console {
    fn write(&mut self, morphemes: &MorphemeList<T>) -> SudachiResult<()> {
        for m in morphemes.iter() {
            write_morpheme_basic(&mut self.writer, &m)?;
            if self.print_all {
                write_morpheme_extended(&mut self.writer, &m)?
            }
            self.writer.write_all(b"\n\r")?;
        }
        self.writer.write_all(b"EOS\n\r")?;
        Ok(())
    }
    fn flush(&mut self) {
        self.writer.flush().expect("flush failed");
    }
}

#[inline]
fn write_morpheme_basic<T: DictionaryAccess>(writer: &mut Writer, morpheme: &Morpheme<T>) -> SudachiResult<()> {
    writer.write_all(morpheme.surface().as_bytes())?;
    writer.write_all(b"\t")?;
    let all_pos = morpheme.part_of_speech();
    for (idx, pos) in all_pos.iter().enumerate() {
        writer.write_all(pos.as_bytes())?;
        if idx + 1 != all_pos.len() {
            writer.write_all(b",")?;
        }
    }
    writer.write_all(b"\t")?;
    writer.write_all(morpheme.normalized_form().as_bytes())?;
    Ok(())
}

#[inline]
fn write_morpheme_extended<T: DictionaryAccess>(writer: &mut Writer, morpheme: &Morpheme<T>, ) -> SudachiResult<()> {
    write!(
        writer,
        "\t{}\t{}\t{}\t{:?}",
        morpheme.dictionary_form(),
        morpheme.reading_form(),
        morpheme.dictionary_id(),
        morpheme.synonym_group_ids(),
    )?;

    if morpheme.is_oov() {
        writer.write_all(b"\t(OOV)")?;
    }
    Ok(())
}
