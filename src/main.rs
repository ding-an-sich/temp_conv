use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let (temp, to) = parse_config(&args);
    let n_temp: f32 = temp.parse().expect("Invalid temperature");
    let conv = convert_temp(n_temp, to).expect("Invalid unit");
    println!("{conv}")
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let temp = &args[1];
    let to = &args[2];

    (temp, to)
}

fn convert_temp(n: f32, to: &str) -> Result<f32, &str> {
    match to {
        "-f" => {
            let conv = (n * 9.0 / 5.0) + 32.0;
            Ok(conv)
        }
        "-c" => {
            let conv = (n - 32.0) * 5.0 / 9.0;
            Ok(conv)
        }
        _ => Err("Invalid unit"),
    }
}
