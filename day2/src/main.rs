use anyhow::Error;
struct Constraint {
    letter: String,
    lower_bound: i32,
    upper_bound: i64
}

impl FromStr for Constraint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Inputs look something like: 1-3 a
        bounds = s.split(" ");
        bounds.split("-");
    }
}




/// Reads all the contents of the file into memory as a vector of strings.
/// Shitty, don't use anywhere you care about.
fn load_file(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);
    let lines = Vec::new();
    for line in reader.lines() {
        lines.push(line);
    }

    lines
}

fn is_password_valid(line: String) -> bool {
    let constraint = line.split(":")[0];
}


fn main() {
    let file = load_file("input.txt");

}
