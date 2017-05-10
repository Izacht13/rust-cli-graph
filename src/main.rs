use std::ops::Range;
use std::f64;

struct LinearScale {
    domain: Range<i64>,
    range: Range<i64>
}

impl LinearScale {
    fn new(domain: Range<i64>, range: Range<i64>) -> LinearScale {
        LinearScale {
            domain: domain,
            range: range
        }
    }
    fn scale(&self, height: f64) -> f64 {
        let domain_dist = self.domain.end - self.domain.start;
        let height_into = height - self.domain.start as f64;
        let weight = height_into / (domain_dist as f64);
        let range_dist = self.range.end - self.range.start;
        range_dist as f64 * weight + self.range.start as f64
    }
    fn check_domain(&self, num: f64) -> bool {
        num >= self.domain.start as f64 && num < self.domain.end as f64
    }
}

fn graph<F: Fn(f64) -> f64>(curve: F, x: Range<i64>, y: Range<i64>) {
    let width = x.end - x.start;
    let height = y.end - y.start;

    let scale_x = LinearScale::new(x.clone(), 0..width);
    let scale_y = LinearScale::new(y, 0..height);

    let mut image = vec![vec![' '; width as usize]; height as usize];
    
    // draw axis
    let mut axis_y: usize = 0;
    if scale_x.check_domain(0.0) {
        axis_y = scale_x.scale(0.0) as usize;
        for row in &mut image {
            row[axis_y] = '|';
        }
    }
    if scale_y.check_domain(0.0) {
        let axis_x = scale_y.scale(0.0) as usize;
        for i in 0..(width as usize) {
            image[axis_x][i] = if i == axis_y { '+' } else { '-' };
        }
    }

    // plot points
    for i in x {
        let plot_x = scale_x.scale(i as f64) as usize;
        let curve_y = curve(i as f64);
        if !scale_y.check_domain(curve_y) {
            continue
        }
        let plot_y = scale_y.scale(curve_y) as usize;
        image[plot_y][plot_x] = '*';
    }

    for row in image {
        for c in row {
            print!("{}", c);
        }
        print!("\n");
    }
}


fn main() {
    graph(|y| (y*0.125).sin()*5.0, -50..50, -10..10)
}