fn main() {
    // Dummy :)
}

// Count the number of times the value between elements has increased
fn count_increases(depths: &[i32]) ->i32 {
    let mut num_increases = 0;

    for i in 1..depths.len(){
        let previous = depths[i-1];
        let current = depths[i];

        if current > previous {
            num_increases+=1;
        }

    }

    num_increases
}

mod tests {
    use super::*;


    #[test]
    fn test_basic() {
        let depths: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(&depths), 7);
    }

    use std::fs;
    use std::str;
    use std::iter;
    use std::path;

    // Find a file relative to a cargo.toml file
    fn find_file(relative_path: &str) -> String {
        use std::path::PathBuf;
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(relative_path);

        return String::from(d.as_path().to_str().expect(""));
    }

    // Test on the full input
    #[test]
    fn test_input() {
        let input : String = fs::read_to_string(find_file("input.txt")).expect("Error reading the file");
        let depths : Vec<i32> = input.split("\n").filter_map(|x| x.parse::<i32>().ok()).collect();
        // println!("{}", count_increases(&depths));
        // 1583
        assert_eq!(count_increases(&depths), 1583);
    }
}
