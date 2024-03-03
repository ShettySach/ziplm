fn main() {
    let vocab = "qwertyuiopasdfghjklzxcvbnm,. '";
    let data = include_str!("../data.txt");
    let ln256 = 2.408;

    let model = ziplm::ZipModel::new(&vocab, data, ln256);
    let _ = model.sample_sequence(150, "", 0.25);
}
