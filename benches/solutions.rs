use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use AOC::solutions::*;
use std::panic;

fn custom_criterion() -> Criterion {
    Criterion::default()
        .save_baseline("main".to_string())
}

macro_rules! bench_solution {
    ($c:expr, $year:expr, $day:expr, $part:expr, $func:path) => {{
        let filepath = format!("inputs/{}/day{:02}.txt", $year, $day);
        if std::path::Path::new(&filepath).exists() {
            // Test if the function is implemented by calling it once
            let test_result = panic::catch_unwind(|| {
                $func(filepath.clone())
            });
            
            if test_result.is_ok() {
                let name = format!("{} Day {:02} Part {}", $year, $day, $part);
                $c.bench_function(&name, |b| {
                    b.iter(|| $func(black_box(filepath.clone())))
                });
            } else {
                eprintln!("Skipping {} Day {:02} Part {} (not implemented)", $year, $day, $part);
            }
        }
    }};
}

fn benchmark_2025(c: &mut Criterion) {
    let mut group = c.benchmark_group("2025");
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    
    // Year 2025
    bench_solution!(group, 2025, 1, 1, year2025::day01::solution_2025_01_01);
    bench_solution!(group, 2025, 1, 2, year2025::day01::solution_2025_01_02);
    bench_solution!(group, 2025, 2, 1, year2025::day02::solution_2025_02_01);
    bench_solution!(group, 2025, 2, 2, year2025::day02::solution_2025_02_02);
    bench_solution!(group, 2025, 3, 1, year2025::day03::solution_2025_03_01);
    bench_solution!(group, 2025, 3, 2, year2025::day03::solution_2025_03_02);
    bench_solution!(group, 2025, 4, 1, year2025::day04::solution_2025_04_01);
    bench_solution!(group, 2025, 4, 2, year2025::day04::solution_2025_04_02);
    bench_solution!(group, 2025, 5, 1, year2025::day05::solution_2025_05_01);
    bench_solution!(group, 2025, 5, 2, year2025::day05::solution_2025_05_02);
    bench_solution!(group, 2025, 6, 1, year2025::day06::solution_2025_06_01);
    bench_solution!(group, 2025, 6, 2, year2025::day06::solution_2025_06_02);
    bench_solution!(group, 2025, 7, 1, year2025::day07::solution_2025_07_01);
    bench_solution!(group, 2025, 7, 2, year2025::day07::solution_2025_07_02);
    // Day 08 Part 1 has special signature with connection_sample_size parameter
    let filepath = "inputs/2025/day08.txt".to_string();
    if std::path::Path::new(&filepath).exists() {
        group.bench_function("2025 Day 08 Part 1", |b| {
            b.iter(|| year2025::day08::solution_2025_08_01(black_box(filepath.clone()), black_box(3)))
        });
    }
    bench_solution!(group, 2025, 8, 2, year2025::day08::solution_2025_08_02);
    bench_solution!(group, 2025, 9, 1, year2025::day09::solution_2025_09_01);
    bench_solution!(group, 2025, 9, 2, year2025::day09::solution_2025_09_02);
    
    group.finish();
}

fn benchmark_2015(c: &mut Criterion) {
    let mut group = c.benchmark_group("2015");
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    
    // Year 2015
    bench_solution!(group, 2015, 1, 1, year2015::day01::solution_2015_01_01);
    bench_solution!(group, 2015, 1, 2, year2015::day01::solution_2015_01_02);
    bench_solution!(group, 2015, 2, 1, year2015::day02::solution_2015_02_01);
    bench_solution!(group, 2015, 2, 2, year2015::day02::solution_2015_02_02);
    bench_solution!(group, 2015, 3, 1, year2015::day03::solution_2015_03_01);
    bench_solution!(group, 2015, 3, 2, year2015::day03::solution_2015_03_02);
    bench_solution!(group, 2015, 4, 1, year2015::day04::solution_2015_04_01);
    bench_solution!(group, 2015, 4, 2, year2015::day04::solution_2015_04_02);
    bench_solution!(group, 2015, 5, 1, year2015::day05::solution_2015_05_01);
    bench_solution!(group, 2015, 5, 2, year2015::day05::solution_2015_05_02);
    bench_solution!(group, 2015, 6, 1, year2015::day06::solution_2015_06_01);
    bench_solution!(group, 2015, 6, 2, year2015::day06::solution_2015_06_02);
    
    group.finish();
}

