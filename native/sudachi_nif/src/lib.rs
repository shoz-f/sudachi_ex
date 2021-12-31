mod analysis;
mod output;
mod exlist;

use std::path::PathBuf;

use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::prelude::Mode;

use self::analysis::AnalyzeSplitted;
use self::exlist::{ExList, ExBuilder};

use rustler::{Env, NifResult};

use once_cell::sync::OnceCell;
static DICT: OnceCell<JapaneseDictionary> = OnceCell::new();

#[rustler::nif]
fn analize<'a>(env: Env<'a>, text: &str, put_all: bool, enable_debug: bool) -> NifResult<ExList<'a>> {
    let dict = DICT.get_or_init(|| {
        let config = Config::new(
            None,
            Some(PathBuf::from(env!("SUDACHI_RESOURCES"))),
            None
        )
        .expect("Failed to load config file");

        JapaneseDictionary::from_cfg(&config)
            .unwrap_or_else(|e| panic!("Failed to create dictionary: {:?}", e))
    });

    let mut analyzer = AnalyzeSplitted::new(&dict, Mode::C, enable_debug);
    let mut builder  = ExBuilder::new(env, put_all);
    analyzer.analyze(&text, &mut builder);

    Ok(builder.checkout())
}

rustler::init!("Elixir.Sudachi.NIF", [
    analize
]);
