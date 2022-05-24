use dq3::attr::Attr;
use dq3::job::Job;
use dq3::personality::Personality;
use dq3::player::{Player, PlayerInit};
use dq3::sex::Sex;

use enum_iterator::IntoEnumIterator;
use plotlib;

fn growthed_player(before: &Player, dst_level: u8) -> Player {
    let mut after = before.clone();

    for _ in before.level()..dst_level {
        after.levelup();
    }

    after
}

fn simulate_growth(before: &Player, dst_level: u8, sample: usize) -> Vec<Player> {
    let mut sequence = Vec::new();

    for _ in 0..sample {
        sequence.push(growthed_player(before, dst_level));
    }

    sequence
}

fn plot_status_dist(sequence: &Vec<Player>) {
    for state in Attr::into_enum_iter() {
        let seq: Vec<_> = sequence
            .iter()
            .map(|player| player.attrs[state].to_num::<u8>() as f64)
            .collect();

        let h = plotlib::repr::Histogram::from_slice(
            &seq,
            plotlib::repr::HistogramBins::Bounds((100..200).map(f64::from).collect::<Vec<f64>>()),
        );

        let v = plotlib::view::ContinuousView::new().add(h);

        plotlib::page::Page::single(&v)
            .save(format!("status_{:?}_hist.svg", state))
            .unwrap();
    }
}

fn main() {
    let before = PlayerInit {
        lv: 1,
        pow: 9,
        spd: 2,
        vit: 19,
        int: 2,
        lck: 3,
        sex: Sex::Man,
        personality: Personality::Tough,
        job: Job::Soldier,
        ..Default::default()
    }
    .init();

    let dst_level = 22;

    let sample = 100000;
    let sequence = simulate_growth(&before, dst_level, sample);

    plot_status_dist(&sequence);
}
