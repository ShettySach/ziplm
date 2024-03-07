#[cfg(feature = "cli")]
mod cli {
    use clap::Parser;
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use ziplm::ZipModel;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Args {
        /// Maximum length of sample generated
        #[arg(short, long, default_value_t = 100)]
        length: u16,

        /// Prefix for sample generated
        #[arg(short, long, default_value_t = String::new())]
        prefix: String,

        /// Temperature for sample generated
        #[arg(short, long, default_value_t = 0.25)]
        temperature: f64,

        /// Path of training data (.txt file)
        /// [Defaults to Mary Shelley's 'Frankenstein']
        #[arg(short, long)]
        data: Option<PathBuf>,

        /// Path of vocabulary data (.txt file)
        /// [Defaults to qwertyuiopasdfghjklzxcvbnm,. '"]
        #[arg(short, long)]
        vocab: Option<PathBuf>,
    }

    fn validate(path: &Option<PathBuf>) -> Option<String> {
        match path {
            Some(path) => match read_to_string(path) {
                Ok(mut content) => {
                    content.retain(|c| c != '\n');
                    Some(content)
                }
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn run() {
        let args: Args = Args::parse();

        let vocab = match validate(&args.vocab) {
            Some(content) => content,
            None => r#"qwertyuiopasdfghjklzxcvbnm,. '""#.to_string(),
        };
        let data = match validate(&args.vocab) {
            Some(content) => content,
            None => include_str!("../data.txt").to_string(),
        };
        let ln256 = 2.408;

        let model = ZipModel::new(&vocab, &data, ln256);
        let _ = model.sample_sequence(args.length, &args.prefix, args.temperature);
    }
}

fn main() {
    #[cfg(feature = "cli")]
    cli::run();
}