fn benchmark_2021(c: &mut Criterion) {
    let mut group = c.benchmark_group("2021");
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    
    // Year 2021
    bench_solution!(group, 2021, 1, 1, year2021::day01::solution_2021_01_01);
    bench_solution!(group, 2021, 1, 2, year2021::day01::solution_2021_01_02);
    bench_solution!(group, 2021, 2, 1, year2021::day02::solution_2021_02_01);
    bench_solution!(group, 2021, 2, 2, year2021::day02::solution_2021_02_02);
    bench_solution!(group, 2021, 3, 1, year2021::day03::solution_2021_03_01);
    bench_solution!(group, 2021, 3, 2, year2021::day03::solution_2021_03_02);
    bench_solution!(group, 2021, 4, 1, year2021::day04::solution_2021_04_01);
    bench_solution!(group, 2021, 4, 2, year2021::day04::solution_2021_04_02);
    bench_solution!(group, 2021, 5, 1, year2021::day05::solution_2021_05_01);
    bench_solution!(group, 2021, 5, 2, year2021::day05::solution_2021_05_02);
    bench_solution!(group, 2021, 6, 1, year2021::day06::solution_2021_06_01);
    bench_solution!(group, 2021, 6, 2, year2021::day06::solution_2021_06_02);
    bench_solution!(group, 2021, 7, 1, year2021::day07::solution_2021_07_01);
    bench_solution!(group, 2021, 7, 2, year2021::day07::solution_2021_07_02);
    bench_solution!(group, 2021, 8, 1, year2021::day08::solution_2021_08_01);
    bench_solution!(group, 2021, 8, 2, year2021::day08::solution_2021_08_02);
    bench_solution!(group, 2021, 9, 1, year2021::day09::solution_2021_09_01);
    bench_solution!(group, 2021, 9, 2, year2021::day09::solution_2021_09_02);
    bench_solution!(group, 2021, 10, 1, year2021::day10::solution_2021_10_01);
    bench_solution!(group, 2021, 10, 2, year2021::day10::solution_2021_10_02);
    bench_solution!(group, 2021, 11, 1, year2021::day11::solution_2021_11_01);
    bench_solution!(group, 2021, 11, 2, year2021::day11::solution_2021_11_02);
    bench_solution!(group, 2021, 12, 1, year2021::day12::solution_2021_12_01);
    bench_solution!(group, 2021, 12, 2, year2021::day12::solution_2021_12_02);
    bench_solution!(group, 2021, 13, 1, year2021::day13::solution_2021_13_01);
    bench_solution!(group, 2021, 13, 2, year2021::day13::solution_2021_13_02);
    bench_solution!(group, 2021, 14, 1, year2021::day14::solution_2021_14_01);
    bench_solution!(group, 2021, 14, 2, year2021::day14::solution_2021_14_02);
    bench_solution!(group, 2021, 15, 1, year2021::day15::solution_2021_15_01);
    bench_solution!(group, 2021, 15, 2, year2021::day15::solution_2021_15_02);
    bench_solution!(group, 2021, 16, 1, year2021::day16::solution_2021_16_01);
    bench_solution!(group, 2021, 16, 2, year2021::day16::solution_2021_16_02);
    bench_solution!(group, 2021, 17, 1, year2021::day17::solution_2021_17_01);
    bench_solution!(group, 2021, 17, 2, year2021::day17::solution_2021_17_02);
    bench_solution!(group, 2021, 18, 1, year2021::day18::solution_2021_18_01);
    bench_solution!(group, 2021, 18, 2, year2021::day18::solution_2021_18_02);
    bench_solution!(group, 2021, 19, 1, year2021::day19::solution_2021_19_01);
    bench_solution!(group, 2021, 19, 2, year2021::day19::solution_2021_19_02);
    bench_solution!(group, 2021, 20, 1, year2021::day20::solution_2021_20_01);
    bench_solution!(group, 2021, 20, 2, year2021::day20::solution_2021_20_02);
    bench_solution!(group, 2021, 21, 1, year2021::day21::solution_2021_21_01);
    bench_solution!(group, 2021, 21, 2, year2021::day21::solution_2021_21_02);
    bench_solution!(group, 2021, 22, 1, year2021::day22::solution_2021_22_01);
    bench_solution!(group, 2021, 22, 2, year2021::day22::solution_2021_22_02);
    bench_solution!(group, 2021, 23, 1, year2021::day23::solution_2021_23_01);
    bench_solution!(group, 2021, 23, 2, year2021::day23::solution_2021_23_02);
    bench_solution!(group, 2021, 24, 1, year2021::day24::solution_2021_24_01);
    bench_solution!(group, 2021, 24, 2, year2021::day24::solution_2021_24_02);
    bench_solution!(group, 2021, 25, 1, year2021::day25::solution_2021_25_01);
    
    group.finish();
}

