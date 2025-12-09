use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchmarkResult {
    year: u32,
    day: u32,
    part1_ns: u64,
    part2_ns: u64,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkCache {
    results: HashMap<String, BenchmarkResult>,
}

impl BenchmarkCache {
    fn load() -> Self {
        let cache_path = "benchmark_cache.json";
        if Path::new(cache_path).exists() {
            let content = fs::read_to_string(cache_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| BenchmarkCache {
                results: HashMap::new(),
            })
        } else {
            BenchmarkCache {
                results: HashMap::new(),
            }
        }
    }

    fn save(&self) {
        let content = serde_json::to_string_pretty(self).unwrap();
        fs::write("benchmark_cache.json", content).unwrap();
    }

    fn get_or_benchmark(&mut self, year: u32, day: u32, force: bool) -> Option<BenchmarkResult> {
        let key = format!("{:04}_{:02}", year, day);
        
        // Check if cached result exists and is recent
        if !force {
            if let Some(cached) = self.results.get(&key) {
                if let Ok(timestamp) = chrono::DateTime::parse_from_rfc3339(&cached.timestamp) {
                    let age = chrono::Utc::now().signed_duration_since(timestamp);
                    if age.num_hours() < 24 {
                        println!("✓ Using cached result for {} day {}", year, day);
                        return Some(cached.clone());
                    }
                }
            }
        }

        // Check if solution exists
        let input_path = format!("inputs/{}/day{:02}.txt", year, day);
        if !Path::new(&input_path).exists() {
            return None;
        }

        // Run benchmark
        println!("⏱  Benchmarking {} day {}...", year, day);
        let result = self.run_benchmark(year, day)?;
        self.results.insert(key, result.clone());
        self.save();
        Some(result)
    }

    fn run_benchmark(&self, year: u32, day: u32) -> Option<BenchmarkResult> {
        let part1_ns = self.benchmark_function(year, day, 1)?;
        let part2_ns = self.benchmark_function(year, day, 2)?;

        Some(BenchmarkResult {
            year,
            day,
            part1_ns,
            part2_ns,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }

    fn benchmark_function(&self, year: u32, day: u32, part: u32) -> Option<u64> {
        // First compile the test in release mode
        let test_name = format!("output_{}_{:02}_{:02}", year, day, part);
        
        let compile = Command::new("cargo")
            .args(&["test", &test_name, "--release", "--no-run"])
            .output()
            .ok()?;
        
        if !compile.status.success() {
            println!("  ✗ Part {} test not found or compilation failed", part);
            return None;
        }

        // Get the test binary path
        let test_binary = self.find_test_binary()?;
        
        // Run 100 iterations and collect timings
        const ITERATIONS: usize = 100;
        let mut times = Vec::with_capacity(ITERATIONS);
        
        print!("  Running part {} {} times... ", part, ITERATIONS);
        std::io::Write::flush(&mut std::io::stdout()).ok()?;
        
        for i in 0..ITERATIONS {
            if i % 10 == 0 && i > 0 {
                print!("{}", i);
                std::io::Write::flush(&mut std::io::stdout()).ok()?;
            } else if i % 10 == 0 {
                print!(".");
            }
            
            let start = Instant::now();
            let output = Command::new(&test_binary)
                .args(&[&test_name, "--ignored", "--nocapture", "--exact"])
                .output()
                .ok()?;
            
            if output.status.success() {
                times.push(start.elapsed().as_nanos() as u64);
            }
        }
        println!(" done");
        
        if times.is_empty() {
            return None;
        }
        
        // Calculate median
        times.sort_unstable();
        let median = times[times.len() / 2];
        
        // Calculate mean for reference
        let mean = times.iter().sum::<u64>() / times.len() as u64;
        let min = *times.first().unwrap();
        let max = *times.last().unwrap();
        
        println!("  ✓ Part {}: min={:.2}ms, median={:.2}ms, mean={:.2}ms, max={:.2}ms", 
            part,
            min as f64 / 1_000_000.0,
            median as f64 / 1_000_000.0,
            mean as f64 / 1_000_000.0,
            max as f64 / 1_000_000.0
        );
        
        Some(median)
    }

    fn find_test_binary(&self) -> Option<String> {
        // Find the test binary in target/release/deps
        let output = Command::new("cargo")
            .args(&["test", "--release", "--no-run", "--message-format=json"])
            .output()
            .ok()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(executable) = json["executable"].as_str() {
                    if executable.contains("AOC") && executable.contains("deps") {
                        return Some(executable.to_string());
                    }
                }
            }
        }
        
        None
    }

    fn find_all_solutions(&self) -> Vec<(u32, u32)> {
        let mut solutions = Vec::new();
        
        for year in [2015, 2022, 2023, 2024, 2025] {
            for day in 1..=25 {
                let input_path = format!("inputs/{}/day{:02}.txt", year, day);
                if Path::new(&input_path).exists() {
                    solutions.push((year, day));
                }
            }
        }
        
        solutions
    }
}

fn format_time(ns: u64) -> String {
    if ns < 1_000 {
        format!("{}ns", ns)
    } else if ns < 1_000_000 {
        format!("{:.2}µs", ns as f64 / 1_000.0)
    } else if ns < 1_000_000_000 {
        format!("{:.2}ms", ns as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", ns as f64 / 1_000_000_000.0)
    }
}

fn generate_readme_table(cache: &BenchmarkCache) -> String {
    let mut output = String::new();
    output.push_str("# Advent of Code\n\n");
    output.push_str("This repository contains my solutions to the [Advent of Code](https://adventofcode.com/) challenges.\n\n");
    
    // Group by year
    let mut years: Vec<u32> = cache.results.values().map(|r| r.year).collect();
    years.sort_unstable();
    years.dedup();
    
    if years.is_empty() {
        output.push_str("*No benchmark results yet. Run `cargo run --release --bin benchmark all` to generate.*\n\n");
        return output;
    }
    
    for year in years.iter().rev() {
        output.push_str(&format!("## Year {} - Runtime\n\n", year));
        output.push_str("*Benchmarks run on: Apple M1 Pro (Release mode, median of 100 runs)*\n\n");
        output.push_str("| Day/Part | Part 1 | Part 2 | Total |\n");
        output.push_str("|:---------|-------:|-------:|------:|\n");
        
        let mut year_results: Vec<_> = cache.results.values()
            .filter(|r| r.year == *year)
            .collect();
        year_results.sort_by_key(|r| r.day);
        
        let mut year_total_ns = 0u64;
        for result in year_results {
            let total_ns = result.part1_ns + result.part2_ns;
            year_total_ns += total_ns;
            
            output.push_str(&format!(
                "| **Day {:02}** | {} | {} | {} |\n",
                result.day,
                format_time(result.part1_ns),
                format_time(result.part2_ns),
                format_time(total_ns)
            ));
        }
        
        output.push_str(&format!("\n**Total runtime: {}**\n\n", format_time(year_total_ns)));
    }
    
    output
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut cache = BenchmarkCache::load();
    
    let force = args.contains(&"--force".to_string());
    
    if args.len() > 1 && (args[1] == "all" || args[1] == "--force") {
        // Benchmark all available solutions
        let solutions = cache.find_all_solutions();
        println!("Found {} solutions to benchmark\n", solutions.len());
        
        for (year, day) in solutions {
            cache.get_or_benchmark(year, day, force);
        }
    } else if args.len() > 2 {
        // Benchmark specific year/day
        let year: u32 = args[1].parse().expect("Invalid year");
        let day: u32 = args[2].parse().expect("Invalid day");
        cache.get_or_benchmark(year, day, force);
    } else {
        println!("Usage:");
        println!("  cargo run --bin benchmark all            # Benchmark all available days");
        println!("  cargo run --bin benchmark all --force    # Force re-benchmark all days");
        println!("  cargo run --bin benchmark <year> <day>   # Benchmark specific day");
        println!("  cargo run --bin benchmark <year> <day> --force   # Force re-benchmark specific day");
        return;
    }
    
    // Generate and save README
    let readme = generate_readme_table(&cache);
    fs::write("README.md", readme).expect("Failed to write README.md");
    println!("\n✅ README.md updated with benchmark results!");
}

