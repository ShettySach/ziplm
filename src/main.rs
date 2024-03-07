#[cfg(feature = "cli")]
mod cli {
    use clap::Parser;
    use ziplm::ZipModel;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Args {
        #[arg(short, long, default_value_t = 100)]
        length: u16,

        #[arg(short, long, default_value_t = String::new())]
        prefix: String,

        #[arg(short, long, default_value_t = 0.25)]
        temperature: f64,
    }

    pub fn run() {
        let vocab = "qwertyuiopasdfghjklzxcvbnm,. '";
        let data = include_str!("../data.txt");
        let ln256 = 2.408;

        let args: Args = Args::parse();

        let model = ZipModel::new(vocab, data, ln256);
        let _ = model.sample_sequence(args.length, &args.prefix, args.temperature);
    }
}

fn main() {
    #[cfg(feature = "cli")]
    cli::run();
}
