use core::f32;
use flate2::write::GzEncoder;
use flate2::Compression;
use ndarray::Array1;
use std::io::Write;
use std::{collections::HashMap, usize};
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

pub struct ZipModel {
    vocabulary: Vec<String>,
    training: String,
    conversion: f64,
    index: HashMap<String, usize>,
}

impl ZipModel {
    pub fn new(vocabulary: Vec<String>, training: &str, conversion: f64) -> Self {
        let index: HashMap<String, usize> = vocabulary
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect();

        ZipModel {
            vocabulary,
            training: training.to_string(),
            conversion,
            index,
        }
    }

    pub fn log_probs(&self, prefix: &str, temperature: f64) -> Array1<f64> {
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

    pub fn sequence_logprob(
        &self,
        sequence: Vec<&str>,
        mut prefix: String,
        temperature: f64,
    ) -> f64 {
        let mut score = 0.0;

        for x in sequence {
            let scores = self.log_probs(&prefix, temperature);
            let index = *self.index.get(x).unwrap();
            score += scores[index];
            prefix.push_str(x);
        }

        score
    }

    pub fn sample(&self, prefix: &str, temperature: f64) -> &str {
        let scores = self.log_probs(prefix, temperature);
        let p = scores.map(|&x| x.exp() as f32);

        let b = WalkerTableBuilder::new(&p.to_vec());
        let w = b.build();
        let i = w.next();

        &self.vocabulary[i]
    }

    pub fn sample_sequence<'b>(
        &'b self,
        maxlen: usize,
        prefix: &'b str,
        temperature: f64,
    ) -> impl Iterator<Item = String> + 'b {
        let iterator = std::iter::successors(Some(prefix.to_string()), move |seq| {
            let result = self.sample(seq, temperature);
            Some(seq.to_string() + result)
        });

        iterator.take(maxlen)
    }
}

fn log_softmax(input: &Array1<f64>) -> Array1<f64> {
    let max_val = input.fold(f64::NEG_INFINITY, |acc, &x| acc.max(x));
    let exp_sum: f64 = input.map(|&x| (x - max_val).exp()).sum();
    let log_exp_sum = exp_sum.ln();

    input.map(|&x| x - max_val - log_exp_sum)
}
