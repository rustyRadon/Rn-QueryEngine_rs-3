mod catalog;
mod storage;
mod compute;
mod util;

use crate::catalog::schema::TableSchema;
use crate::storage::column::{ColumnManager, ColumnVault};
use crate::compute::functions::{screen_for_matches, crunch_float_sum};
use std::env;
use std::time::Instant;
use bytemuck;

fn main() -> Result<(), String> {
    let schema = TableSchema::from_file("metadata.json")?;
    
    let args: Vec<String> = env::args().collect();
    let target_dept = args
        .get(1)
        .and_then(|s| s.parse::<i32>()
        .ok()).unwrap_or(1);
    let min_age = args
        .get(2)
        .and_then(|s| s.parse::<i32>()
        .ok())
        .unwrap_or(18);
    let min_salary = args
        .get(3) 
        .and_then(|s| s.parse::<f64>()
        .ok())
        .unwrap_or(0.0);

    let dept_info = schema
        .get_column("dept")
        .ok_or("Dept column missing")?;
    let age_info = schema
        .get_column("age")
        .ok_or("Age column missing")?;
    let sal_info = schema
        .get_column("salary").ok_or("Salary column missing")?;

    let dept_m = ColumnManager::load_from_disk(&dept_info.name, &dept_info.data_type, &dept_info.file)?;
    let age_m = ColumnManager::load_from_disk(&age_info.name, &age_info.data_type, &age_info.file)?;
    let sal_m = ColumnManager::load_from_disk(&sal_info.name, &sal_info.data_type, &sal_info.file)?;

    let start_time = Instant::now();

    let mut mask = screen_for_matches(&dept_m.vault, |id| id == target_dept);

    if let ColumnVault::WholeNumbers(store) = &age_m.vault {
        let age_data: &[i32] = bytemuck::cast_slice(store.as_bytes());
        for (i, &age) in age_data.iter().enumerate() {
            if mask.get(i) && age < min_age {
                mask.unset(i); 
            }
        }
    }

    if let ColumnVault::MoneyAndScores(store) = &sal_m.vault {
        let sal_data: &[f64] = bytemuck::cast_slice(store.as_bytes());
        for (i, &sal) in sal_data.iter().enumerate() {
            if mask.get(i) && sal < min_salary {
                mask.unset(i);
            }
        }
    }

    let total_salary = crunch_float_sum(&sal_m.vault, &mask);
    let match_count = mask.count_active();
    let duration = start_time.elapsed();

    println!("--- Engine: {} ---", schema.name);
    println!("Criteria      : Dept {}, Age > {}, Sal > ${}", target_dept, min_age, min_salary);
    println!("Rows Matched  : {}", match_count);
    
    if match_count > 0 {
        println!("Total Payroll : ${:.2}", total_salary);
        println!("Average Salary: ${:.2}", total_salary / (match_count as f64));
    }

    println!("-----------------");
    println!("Execution Time: {:.2?}", duration);
    println!("Throughput    : {} rows/sec", (1_000_000.0 / duration.as_secs_f64()) as u64);

    Ok(())
}