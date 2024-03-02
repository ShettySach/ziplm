use flate2::write::GzEncoder;
use flate2::Compression;
use ndarray::Array1;
use std::collections::HashMap;
use std::io::Write;

struct ZipModel<'a> {
    vocabulary: Vec<&'a str>,
    training: String,
    conversion: f64,
    index: HashMap<&'a str, usize>,
}

impl<'a> ZipModel<'a> {
    fn new(vocabulary: Vec<&'a str>, training: &str, conversion: f64) -> Self {
        let index: HashMap<&str, usize> = vocabulary
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect();

        ZipModel {
            vocabulary,
            training: training.to_string(),
            conversion,
            index,
        }
    }

    fn logprobs(self, prefix: &str, temperature: f64) -> Array1<f64> {
        let code_lengths: Vec<usize> = self
            .vocabulary
            .iter()
            .map(|v| {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

                encoder
                    .write_all(format!("{}{}{}", self.training, prefix, v).as_bytes())
                    .unwrap();

                encoder.finish().unwrap().len()
            })
            .collect();

        let code_lengths: Vec<f64> = code_lengths
            .iter()
            .map(|&x| -1.0 * x as f64 * self.conversion * (1.0 / temperature))
            .collect();

        log_softmax(&Array1::from(code_lengths))
    }
}

fn log_softmax(input: &Array1<f64>) -> Array1<f64> {
    let max_val = input.fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));
    let exp_sum: f64 = input.map(|&x| (x - max_val).exp()).sum();
    let log_exp_sum = exp_sum.ln();

    input.map(|&x| x - max_val - log_exp_sum)
}
