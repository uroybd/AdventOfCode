use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use serde_json::Value;

type YearResults = BTreeMap<String, BTreeMap<String, BTreeMap<String, f64>>>;

fn parse_criterion_results(criterion_dir: &Path) -> YearResults {
    let mut results = BTreeMap::new();
    
    if !criterion_dir.exists() {
        return results;
    }
    
    // Function to process a directory looking for benchmark results
    fn process_directory(dir: &Path, results: &mut YearResults) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let dir_name = entry.file_name().to_string_lossy().to_string();
                
                if !path.is_dir() {
                    continue;
                }
                
                // Parse directory name: "YEAR Day DD Part P"
                let parts: Vec<&str> = dir_name.split_whitespace().collect();
                if parts.len() == 5 && parts[1] == "Day" && parts[3] == "Part" {
                    let year = parts[0].to_string();
                    let day = parts[2].to_string();
                    let part = parts[4].to_string();
                    
                    // Read the estimates.json file
                    let estimates_file = path.join("new/estimates.json");
                    if estimates_file.exists() {
                        if let Ok(data) = fs::read_to_string(&estimates_file) {
                            if let Ok(json) = serde_json::from_str::<Value>(&data) {
                                // Get median time in nanoseconds
                                if let Some(median_ns) = json["median"]["point_estimate"].as_f64() {
                                    results
                                        .entry(year)
                                        .or_insert_with(BTreeMap::new)
                                        .entry(day)
                                        .or_insert_with(BTreeMap::new)
                                        .insert(part, median_ns);
                                }
                            }
                        }
                    }
                } else {
                    // Recursively check subdirectories (for grouped benchmarks)
                    process_directory(&path, results);
                }
            }
        }
    }
    
    process_directory(criterion_dir, &mut results);
    results
}

fn format_time(ns: f64) -> String {
    if ns < 1000.0 {
        format!("{:.2} ns", ns)
    } else if ns < 1_000_000.0 {
        format!("{:.2} µs", ns / 1000.0)
    } else if ns < 1_000_000_000.0 {
        format!("{:.2} ms", ns / 1_000_000.0)
    } else {
        format!("{:.2} s", ns / 1_000_000_000.0)
    }
}

fn generate_table(year_results: &BTreeMap<String, BTreeMap<String, f64>>) -> (String, f64) {
    let mut lines = vec![
        "| Day | Part 1 | Part 2 | Total |".to_string(),
        "|-----|--------|--------|-------|".to_string(),
    ];
    
    let mut year_total = 0.0;
    
    for (day, parts) in year_results {
        let part1_ns = parts.get("1").copied().unwrap_or(0.0);
        let part2_ns = parts.get("2").copied().unwrap_or(0.0);
        let day_total = part1_ns + part2_ns;
        year_total += day_total;
        
        let part1 = if part1_ns > 0.0 {
            format_time(part1_ns)
        } else {
            "-".to_string()
        };
        let part2 = if part2_ns > 0.0 {
            format_time(part2_ns)
        } else {
            "-".to_string()
        };
        
        let day_num: i32 = day.parse().unwrap_or(0);
        lines.push(format!("| {} | {} | {} | {} |", 
            day_num, part1, part2, format_time(day_total)));
    }
    
    (lines.join("\n"), year_total)
}

fn update_readme(results: &YearResults) {
    let readme_path = "README.md";
    let content = fs::read_to_string(readme_path).unwrap();
    
    // Find benchmarks section
    let benchmark_start = content.find("## Benchmarks")
        .unwrap_or(content.len());
    
    let before = &content[..benchmark_start];
    
    // Build new benchmarks section
    let mut new_section = String::from("## Benchmarks\n\n");
    new_section.push_str("Run on: Apple M1 Max\n\n");
    
    // Add tables for each year in reverse order
    let mut years: Vec<_> = results.keys().collect();
    years.sort_by(|a, b| b.cmp(a));
    
    for year in years {
        new_section.push_str(&format!("### {}\n\n", year));
        let (table, total) = generate_table(&results[year]);
        new_section.push_str(&table);
        new_section.push_str(&format!("\n\n**Total runtime: {}**\n\n", format_time(total)));
    }
    
    // Write back
    fs::write(readme_path, format!("{}{}", before, new_section)).unwrap();
}

fn main() {
    let criterion_dir = Path::new("target/criterion");
    
    if !criterion_dir.exists() {
        eprintln!("Error: No criterion results found. Run 'cargo bench' first.");
        return;
    }
    
    println!("Parsing criterion results...");
    let results = parse_criterion_results(criterion_dir);
    
    println!("Found results for {} years", results.len());
    for (year, days) in &results {
        println!("  {}: {} days", year, days.len());
    }
    
    println!("\nUpdating README.md...");
    update_readme(&results);
    println!("Done!");
}
