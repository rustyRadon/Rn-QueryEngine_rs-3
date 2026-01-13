use crate::storage::column::ColumnVault;
use crate::util::bitmask::BitMask;
use rayon::prelude::*;
use bytemuck;

/// Parallel Integer Sum
pub fn crunch_integer_sum(vault: &ColumnVault, mask: &BitMask) -> i64 {
    if let ColumnVault::WholeNumbers(store) = vault {
        let numbers: &[i32] = bytemuck::cast_slice(store.as_bytes());
        
        numbers.par_iter()
            .enumerate()
            .map(|(i, &val)| {
                if mask.get(i) { val as i64 } else { 0 }
            })
            .sum()
    } else {
        0
    }
}

///Parallel Float Sum 
pub fn crunch_float_sum(vault: &ColumnVault, mask: &BitMask) -> f64 {
    if let ColumnVault::MoneyAndScores(store) = vault {
        let numbers: &[f64] = bytemuck::cast_slice(store.as_bytes());
        
        numbers.par_iter()
            .enumerate()
            .map(|(i, &val)| {
                if mask.get(i) { val } else { 0.0 }
            })
            .sum()
    } else {
        0.0
    }
}

/// Row Screener
pub fn screen_for_matches<F>(vault: &ColumnVault, mut rule: F) -> BitMask 
where 
    F: FnMut(i32) -> bool 
{
    if let ColumnVault::WholeNumbers(store) = vault {
        let numbers: &[i32] = bytemuck::cast_slice(store.as_bytes());
        let mut mask = BitMask::new(numbers.len());

        for (i, &val) in numbers.iter().enumerate() {
            if rule(val) {
                mask.set(i);
            }
        }
        return mask;
    }
    BitMask::new(0)
}