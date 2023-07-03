use clap::Parser;

struct Point {
    x: f32,
    y: f32
}

/// Simple program to measure a line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MeasureApp {
    /// X-value of start of line to measure
    #[arg(long)]
    x1: f32,

    /// Y-value of start of line to measure
    #[arg(long)]
    y1: f32,

    /// X-value of end of line to measure
    #[arg(long)]
    x2: f32,

    /// Y-value of end of line to measure
    #[arg(long)]
    y2: f32
}

fn main() {
    let args = MeasureApp::parse();

    println!("Start: x = {}, y = {}", args.x1, args.y1);
    println!("End: x = {}, y = {}", args.x2, args.y2);
    
    let point_1 = Point { x: args.x1, y: args.y1 };
    let point_2 = Point { x: args.x2, y: args.y2 };
    let distance = distance_between_points(point_1, point_2);

    println!("Distance between points: {}", distance);
}

/// Calculate the distance between two points
fn distance_between_points(point_1: Point, point_2: Point) -> f32 {
    let x_diff = point_1.x - point_2.x;
    let y_diff = point_1.y - point_2.y;
    let x_squared = x_diff.powi(2);
    let y_squared = y_diff.powi(2);
    let sum = x_squared + y_squared;
    let distance = sum.sqrt();
    distance
}
