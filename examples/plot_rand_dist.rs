use dq3::rand::Rng;
use plotlib;

fn rand_sequence(sample: usize) -> Vec<u8> {
    let mut rng = Rng::new(None);
    let mut sequence = Vec::new();

    for _ in 0..sample {
        sequence.push(rng.rand(None));
    }

    return sequence;
}

fn plot_rand_dist(dim: usize, sample: usize) {
    let sequence = rand_sequence(sample);
    let mut dists = Vec::new();

    for i in 0..sample - dim + 1 {
        let slice = &sequence[i..i + dim];

        let ave = slice.iter().fold(0, |acc: u32, v| acc + *v as u32);
        dists.push(ave as f64);
    }

    let h = plotlib::repr::Histogram::from_slice(
        &dists,
        plotlib::repr::HistogramBins::Count(256 * dim),
    );

    let v = plotlib::view::ContinuousView::new().add(h);

    plotlib::page::Page::single(&v).save("hist.svg").unwrap();
}

fn main() {
    plot_rand_dist(3, 1000000);
}
