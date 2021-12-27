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

#[rustler::nif]
fn rustler_test<'a>(env: Env<'a>, text: &str, print_all: bool) -> NifResult<ExList<'a>> {

    let config = Config::new(
        None, //args.config_file.clone(),
        None, //args.resource_dir.clone(),
        Some(PathBuf::from(r"/home/shoz/Sudachi/sudachi_ex/native/sudachi_nif/sudachi-cli/resources/system.dic")),
    )
    .expect("Failed to load config file");

    let dict = JapaneseDictionary::from_cfg(&config)
        .unwrap_or_else(|e| panic!("Failed to create dictionary: {:?}", e));

    let enable_debug = false;
    let mut analyzer = AnalyzeSplitted::new(&dict, Mode::C, enable_debug);

    let mut builder = ExBuilder::new(env, print_all);

    analyzer.analyze(&text, &mut builder);

    Ok(builder.res)
}

rustler::init!("Elixir.Sudachi.NIF", [
    rustler_test
]);
