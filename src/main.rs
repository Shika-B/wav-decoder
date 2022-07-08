mod utility;
mod wave_objects;
use wave_objects::Wave;

use std::fs::File;
use std::io::prelude::*;

use plotlib::page::Page;
use plotlib::repr::{ContinuousRepresentation, Plot};
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn partial_plot(wave: &Wave, amount_to_plot: usize) {
    let plot_points: Vec<(f64, f64)> = wave.data.data[0..amount_to_plot]
        .iter()
        .enumerate()
        .map(|(index, value)| (index as f64, (*value as f64) / 32767.0))
        .collect();
    let plot: Plot = Plot::new(plot_points).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .colour("#DD3355")
            .size(1.5),
    );
    let view = ContinuousView::new()
        .add(plot)
        .x_range(0.0, amount_to_plot as f64)
        .y_range(-1.0, 1.0)
        .x_label(format!("{} units = 1s", wave.header.sample_rate))
        .y_label("Normalized amplitude");
    Page::single(&view).save("scatter.svg").unwrap();
}
fn main() {
    let file = include_bytes!("sound.wav");
    let wave = Wave::new(file);
    println!("{:#?}", wave.header);
    println!("all good");
    partial_plot(&wave, 00);
}
