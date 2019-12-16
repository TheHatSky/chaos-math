// extern crate plotlib;

use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::{Marker, Point};
use plotlib::view::ContinuousView;

fn main() {
    let initial_value = 0.5;

    let mut data: Vec<(f64, f64)> = vec![];
    for i in 0..60000 {
        let lambda = 3.4 + f64::from(i) / 100000.0;
        let s = calculate(initial_value, lambda, 1000);

        // let s = find_stable_iteration_cycle(&vec![0.12123, 0.84475, 0.99999, 0.12123]);

        for j in 0..s.len() {
            data.push((lambda, s[j]));
        }
    }

    // We create our scatter plot from the data
    let s1: Scatter = Scatter::from_slice(&data).style(
        Style::new()
            .size(0.1)
            .marker(Marker::Circle)
            .colour("#DD3355"),
    ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(&s1)
        .x_range(3.4, 4.)
        .y_range(0., 1.);

    print!("Hi there");
    // A page with a single view is then saved to an SVG file
    Page::single(&v)
        .save(format!("scatter_{}.svg", initial_value))
        .unwrap();
}

fn calculate(initial_value: f64, lambda: f64, n: i32) -> Vec<f64> {
    let mut current_value = initial_value;
    let mut result: Vec<f64> = vec![];

    for _ in 0..n {
        current_value = lambda * current_value * (1.0 - current_value);
        result.push(current_value);
    }

    // println!("{:?}", result);
    // find_stable_iteration_cycle(&result)

    result.dedup();

    result
}

const EPSILON: f64 = 0.01;

fn find_stable_iteration_cycle(values: &Vec<f64>) -> Vec<f64> {
    let len = values.len();
    // println!("len = {}", len);

    let mut result: Vec<f64> = vec![];

    for i in 0..len {
        let ith_element = values[len - i - 1];

        // println!("i {}-th = {}", i, ith_element);
        // println!("{:?}", result);

        for added_value in &result {
            // println!("{} - {}", added_value, ith_element);

            let diff = (added_value - ith_element).abs();
            // println!("diff {} {:?}", diff, diff < EPSILON);

            if diff < EPSILON {
                // println!("return");
                return result;
            }
        }

        result.push(ith_element);
    }

    result
}
