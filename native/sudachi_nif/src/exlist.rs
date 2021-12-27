use sudachi::analysis::morpheme::Morpheme;
use sudachi::analysis::stateless_tokenizer::DictionaryAccess;
use sudachi::prelude::{MorphemeList, SudachiResult};

use rustler::{Env, Term, Encoder};

use super::output::{SudachiOutput};

pub type ExList<'a> = Vec<Term<'a>>;

pub struct ExBuilder<'a> {
    print_all: bool,
    env: Env<'a>,
    pub res: ExList<'a>,
}
impl<'a> ExBuilder<'a> {
    pub fn new(env: Env<'a>, print_all: bool) -> Self {
        ExBuilder {
            env,
            res: Vec::new(),
            print_all,
        }
    }
//    pub fn checkout(&self) -> ExList<'a> {
//        self.res
//    }
}
impl<'a, T: DictionaryAccess> SudachiOutput<T> for ExBuilder<'a> {
    fn write(&mut self, morphemes: &MorphemeList<T>) -> SudachiResult<()> {
        for m in morphemes.iter() {
            let mut word: ExList = Vec::new();
            write_morpheme_basic(self.env, &mut word, &m);
            if self.print_all {
                write_morpheme_extended(self.env, &mut word, &m)
            }
            self.res.push(word.encode(self.env));
        }
        Ok(())
    }

    fn flush(&mut self) {
    }
}

#[inline]
fn write_morpheme_basic<'a, T: DictionaryAccess>(env: Env<'a>, word: &mut ExList<'a>, morpheme: &Morpheme<T>) {
    word.push(morpheme.surface().encode(env));

    let all_pos = morpheme.part_of_speech();
    for pos in all_pos.iter() {
        word.push(pos.encode(env));
    }

    word.push(morpheme.normalized_form().encode(env));
}

#[inline]
fn write_morpheme_extended<'a, T: DictionaryAccess>(env: Env<'a>, word: &mut ExList<'a>, morpheme: &Morpheme<T>) {
    word.push(morpheme.dictionary_form().encode(env));
    word.push(morpheme.reading_form().encode(env));
    word.push(morpheme.dictionary_id().encode(env));
    word.push(morpheme.synonym_group_ids().encode(env));
}
