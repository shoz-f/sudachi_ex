mod analysis;
mod output;

use std::io;
use std::path::PathBuf;

//use crate::analysis::{AnalyzeSplitted};
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::prelude::Mode;

use self::analysis::AnalyzeSplitted;
use self::output::Simple;
//use crate::output::{SudachiOutput};

/// A Japanese tokenizer
///
/// If you are looking for options for the dictionary building, try sudachi build/ubuild --help.
fn main() {
    // load config file
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

    //let mut data = String::with_capacity(4 * 1024);
    let data: &str = "打込む かつ丼 附属 vintage";

    // output: stdout or file
    let print_all  = true;
    let mut writer = Simple::new(print_all, Box::new(io::stdout()));

    // tokenize and output results
    analyzer.analyze(&data, &mut writer);
}
