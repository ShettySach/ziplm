## ziplm.rs

[Futrell's ziplm](https://github.com/Futrell/ziplm), written in Rust. WIP.

### CLI Instructions

#### Install
```bash
cargo install --git=https://github.com/ShettySach/ziplm --features=cli
```

#### Usage
```bash
Usage: ziplm [OPTIONS]

Options:
  -l, --length <LENGTH>            [default: 100]
  -p, --prefix <PREFIX>            [default: ]
  -t, --temperature <TEMPERATURE>  [default: 0.25]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Library Instructions

#### Install
```bash
cargo add --git=https://github.com/ShettySach/ziplm
```

#### Usage
```Rust
use ziplm::ZipModel

 fn main() {
    let vocab = "qwertyuiopasdfghjklzxcvbnm,. '";
    let data = include_str!("../data.txt");
    let ln256 = 2.408;

    let args: Args = Args::parse();

    let model = ZipModel::new(vocab, data, ln256);
    let _ = model.sample_sequence(args.length, &args.prefix, args.temperature);
}
```

### Todo
- Add options to train on custom .txt file
- Add options for other compression methods ( Currently supports only gzip )
- Optimizations, accuracy and precision improvements

### Credits
- [Original ziplm repo by Futrell](https://github.com/Futrell/ziplm)
- [Frankenstein; Or, The Modern Prometheus by Mary Shelley](https://www.gutenberg.org/cache/epub/84/pg84.txt)
