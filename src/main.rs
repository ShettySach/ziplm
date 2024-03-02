fn main() {
    let data = include_str!("../data.txt");
    let alphabet = "this is your life, and it is ending one minute at-a-time.";
    let ln256 = 2.408;

    let model = ziplm::ZipModel::new(alphabet, &data, ln256);
    let response = model.sample_sequence(100, "", 1);

    println!("{}", response);
}
