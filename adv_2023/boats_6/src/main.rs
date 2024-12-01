use std::f32::consts::SQRT_2;

fn main() {
    // let data: Vec<(f64, f64)> = vec![
    //     (54.0, 446.0),
    //     (81.0, 1292.0),
    //     (70.0, 1035.0),
    //     (88.0, 1007.0),
    // ];
    let data: Vec<(f64, f64)> = vec![(54817088.0, 446129210351007.0)];
    // let data: Vec<(f64, f64)> = vec![(7.0, 9.0), (15.0, 40.0), (30.0, 200.0)];
    let mut result = 1.0;
    for (time, distance) in data {
        let factor = (time.powf(2.0) - 4.0 * distance).sqrt();
        let higher = ((-time - factor) / (-2.0));
        let lower = ((-time + factor) / (-2.0));
        let higher = if higher == higher.floor() {
            higher - 1.0
        } else {
            higher.floor()
        };
        let lower = if lower == lower.ceil() {
            lower + 1.0
        } else {
            lower.ceil()
        };
        println!("{},{}", lower, higher);
        result *= higher - lower + 1.0;
    }
    println!("{}", result);
}
