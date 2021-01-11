use std::env;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        // Parse the argument to turn it from string to a Float32
        let temp = arg1.parse::<f64>().unwrap();

        let result = (temp - 32.0) * 5.0/9.0;

        // Print the damn temp!
        println!("{}°C = {}°F", result, temp)

    }
}