fn benchmark_2022(c: &mut Criterion) {
    let mut group = c.benchmark_group("2022");
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    
    // Year 2022
    bench_solution!(group, 2022, 1, 1, year2022::day01::solution_2022_01_01);
    bench_solution!(group, 2022, 1, 2, year2022::day01::solution_2022_01_02);
    bench_solution!(group, 2022, 2, 1, year2022::day02::solution_2022_02_01);
    bench_solution!(group, 2022, 2, 2, year2022::day02::solution_2022_02_02);
    bench_solution!(group, 2022, 3, 1, year2022::day03::solution_2022_03_01);
    bench_solution!(group, 2022, 3, 2, year2022::day03::solution_2022_03_02);
    bench_solution!(group, 2022, 4, 1, year2022::day04::solution_2022_04_01);
    bench_solution!(group, 2022, 4, 2, year2022::day04::solution_2022_04_02);
    bench_solution!(group, 2022, 5, 1, year2022::day05::solution_2022_05_01);
    bench_solution!(group, 2022, 5, 2, year2022::day05::solution_2022_05_02);
    bench_solution!(group, 2022, 6, 1, year2022::day06::solution_2022_06_01);
    bench_solution!(group, 2022, 6, 2, year2022::day06::solution_2022_06_02);
    bench_solution!(group, 2022, 7, 1, year2022::day07::solution_2022_07_01);
    bench_solution!(group, 2022, 7, 2, year2022::day07::solution_2022_07_02);
    bench_solution!(group, 2022, 8, 1, year2022::day08::solution_2022_08_01);
    bench_solution!(group, 2022, 8, 2, year2022::day08::solution_2022_08_02);
    bench_solution!(group, 2022, 9, 1, year2022::day09::solution_2022_09_01);
    bench_solution!(group, 2022, 9, 2, year2022::day09::solution_2022_09_02);
    bench_solution!(group, 2022, 10, 1, year2022::day10::solution_2022_10_01);
    bench_solution!(group, 2022, 10, 2, year2022::day10::solution_2022_10_02);
    bench_solution!(group, 2022, 11, 1, year2022::day11::solution_2022_11_01);
    bench_solution!(group, 2022, 11, 2, year2022::day11::solution_2022_11_02);
    bench_solution!(group, 2022, 12, 1, year2022::day12::solution_2022_12_01);
    bench_solution!(group, 2022, 12, 2, year2022::day12::solution_2022_12_02);
    bench_solution!(group, 2022, 13, 1, year2022::day13::solution_2022_13_01);
    bench_solution!(group, 2022, 13, 2, year2022::day13::solution_2022_13_02);
    bench_solution!(group, 2022, 14, 1, year2022::day14::solution_2022_14_01);
    bench_solution!(group, 2022, 14, 2, year2022::day14::solution_2022_14_02);
    // Day 15 has special signature with parameters
    let filepath = "inputs/2022/day15.txt".to_string();
    if std::path::Path::new(&filepath).exists() {
        group.bench_function("2022 Day 15 Part 1", |b| {
            b.iter(|| year2022::day15::solution_2022_15_01(black_box(filepath.clone()), black_box(2000000)))
        });
        group.bench_function("2022 Day 15 Part 2", |b| {
            b.iter(|| year2022::day15::solution_2022_15_02(black_box(filepath.clone()), black_box(4000000)))
        });
    }
    bench_solution!(group, 2022, 16, 1, year2022::day16::solution_2022_16_01);
    bench_solution!(group, 2022, 16, 2, year2022::day16::solution_2022_16_02);
    
    group.finish();
}

