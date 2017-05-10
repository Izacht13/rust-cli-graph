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

struct Graph {
    x: Range<i64>,
    y: Range<i64>,
    image: Vec<Vec<char>>,
    scale_x: LinearScale,
    scale_y: LinearScale,
    width: usize,
    height: usize
}

impl Graph {
    fn new(x: Range<i64>, y: Range<i64>) -> Graph {
        let width = (x.end - x.start) as usize;
        let height = (y.end - y.start) as usize;
        let scale_x = LinearScale::new(x.clone(), 0..(width as i64));
        let scale_y = LinearScale::new(y.clone(), 0..(height as i64));
        
        let mut image = vec![vec![' '; width]; height];
        
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
        Graph {
            x: x,
            y: y,
            width: width,
            height: height,
            scale_x: scale_x,
            scale_y: scale_y,
            image: image
        }
    }
    fn curve<F: Fn(f64) -> f64>(&mut self, curve: F, dot: char) {
        // plot points
        let mut x = self.x.clone();
        for i in x {
            print!("iter {}", dot);
            let plot_x = self.scale_x.scale(i as f64) as usize;
            let curve_y = curve(i as f64);
            if !self.scale_y.check_domain(curve_y) {
                continue
            }
            let plot_y = self.scale_y.scale(curve_y) as usize;
            self.image[(self.height - 1)-plot_y][plot_x] = dot;
        }

    }
    fn draw(self) {
        for row in self.image {
            for c in row {
                print!("{}", c);
            }
            print!("\n");
        }
    }
}

fn main() {
    let mut chart = Graph::new(-70..70, -20..20);
    chart.curve(|x| (x/8.0 + 2.0).sin()*19.0, '$');
    chart.curve(|x| (x/8.0).sin()*5.0, '*');
    chart.draw();
}