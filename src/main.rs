fn main() {
    let data = std::fs::read_to_string("data.txt").unwrap().to_lowercase();
    let alphabet = "this is your life, and it is ending one minute at-a-time.";
    let ln256 = 2.408;

    let model = ziplm::ZipModel::new(
        alphabet.chars().map(|ch| ch.to_string()).collect(),
        &data,
        ln256,
    );

    let results = model.sample_sequence(300, "the ", 0.5).collect::<Vec<_>>();

    println!("{}", results.last().unwrap());
}
