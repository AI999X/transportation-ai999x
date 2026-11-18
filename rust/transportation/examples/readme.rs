//! Transportation: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p transportation-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&transportation_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "transport_braking_distance", "inputs": {"v": "27.77777778 m/s", "a": "3.4 m/s^2"}}));
    println!();
    show(json!({"key": "transport_freeway_level_of_service", "inputs": {"V": "4000 1/h", "PHF": "0.92", "N": "3", "P_T": "0.1", "E_T": "2", "S": "27.77777778 m/s"}}));
    println!();
    show(json!({"key": "transport_clothoid_coordinates", "inputs": {"R": "300 m", "L_s": "80 m"}}));
    println!();
    show(json!({"key": "transport_logit_elasticities", "inputs": {"V": [-1.2, -0.8, -2.1], "x": [30, 45, 20], "beta": "-0.04"}}));
    println!();
    show(json!({"key": "transport_equivalent_single_axle_loads", "inputs": {"loads": [80, 100, 120], "counts": [500, 200, 100], "Y": "20"}}));
    println!();
    show(json!({"key": "transport_erlang_b_blocking", "inputs": {"lam": "60 1/h", "mu": "0.5 1/h", "c": "120"}}));
    println!();
    show(json!({"key": "transport_rail_station_headway", "inputs": {"v": "22.22222222 m/s", "a": "1 m/s^2", "b": "1.1 m/s^2", "L_t": "150 m", "t_d": "30 s", "s_m": "50 m"}}));
    println!();
    show(json!({"key": "transport_fastest_route", "inputs": {"roads": "A-B 4, A-C 2.5, B-C 1.2, B-D 5, C-D 8, C-E 10, D-E 2", "speeds": "60, 50, 40, 80, 100, 90, 50", "origin": "A", "destination": "E"}}));
    println!();
    show(json!({"key": "transport_design_hourly_volume", "inputs": {"AADT": "40000", "K": "0.1", "D": "0.6"}}));
    println!();
    show(json!({"key": "transport_greenberg_fit", "inputs": {"speeds": [82, 70, 58, 47, 33, 21], "densities": [18, 32, 46, 60, 78, 95]}}));
    println!();
    show(json!({"key": "transport_signal_green_splits", "inputs": {"C": "90 s", "L": "10 s", "y": [0.3, 0.25, 0.15]}}));
    println!();
    show(json!({"key": "transport_spot_speed_percentiles", "inputs": {"speeds": [52, 61, 48, 70, 55, 63, 58, 66, 49, 72]}}));
    println!();
    show(json!({"key": "transport_all_or_nothing_assignment", "inputs": {"network": "A-B 5, B-A 5, A-C 3, C-A 3, B-D 4, D-B 4, C-D 7, D-C 7, B-C 1, C-B 1", "od": "A-D 800, C-B 200, D-A 300"}}));
    println!();
    show(json!({"key": "transport_gap_acceptance_capacity", "inputs": {"q": "800 1/h", "t_c": "6.5 s", "t_f": "3.3 s"}}));
    println!();
    show(json!({"key": "transport_lateral_load_transfer", "inputs": {"m": "1500 kg", "a_y": "0.6 m/s^2", "h": "0.55 m", "t_w": "1.55 m"}}));
}