fn benchmark_2023(c: &mut Criterion) {
    let mut group = c.benchmark_group("2023");
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    
    // Year 2023
    bench_solution!(group, 2023, 1, 1, year2023::day01::solution_2023_01_01);
    bench_solution!(group, 2023, 1, 2, year2023::day01::solution_2023_01_02);
    bench_solution!(group, 2023, 2, 1, year2023::day02::solution_2023_02_01);
    bench_solution!(group, 2023, 2, 2, year2023::day02::solution_2023_02_02);
    bench_solution!(group, 2023, 3, 1, year2023::day03::solution_2023_03_01);
    bench_solution!(group, 2023, 3, 2, year2023::day03::solution_2023_03_02);
    bench_solution!(group, 2023, 4, 1, year2023::day04::solution_2023_04_01);
    bench_solution!(group, 2023, 4, 2, year2023::day04::solution_2023_04_02);
    bench_solution!(group, 2023, 5, 1, year2023::day05::solution_2023_05_01);
    bench_solution!(group, 2023, 5, 2, year2023::day05::solution_2023_05_02);
    bench_solution!(group, 2023, 6, 1, year2023::day06::solution_2023_06_01);
    bench_solution!(group, 2023, 6, 2, year2023::day06::solution_2023_06_02);
    bench_solution!(group, 2023, 7, 1, year2023::day07::solution_2023_07_01);
    bench_solution!(group, 2023, 7, 2, year2023::day07::solution_2023_07_02);
    bench_solution!(group, 2023, 8, 1, year2023::day08::solution_2023_08_01);
    bench_solution!(group, 2023, 8, 2, year2023::day08::solution_2023_08_02);
    bench_solution!(group, 2023, 9, 1, year2023::day09::solution_2023_09_01);
    bench_solution!(group, 2023, 9, 2, year2023::day09::solution_2023_09_02);
    bench_solution!(group, 2023, 10, 1, year2023::day10::solution_2023_10_01);
    bench_solution!(group, 2023, 10, 2, year2023::day10::solution_2023_10_02);
    // Day 11 not implemented yet
    bench_solution!(group, 2023, 12, 1, year2023::day12::solution_2023_12_01);
    bench_solution!(group, 2023, 12, 2, year2023::day12::solution_2023_12_02);
    bench_solution!(group, 2023, 13, 1, year2023::day13::solution_2023_13_01);
    bench_solution!(group, 2023, 13, 2, year2023::day13::solution_2023_13_02);
    bench_solution!(group, 2023, 14, 1, year2023::day14::solution_2023_14_01);
    bench_solution!(group, 2023, 14, 2, year2023::day14::solution_2023_14_02);
    bench_solution!(group, 2023, 15, 1, year2023::day15::solution_2023_15_01);
    bench_solution!(group, 2023, 15, 2, year2023::day15::solution_2023_15_02);
    bench_solution!(group, 2023, 16, 1, year2023::day16::solution_2023_16_01);
    bench_solution!(group, 2023, 16, 2, year2023::day16::solution_2023_16_02);
    bench_solution!(group, 2023, 17, 1, year2023::day17::solution_2023_17_01);
    bench_solution!(group, 2023, 17, 2, year2023::day17::solution_2023_17_02);
    bench_solution!(group, 2023, 18, 1, year2023::day18::solution_2023_18_01);
    bench_solution!(group, 2023, 18, 2, year2023::day18::solution_2023_18_02);
    bench_solution!(group, 2023, 19, 1, year2023::day19::solution_2023_19_01);
    bench_solution!(group, 2023, 19, 2, year2023::day19::solution_2023_19_02);
    bench_solution!(group, 2023, 20, 1, year2023::day20::solution_2023_20_01);
    bench_solution!(group, 2023, 20, 2, year2023::day20::solution_2023_20_02);
    // Day 21 has special signature with moves parameter
    let filepath = "inputs/2023/day21.txt".to_string();
    if std::path::Path::new(&filepath).exists() {
        group.bench_function("2023 Day 21 Part 1", |b| {
            b.iter(|| year2023::day21::solution_2023_21_01(black_box(filepath.clone()), black_box(64)))
        });
        group.bench_function("2023 Day 21 Part 2", |b| {
            b.iter(|| year2023::day21::solution_2023_21_02(black_box(filepath.clone()), black_box(26501365)))
        });
    }
    bench_solution!(group, 2023, 22, 1, year2023::day22::solution_2023_22_01);
    bench_solution!(group, 2023, 22, 2, year2023::day22::solution_2023_22_02);
    bench_solution!(group, 2023, 23, 1, year2023::day23::solution_2023_23_01);
    bench_solution!(group, 2023, 23, 2, year2023::day23::solution_2023_23_02);
    // Day 24 has special signature with range parameter
    let filepath = "inputs/2023/day24.txt".to_string();
    if std::path::Path::new(&filepath).exists() {
        group.bench_function("2023 Day 24 Part 1", |b| {
            b.iter(|| year2023::day24::solution_2023_24_01(black_box(filepath.clone()), black_box((200000000000000.0, 400000000000000.0))))
        });
    }
    bench_solution!(group, 2023, 24, 2, year2023::day24::solution_2023_24_02);
    bench_solution!(group, 2023, 25, 1, year2023::day25::solution_2023_25_01);
    // Day 25 part 2 typically doesn't exist (free star)
    
    group.finish();
}

