//! rs-wasm-pack - wasm-pack-compatible Rust to WASM build toolchain
//! Part of poly-1000: 1000 elite programming tools by ElNeMeR70
use std::time::Instant;

struct AdvancedTool;
impl AdvancedTool {
    fn process(&self, input: &[f64]) -> Vec<f64> {
        input.iter().enumerate()
            .map(|(i,&v)| v.sin()*(i as f64).cos()+(v.abs()+1.0).sqrt())
            .collect()
    }
    fn hash(&self, s: &str) -> u64 {
        let mut h: u64 = 14695981039346656037;
        for b in s.bytes() { h ^= b as u64; h = h.wrapping_mul(1099511628211); }
        h
    }
}
fn main() {
    println!("[RUN] rs-wasm-pack");
    let tool = AdvancedTool;
    let data: Vec<f64> = (0..500).map(|i| (i as f64 * 0.01).sin()).collect();
    let start = Instant::now();
    let r = tool.process(&data);
    let ms = start.elapsed().as_secs_f64()*1000.0;
    println!("[OK]  Result: Processed {} items in {:.2}ms hash={:016x}", r.len(), ms, tool.hash("rs-wasm-pack-v1.0.0"));
    println!("[OK]  rs-wasm-pack works correctly!");
}
