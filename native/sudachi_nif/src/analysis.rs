use sudachi::analysis::stateful_tokenizer::StatefulTokenizer;
use sudachi::analysis::stateless_tokenizer::DictionaryAccess;
use sudachi::analysis::Mode;
use sudachi::prelude::MorphemeList;
use sudachi::sentence_splitter::{SentenceSplitter, SplitSentences};

use super::output::{SudachiOutput};

pub struct AnalyzeSplitted<'a, D: DictionaryAccess + 'a> {
    splitter: SentenceSplitter<'a>,
    analyzer: StatefulTokenizer<&'a D>,
    morphemes: MorphemeList<&'a D>,
}
impl<'a, D: DictionaryAccess + 'a> AnalyzeSplitted<'a, D> {
    pub fn new(dict: &'a D, mode: Mode, flag_debug: bool) -> Self {
        Self {
            splitter: SentenceSplitter::new().with_checker(dict.lexicon()),
            morphemes: MorphemeList::empty(dict.clone()),
            analyzer: StatefulTokenizer::create(dict, flag_debug, mode),
        }
    }
    pub fn analyze(&mut self, text: &str, output: &mut impl SudachiOutput<&'a D>) {
        for (_, sent) in self.splitter.split(text) {
            self.analyzer.reset().push_str(sent);
            self.analyzer.do_tokenize()
                .unwrap_or_else(|e| panic!("tokenization failed, input: {}\n{}", sent, e));

            self.morphemes.collect_results(&mut self.analyzer)
                .expect("result collection failed");

            output.write(&self.morphemes)
                .expect("write result failed");
        }
        output.flush()
    }
}