fn benchmark_2024(c: &mut Criterion) {
    let mut group = c.benchmark_group("2024");
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    
    // Year 2024
    bench_solution!(group, 2024, 1, 1, year2024::day01::solution_2024_01_01);
    bench_solution!(group, 2024, 1, 2, year2024::day01::solution_2024_01_02);
    bench_solution!(group, 2024, 2, 1, year2024::day02::solution_2024_02_01);
    bench_solution!(group, 2024, 2, 2, year2024::day02::solution_2024_02_02);
    bench_solution!(group, 2024, 3, 1, year2024::day03::solution_2024_03_01);
    bench_solution!(group, 2024, 3, 2, year2024::day03::solution_2024_03_02);
    
    group.finish();
}

criterion_group!(
    name = benches_2025;
    config = custom_criterion();
    targets = benchmark_2025
);
criterion_group!(
    name = benches_2024;
    config = custom_criterion();
    targets = benchmark_2024
);
criterion_group!(
    name = benches_2023;
    config = custom_criterion();
    targets = benchmark_2023
);
criterion_group!(
    name = benches_2022;
    config = custom_criterion();
    targets = benchmark_2022
);
criterion_group!(
    name = benches_2021;
    config = custom_criterion();
    targets = benchmark_2021
);
criterion_group!(
    name = benches_2015;
    config = custom_criterion();
    targets = benchmark_2015
);
criterion_main!(benches_2025, benches_2024, benches_2023, benches_2022, benches_2021, benches_2015);
