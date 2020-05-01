use dq3::rand::Rng;
use gnuplot;

fn rand_sequence(sample: usize) -> Vec<u8> {
    let mut rng = Rng::new(None);
    let mut sequence = Vec::new();

    for _ in 0..sample {
        sequence.push(rng.rand(None));
    }

    return sequence;
}

fn plot_rand_scatter3d(sample: usize) {
    let sequence = rand_sequence(sample);
    let mut vecx = Vec::new();
    let mut vecy = Vec::new();
    let mut vecz = Vec::new();

    for i in 0..sample - 2 {
        vecx.push(sequence[i]);
        vecy.push(sequence[i + 1]);
        vecz.push(sequence[i + 2]);
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes3d()
        .points(&vecx, &vecy, &vecz, &[gnuplot::Color("red")]);
    fg.set_scale(1.0, 1.0).show();
}

fn main() {
    plot_rand_scatter3d(10000);
}
