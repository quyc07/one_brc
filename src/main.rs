mod calc;
mod generate;
const FILE_PATH: &str = "data.txt";

fn main() {
    generate::generate_data();
    calc::calc().unwrap();
}
