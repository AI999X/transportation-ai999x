//! transportation: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/transportation/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "transport_acceleration_distance",
    "transport_acceleration_under_power",
    "transport_all_or_nothing_assignment",
    "transport_bottleneck_queue",
    "transport_braking_distance",
    "transport_braking_weight_transfer",
    "transport_braking_with_air_resistance",
    "transport_clothoid_coordinates",
    "transport_coast_down",
    "transport_crest_curve_length",
    "transport_crest_curve_sight_distance",
    "transport_curve_safe_speed",
    "transport_design_hourly_volume",
    "transport_equivalent_single_axle_loads",
    "transport_erlang_b_blocking",
    "transport_fastest_route",
    "transport_fifo_queue_simulation",
    "transport_freeway_level_of_service",
    "transport_furness_balancing",
    "transport_gap_acceptance_capacity",
    "transport_gravity_model",
    "transport_greenberg_capacity",
    "transport_greenberg_fit",
    "transport_greenshields_capacity",
    "transport_greenshields_fit",
    "transport_greenshields_shockwave",
    "transport_greenshields_speed_at_flow",
    "transport_headway_probability",
    "transport_horizontal_sight_offset",
    "transport_impact_speed",
    "transport_interval_queue",
    "transport_lateral_load_transfer",
    "transport_logit_elasticities",
    "transport_logit_from_attributes",
    "transport_logit_mode_shares",
    "transport_logsum_user_benefit",
    "transport_md1_queue",
    "transport_minimum_curve_radius",
    "transport_mm1_queue",
    "transport_mmc_queue",
    "transport_nested_logit",
    "transport_overtaking_manoeuvre",
    "transport_passing_sight_distance",
    "transport_peak_hour_factor",
    "transport_poisson_arrivals",
    "transport_quarter_car_frequencies",
    "transport_quarter_car_transmissibility",
    "transport_rail_station_headway",
    "transport_rollover_threshold",
    "transport_safe_following_gap",
    "transport_sag_curve_headlight",
    "transport_shockwave_speed",
    "transport_side_friction_demand",
    "transport_sight_distance_from_offset",
    "transport_signal_green_splits",
    "transport_signal_queue_shockwave",
    "transport_signal_queue_uniform_delay",
    "transport_simplified_curve_formula",
    "transport_skid_speed",
    "transport_speed_reduction_distance",
    "transport_spot_speed_percentiles",
    "transport_steady_state_cornering",
    "transport_steer_angle_for_radius",
    "transport_stopping_sight_distance",
    "transport_superelevation_required",
    "transport_time_and_space_mean_speed",
    "transport_top_speed",
    "transport_traction_limited_acceleration",
    "transport_train_acceleration",
    "transport_train_braking_distance",
    "transport_train_running_time",
    "transport_travel_time_reliability",
    "transport_travel_time_skim",
    "transport_trip_generation_regression",
    "transport_understeer_gradient",
    "transport_underwood_capacity",
    "transport_underwood_fit",
    "transport_vehicle_resistance",
    "transport_webster_optimum_cycle",
    "transport_yaw_natural_frequency",
    "transport_yellow_change_interval",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "transport_acceleration_distance" => {
            let r = f_accel_distance(match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_acceleration_under_power" => {
            let r = f_power_accel(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.2)) })().ok()? } }, match v.get("f_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.015)) })().ok()? } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("eta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_all_or_nothing_assignment" => {
            let r = f_aon(match v.get("network") { Some(x) => py::from_val(x), None => { return None } }, match v.get("od") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_bottleneck_queue" => {
            let r = f_bottleneck(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_braking_distance" => {
            let r = f_braking_distance(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_braking_weight_transfer" => {
            let r = f_brake_transfer(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l_f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_braking_with_air_resistance" => {
            let r = f_brake_resist(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.2)) })().ok()? } }, match v.get("f_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.01)) })().ok()? } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("eta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.04)) })().ok()? } }, match v.get("v2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_clothoid_coordinates" => {
            let r = f_clothoid(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_coast_down" => {
            let r = f_coast_down(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.2)) })().ok()? } }, match v.get("f_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.012)) })().ok()? } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_crest_curve_length" => {
            let r = f_crest_length(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.08)) })().ok()? } }, match v.get("h2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.6)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_crest_curve_sight_distance" => {
            let r = f_crest_sight(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.08)) })().ok()? } }, match v.get("h2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.6)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_curve_safe_speed" => {
            let r = f_curve_speed(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_design_hourly_volume" => {
            let r = f_design_hourly_volume(match v.get("AADT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_equivalent_single_axle_loads" => {
            let r = f_esal(match v.get("loads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(80000.0)) })().ok()? } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(4.0)) })().ok()? } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("f_d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_erlang_b_blocking" => {
            let r = f_erlang_b(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_fastest_route" => {
            let r = f_fastest_route(match v.get("roads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("origin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("destination") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_fifo_queue_simulation" => {
            let r = f_fifo_queue(match v.get("arrivals") { Some(x) => py::from_val(x), None => { return None } }, match v.get("service") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_freeway_level_of_service" => {
            let r = f_freeway_los(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PHF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f_p") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_furness_balancing" => {
            let r = f_furness(match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("O") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_gap_acceptance_capacity" => {
            let r = f_gap_capacity(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("harders")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_gravity_model" => {
            let r = f_gravity(match v.get("O") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("costs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("deterrence") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("exponential")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_greenberg_capacity" => {
            let r = f_greenberg_cap(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kj") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_greenberg_fit" => {
            let r = f_greenberg_fit(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("densities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_greenshields_capacity" => {
            let r = f_greenshields_cap(match v.get("vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kj") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_greenshields_fit" => {
            let r = f_greenshields_fit(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("densities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_greenshields_shockwave" => {
            let r = f_gs_shockwave(match v.get("vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kj") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_greenshields_speed_at_flow" => {
            let r = f_greenshields_at_flow(match v.get("vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kj") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_headway_probability" => {
            let r = f_headway_prob(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_horizontal_sight_offset" => {
            let r = f_hso(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_impact_speed" => {
            let r = f_impact_speed(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_interval_queue" => {
            let r = f_interval_queue(match v.get("arrivals") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_lateral_load_transfer" => {
            let r = f_lateral_transfer(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_logit_elasticities" => {
            let r = f_logit_elasticities(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_logit_from_attributes" => {
            let r = f_logit_attributes(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("asc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_logit_mode_shares" => {
            let r = f_logit_shares(match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_logsum_user_benefit" => {
            let r = f_logsum_benefit(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta_cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_md1_queue" => {
            let r = f_md1(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_minimum_curve_radius" => {
            let r = f_curve_radius(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_mm1_queue" => {
            let r = f_mm1(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_mmc_queue" => {
            let r = f_mmc(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_nested_logit" => {
            let r = f_nested_logit(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_overtaking_manoeuvre" => {
            let r = f_overtaking(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_passing_sight_distance" => {
            let r = f_passing_sight(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_diff") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d3") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_peak_hour_factor" => {
            let r = f_peak_hour_factor(match v.get("counts") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_poisson_arrivals" => {
            let r = f_poisson_arrivals(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_quarter_car_frequencies" => {
            let r = f_quarter_car_freq(match v.get("m_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_quarter_car_transmissibility" => {
            let r = f_quarter_car_tr(match v.get("m_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_rail_station_headway" => {
            let r = f_station_headway(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s_m") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_rollover_threshold" => {
            let r = f_rollover(match v.get("t_w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_safe_following_gap" => {
            let r = f_following_gap(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_sag_curve_headlight" => {
            let r = f_sag_headlight(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_shockwave_speed" => {
            let r = f_shockwave(match v.get("q1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_side_friction_demand" => {
            let r = f_side_friction(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_sight_distance_from_offset" => {
            let r = f_hso_sight(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_signal_green_splits" => {
            let r = f_green_splits(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_signal_queue_shockwave" => {
            let r = f_signal_shock_queue(match v.get("q_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_j") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_signal_queue_uniform_delay" => {
            let r = f_signal_dd1(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_simplified_curve_formula" => {
            let r = f_simplified_curve(match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_skid_speed" => {
            let r = f_skid_speed(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_speed_reduction_distance" => {
            let r = f_slow_distance(match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_spot_speed_percentiles" => {
            let r = f_speed_percentiles(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_steady_state_cornering" => {
            let r = f_cornering(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_steer_angle_for_radius" => {
            let r = f_steer_for_radius(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_stopping_sight_distance" => {
            let r = f_ssd(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_superelevation_required" => {
            let r = f_superelevation(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_time_and_space_mean_speed" => {
            let r = f_mean_speeds(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_top_speed" => {
            let r = f_top_speed(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.2)) })().ok()? } }, match v.get("f_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.015)) })().ok()? } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("eta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_traction_limited_acceleration" => {
            let r = f_traction_limit(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("drive") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("rear")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_train_acceleration" => {
            let r = f_train_accel(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.08)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_train_braking_distance" => {
            let r = f_train_braking(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.06)) })().ok()? } }, match v.get("v2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_train_running_time" => {
            let r = f_running_time(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_travel_time_reliability" => {
            let r = f_reliability(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_free") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_travel_time_skim" => {
            let r = f_skim(match v.get("roads") { Some(x) => py::from_val(x), None => { return None } }, match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_trip_generation_regression" => {
            let r = f_trip_regression(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_new") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_understeer_gradient" => {
            let r = f_understeer(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_underwood_capacity" => {
            let r = f_underwood_cap(match v.get("vf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_underwood_fit" => {
            let r = f_underwood_fit(match v.get("speeds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("densities") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_vehicle_resistance" => {
            let r = f_vehicle_resistance(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.2)) })().ok()? } }, match v.get("f_r") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.015)) })().ok()? } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_webster_optimum_cycle" => {
            let r = f_webster_cycle(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_yaw_natural_frequency" => {
            let r = f_yaw_modes(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Iz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "transport_yellow_change_interval" => {
            let r = f_yellow_interval(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_G() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.81)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G_() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.81)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LOS_LIMITS_MI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(11.0), Py::Float(18.0), Py::Float(26.0), Py::Float(35.0), Py::Float(45.0)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LOS_NAMES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("A"), Py::str("B"), Py::str("C"), Py::str("D"), Py::str("E")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__balance_check(mut v_O: Py, mut v_D: Py) -> R {
    v_O = f__vec(Py::str("productions"), v_O.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_D = f__vec(Py::str("attractions"), v_D.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t741 = py::abs(&py::sub(&py::sum(&v_O.clone(), None)?, &py::sum(&v_D.clone(), None)?)?)?; Py::Bool({ let t742 = py::mul(&Py::Float(1e-09), &py::sum(&v_O.clone(), None)?)?; if py::le(&t741, &t742)? { true } else { false } }) }, Py::str("total productions must equal total attractions"))?;
    return Ok(Py::tuple(vec![v_O.clone(), v_D.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bike_check(mut v_m: Py, mut v_L: Py, mut v_l_f: Py, mut v_C_f: Py, mut v_C_r: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("C_f"), v_C_f.clone()), (Py::str("C_r"), v_C_r.clone())]))?;
    let _ = f__check_axles(v_L.clone(), v_l_f.clone())?;
    return Ok(Py::tuple(vec![py::sub(&v_L.clone(), &v_l_f.clone())?, py::mul(&v_m.clone(), &py::sub(&py::truediv(&py::sub(&v_L.clone(), &v_l_f.clone())?, &py::mul(&v_L.clone(), &v_C_f.clone())?)?, &py::truediv(&v_l_f.clone(), &py::mul(&v_L.clone(), &v_C_r.clone())?)?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect_root(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_iters: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t46 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t46.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t46[0].clone();
    v_fb = t46[1].clone();
    if ({ let t47 = v_fa.clone(); Py::Bool({ let t48 = Py::Int(0i64); if py::eq(&t47, &t48) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t49 = v_fb.clone(); Py::Bool({ let t50 = Py::Int(0i64); if py::eq(&t49, &t50) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t51 = py::mul(&v_fa.clone(), &v_fb.clone())?; Py::Bool({ let t52 = Py::Int(0i64); if py::lt(&t51, &t52)? { true } else { false } }) }, Py::str("no root in the admissible range"))?;
    for t53 in py::range_iter(&[(v_iters.clone()).i()?]) {
        v__ = t53;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t58 = { let t54 = v_m.clone(); Py::Bool({ let t55 = v_a.clone(); if py::eq(&t54, &t55) { true } else { false } }) }; if t58.truthy() { t58 } else { { let t56 = v_m.clone(); Py::Bool({ let t57 = v_b.clone(); if py::eq(&t56, &t57) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t59 = v_fm.clone(); Py::Bool({ let t60 = Py::Int(0i64); if py::eq(&t59, &t60) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t61 = { let t63 = v_fm.clone(); Py::Bool({ let t64 = Py::Int(0i64); if py::lt(&t63, &t64)? { true } else { false } }) }; Py::Bool({ let t62 = { let t65 = v_fa.clone(); Py::Bool({ let t66 = Py::Int(0i64); if py::lt(&t65, &t66)? { true } else { false } }) }; if py::eq(&t61, &t62) { true } else { false } }) }).truthy() {
            let t67 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t67.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t67[0].clone();
            v_fa = t67[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_axles(mut v_L: Py, mut v_l_f: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("l_f"), v_l_f.clone())]))?;
    let _ = f__need({ let t2539 = v_l_f.clone(); Py::Bool({ let t2540 = v_L.clone(); if py::lt(&t2539, &t2540)? { true } else { false } }) }, Py::str("the distance from the front axle to the centre of gravity must be less than the wheelbase"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_obs(mut v_speeds: Py, mut v_densities: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = f__vec(Py::str("speeds"), v_speeds.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_k = f__vec(Py::str("densities"), v_densities.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t2448 = py::len(&v_v.clone())?; Py::Bool({ let t2449 = py::len(&v_k.clone())?; if py::eq(&t2448, &t2449) { true } else { false } }) }, Py::str("give one density for every speed"))?;
    let _ = f__need({ let t2450 = py::len(&v_v.clone())?; Py::Bool({ let t2451 = Py::Int(3i64); if py::ge(&t2450, &t2451)? { true } else { false } }) }, Py::str("give at least three observations"))?;
    return Ok(Py::tuple(vec![v_v.clone(), v_k.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__crest_k(mut v_h1: Py, mut v_h2: Py) -> R {
    return Ok(py::pow(&py::add(&py::math1("sqrt", &v_h1.clone())?, &py::math1("sqrt", &v_h2.clone())?)?, &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__davis(mut v_Ad: Py, mut v_Bd: Py, mut v_Cd: Py) -> R {
    let _ = f__nonneg(Py::dict(vec![(Py::str("A"), v_Ad.clone()), (Py::str("B"), v_Bd.clone()), (Py::str("C"), v_Cd.clone())]))?;
    return Ok({ let v_Ad = v_Ad.clone(); let v_Bd = v_Bd.clone(); let v_Cd = v_Cd.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&v_Ad.clone(), &py::mul(&v_Bd.clone(), &v_v.clone())?)?, &py::mul(&py::mul(&v_Cd.clone(), &v_v.clone())?, &v_v.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__decel_drag(mut v_m: Py, mut v_F0: Py, mut v_K: Py, mut v_gamma: Py, mut v_v1: Py, mut v_v2: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_gm = py::mul(&v_gamma.clone(), &v_m.clone())?;
    v_S = py::mul(&py::truediv(&v_gm.clone(), &py::mul(&Py::Int(2i64), &v_K.clone())?)?, &py::math1("log1p", &py::truediv(&py::mul(&v_K.clone(), &py::sub(&py::mul(&v_v1.clone(), &v_v1.clone())?, &py::mul(&v_v2.clone(), &v_v2.clone())?)?)?, &py::add(&v_F0.clone(), &py::mul(&py::mul(&v_K.clone(), &v_v2.clone())?, &v_v2.clone())?)?)?)?)?;
    v_c = py::math1("sqrt", &py::truediv(&v_K.clone(), &v_F0.clone())?)?;
    v_t = py::mul(&py::truediv(&v_gm.clone(), &py::math1("sqrt", &py::mul(&v_K.clone(), &v_F0.clone())?)?)?, &py::sub(&py::math1("atan", &py::mul(&v_v1.clone(), &v_c.clone())?)?, &py::math1("atan", &py::mul(&v_v2.clone(), &v_c.clone())?)?)?)?;
    return Ok(Py::tuple(vec![v_S.clone(), v_t.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dijkstra(mut v_adj: Py, mut v_src: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cand: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_done: Py = Py::None;
    let mut v_heap: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_best = Py::dict(vec![(v_src.clone(), Py::tuple(vec![Py::Float(0.0), Py::Int(0i64), Py::list(vec![v_src.clone()]), Py::Float(0.0)]))]);
    v_heap = Py::list(vec![Py::tuple(vec![Py::Float(0.0), Py::Int(0i64), Py::list(vec![v_src.clone()]), Py::Float(0.0)])]);
    v_done = py::set(&Py::list(vec![]))?;
    while (v_heap.clone()).truthy() {
        let t2287 = (py::heapq("heappop", &[v_heap.clone()])?).items()?;
        if t2287.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_t = t2287[0].clone();
        v_n = t2287[1].clone();
        v_path = t2287[2].clone();
        v_dist = t2287[3].clone();
        v_u = py::getitem(&v_path.clone(), &py::neg(&Py::Int(1i64))?)?;
        if ({ let t2288 = v_u.clone(); Py::Bool({ let t2289 = v_done.clone(); if py::contains(&t2289, &t2288)? { true } else { false } }) }).truthy() {
            continue;
        }
        let _ = py::method(&v_done.clone(), "add", &[v_u.clone()])?;
        for t2290 in py::iter(&(py::method(&py::getitem(&v_adj.clone(), &v_u.clone())?, "items", &[])?))? {
            let t2296 = (t2290).items()?;
            if t2296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t2296[0].clone();
            let t2297 = (t2296[1].clone()).items()?;
            if t2297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_w = t2297[0].clone();
            v_L = t2297[1].clone();
            v_cand = Py::tuple(vec![py::add(&v_t.clone(), &v_w.clone())?, py::add(&v_n.clone(), &Py::Int(1i64))?, py::add(&v_path.clone(), &Py::list(vec![v_v.clone()]))?, py::add(&v_dist.clone(), &v_L.clone())?]);
            if ({ let t2295 = { let t2291 = v_v.clone(); Py::Bool({ let t2292 = v_best.clone(); if !py::contains(&t2292, &t2291)? { true } else { false } }) }; if t2295.truthy() { t2295 } else { { let t2293 = py::slice(&v_cand.clone(), None, Some((Py::Int(3i64)).i()?), None)?; Py::Bool({ let t2294 = py::slice(&py::getitem(&v_best.clone(), &v_v.clone())?, None, Some((Py::Int(3i64)).i()?), None)?; if py::lt(&t2293, &t2294)? { true } else { false } }) } } }).truthy() {
                py::setitem(&v_best.clone(), &v_v.clone(), v_cand.clone())?;
                let _ = py::heapq("heappush", &[v_heap.clone(), v_cand.clone()])?;
            }
        }
    }
    return Ok(v_best.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__e_check(mut v_e: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("e"), v_e.clone())]))?;
    let _ = f__need({ let t1688 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1689 = v_e.clone(); if py::lt(&t1688, &t1689)? { { let t1690 = Py::Int(1i64); if py::lt(&t1689, &t1690)? { true } else { false } } } else { false } }) }, Py::str("the superelevation rate must lie between -1 and 1 (a decimal or a percentage)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t2553 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t2556 = (t2553).items()?;
        if t2556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2556[0].clone();
        v_v = t2556[1].clone();
        if ({ let t2555 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t2555.truthy() { t2555 } else { { let t2554 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t2554.truthy() { t2554 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__furness(mut v_base: Py, mut v_O: Py, mut v_D: Py, mut v_tol: Py, mut v_it: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_err: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t777 = (Py::tuple(vec![py::len(&v_O.clone())?, py::len(&v_D.clone())?])).items()?;
    if t777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t777[0].clone();
    v_m = t777[1].clone();
    v_T = { let mut out = vec![]; for t778 in py::iter(&(v_base.clone()))? { let mut v_row: Py; 
v_row = t778;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    for t779 in py::range_iter(&[(v_it.clone()).i()?]) {
        v__ = t779;
        for t780 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t780;
            v_s = py::fsum(&py::getitem(&v_T.clone(), &v_i.clone())?)?;
            let _ = f__need({ let t781 = v_s.clone(); Py::Bool({ let t782 = Py::Int(0i64); if py::gt(&t781, &t782)? { true } else { false } }) }, Py::str("every origin needs a positive entry"))?;
            v_f = py::truediv(&py::getitem(&v_O.clone(), &v_i.clone())?, &v_s.clone())?;
            py::setitem(&v_T.clone(), &v_i.clone(), { let mut out = vec![]; for t783 in py::iter(&(py::getitem(&v_T.clone(), &v_i.clone())?))? { let mut v_x: Py; 
v_x = t783;
out.push(py::mul(&v_x.clone(), &v_f.clone())?); } Py::list(out) })?;
        }
        v_err = Py::Float(0.0);
        for t784 in py::range_iter(&[(v_m.clone()).i()?]) {
            v_j = t784;
            v_s = py::fsum(&{ let mut out = vec![]; for t785 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t785;
out.push(py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) })?;
            let _ = f__need({ let t786 = v_s.clone(); Py::Bool({ let t787 = Py::Int(0i64); if py::gt(&t786, &t787)? { true } else { false } }) }, Py::str("every destination needs a positive entry"))?;
            v_f = py::truediv(&py::getitem(&v_D.clone(), &v_j.clone())?, &v_s.clone())?;
            for t788 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_i = t788;
                let t789 = py::getitem(&v_T.clone(), &v_i.clone())?; let t790 = v_j.clone();
                py::setitem(&t789, &t790, py::mul(&py::getitem(&t789, &t790)?, &v_f.clone())?)?;
            }
        }
        for t791 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t791;
            v_err = py::minmax(&[v_err.clone(), py::truediv(&py::abs(&py::sub(&py::fsum(&py::getitem(&v_T.clone(), &v_i.clone())?)?, &py::getitem(&v_O.clone(), &v_i.clone())?)?)?, &py::getitem(&v_O.clone(), &v_i.clone())?)?], true)?;
        }
        if ({ let t792 = v_err.clone(); Py::Bool({ let t793 = v_tol.clone(); if py::lt(&t792, &t793)? { true } else { false } }) }).truthy() {
            return Ok(v_T.clone());
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("the balancing did not converge"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__graph(mut v_roads: Py, mut v_speeds: Py, mut v_directed: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t2252 = Py::Bool(matches!(v_roads.clone(), Py::List(_))); if !t2252.truthy() { t2252 } else { { let t2251 = v_roads.clone(); if !t2251.truthy() { t2251 } else { Py::Bool(({ let mut out = vec![]; for t2248 in py::iter(&(v_roads.clone()))? { let mut v_e: Py; 
v_e = t2248;
out.push({ let t2249 = py::len(&v_e.clone())?; Py::Bool({ let t2250 = Py::Int(3i64); if py::eq(&t2249, &t2250) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } }, Py::str("give every road with its length: 'A-B 2.5' (km)"))?;
    v_sp = f__vec(Py::str("speeds"), v_speeds.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t2268 = py::len(&v_sp.clone())?; Py::Bool({ let t2269 = py::len(&v_roads.clone())?; if py::eq(&t2268, &t2269) { true } else { false } }) }, Py::str("give one speed for every road, in the same order"))?;
    v_adj = Py::dict(vec![]);
    for t2270 in py::iter(&(py::zip(&[v_roads.clone(), v_sp.clone()])?))? {
        let t2282 = (t2270).items()?;
        if t2282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2283 = (t2282[0].clone()).items()?;
        if t2283.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2283[0].clone();
        v_v = t2283[1].clone();
        v_L = t2283[2].clone();
        v_s = t2282[1].clone();
        let _ = f__need({ let t2271 = v_L.clone(); Py::Bool({ let t2272 = Py::Int(0i64); if py::gt(&t2271, &t2272)? { true } else { false } }) }, Py::str("road lengths must be positive"))?;
        let _ = f__need({ let t2273 = v_u.clone(); Py::Bool({ let t2274 = v_v.clone(); if py::ne(&t2273, &t2274) { true } else { false } }) }, Py::str("a road must join two different places"))?;
        v_t = py::truediv(&py::mul(&Py::Float(60.0), &v_L.clone())?, &v_s.clone())?;
        for t2275 in py::iter(&(py::add(&Py::tuple(vec![Py::tuple(vec![v_u.clone(), v_v.clone()])]), &(if (v_directed.clone()).truthy() { Py::tuple(vec![]) } else { Py::tuple(vec![Py::tuple(vec![v_v.clone(), v_u.clone()])]) }))?))? {
            let t2281 = (t2275).items()?;
            if t2281.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2281[0].clone();
            v_b = t2281[1].clone();
            let _ = py::method(&v_adj.clone(), "setdefault", &[v_a.clone(), Py::dict(vec![])])?;
            let _ = py::method(&v_adj.clone(), "setdefault", &[v_b.clone(), Py::dict(vec![])])?;
            if ({ let t2280 = { let t2276 = v_b.clone(); Py::Bool({ let t2277 = py::getitem(&v_adj.clone(), &v_a.clone())?; if !py::contains(&t2277, &t2276)? { true } else { false } }) }; if t2280.truthy() { t2280 } else { { let t2278 = v_t.clone(); Py::Bool({ let t2279 = py::getitem(&py::getitem(&py::getitem(&v_adj.clone(), &v_a.clone())?, &v_b.clone())?, &Py::Int(0i64))?; if py::lt(&t2278, &t2279)? { true } else { false } }) } } }).truthy() {
                py::setitem(&py::getitem(&v_adj.clone(), &v_a.clone())?, &v_b.clone(), Py::tuple(vec![v_t.clone(), v_L.clone()]))?;
            }
        }
    }
    return Ok(v_adj.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kmh(mut v_v: Py) -> R {
    return Ok(f__r(py::mul(&v_v.clone(), &Py::Float(3.6))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__linfit(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let t2453 = (Py::tuple(vec![py::truediv(&py::sum(&v_x.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?])).items()?;
    if t2453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2453[0].clone();
    v_my = t2453[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2454 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t2454;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_syy = py::sum(&{ let mut out = vec![]; for t2455 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t2455;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t2456 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2457 = (t2456).items()?;
if t2457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2457[0].clone();
v_b = t2457[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2458 = v_sxx.clone(); Py::Bool({ let t2459 = Py::Int(0i64); if py::gt(&t2458, &t2459)? { true } else { false } }) }, Py::str("the densities must not all be equal"))?;
    let _ = f__need({ let t2460 = v_syy.clone(); Py::Bool({ let t2461 = Py::Int(0i64); if py::gt(&t2460, &t2461)? { true } else { false } }) }, Py::str("the speeds must not all be equal"))?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    return Ok(Py::tuple(vec![py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?, v_b.clone(), py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lstsq_qr(mut v_A: Py, mut v_y: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_norm: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vv: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2343 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t2343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2343[0].clone();
    v_n = t2343[1].clone();
    v_R = { let mut out = vec![]; for t2344 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t2344;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    v_b = py::slice(&v_y.clone(), None, None, None)?;
    for t2345 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2345;
        v_norm = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2346 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2346;
out.push(py::pow(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        let _ = f__need({ let t2347 = v_norm.clone(); Py::Bool({ let t2348 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2349 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2349;
out.push(py::abs(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_k.clone())?)?); } Py::list(out) }], true)?], true)?)?; if py::gt(&t2347, &t2348)? { true } else { false } }) }, Py::str("the predictors are collinear"))?;
        v_alpha = (if ({ let t2350 = py::getitem(&py::getitem(&v_R.clone(), &v_k.clone())?, &v_k.clone())?; Py::Bool({ let t2351 = Py::Int(0i64); if py::ge(&t2350, &t2351)? { true } else { false } }) }).truthy() { py::neg(&v_norm.clone())? } else { v_norm.clone() });
        v_v = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?;
        py::setitem(&v_v.clone(), &v_k.clone(), py::sub(&py::getitem(&py::getitem(&v_R.clone(), &v_k.clone())?, &v_k.clone())?, &v_alpha.clone())?)?;
        for t2352 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_m.clone()).i()?]) {
            v_i = t2352;
            py::setitem(&v_v.clone(), &v_i.clone(), py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_k.clone())?)?;
        }
        v_vv = py::fsum(&{ let mut out = vec![]; for t2353 in py::iter(&(py::slice(&v_v.clone(), Some((v_k.clone()).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t2353;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) })?;
        if ({ let t2354 = v_vv.clone(); Py::Bool({ let t2355 = Py::Int(0i64); if py::eq(&t2354, &t2355) { true } else { false } }) }).truthy() {
            continue;
        }
        for t2356 in py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?]) {
            v_j = t2356;
            v_s = py::truediv(&py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t2357 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2357;
out.push(py::mul(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) })?)?, &v_vv.clone())?;
            for t2358 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) {
                v_i = t2358;
                let t2359 = py::getitem(&v_R.clone(), &v_i.clone())?; let t2360 = v_j.clone();
                py::setitem(&t2359, &t2360, py::sub(&py::getitem(&t2359, &t2360)?, &py::mul(&v_s.clone(), &py::getitem(&v_v.clone(), &v_i.clone())?)?)?)?;
            }
        }
        v_s = py::truediv(&py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t2361 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2361;
out.push(py::mul(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&v_b.clone(), &v_i.clone())?)?); } Py::list(out) })?)?, &v_vv.clone())?;
        for t2362 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) {
            v_i = t2362;
            let t2363 = v_b.clone(); let t2364 = v_i.clone();
            py::setitem(&t2363, &t2364, py::sub(&py::getitem(&t2363, &t2364)?, &py::mul(&v_s.clone(), &py::getitem(&v_v.clone(), &v_i.clone())?)?)?)?;
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t2365 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_i = t2365;
        let _ = f__need({ let t2366 = py::abs(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_i.clone())?)?; Py::Bool({ let t2367 = Py::Int(0i64); if py::gt(&t2366, &t2367)? { true } else { false } }) }, Py::str("the predictors are collinear"))?;
        py::setitem(&v_x.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t2368 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2368;
out.push(py::mul(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mat(mut v_name: Py, mut v_m: Py, mut v_rows: Py, mut v_cols: Py, mut v_nonneg: Py, mut v_positive: Py) -> R {
    let mut v_out: Py = Py::None;
    let _ = f__need({ let t2312 = Py::Bool(matches!(v_m.clone(), Py::List(_) | Py::Tuple(_))); if !t2312.truthy() { t2312 } else { { let t2310 = py::len(&v_m.clone())?; Py::Bool({ let t2311 = Py::Int(0i64); if py::gt(&t2310, &t2311)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix".to_string()].concat()))?;
    v_out = { let mut out = vec![]; for t2313 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t2313;
out.push(f__vec(v_name.clone(), v_r.clone(), Py::None, v_positive.clone(), v_nonneg.clone())?); } Py::list(out) };
    let _ = f__need({ let t2329 = py::len(&py::set(&{ let mut out = vec![]; for t2331 in py::iter(&(v_out.clone()))? { let mut v_r: Py; 
v_r = t2331;
out.push(py::len(&v_r.clone())?); } Py::list(out) })?)?; Py::Bool({ let t2330 = Py::Int(1i64); if py::eq(&t2329, &t2330) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be rectangular".to_string()].concat()))?;
    if ({ let t2332 = v_rows.clone(); Py::Bool({ let t2333 = Py::None; if !py::is(&t2332, &t2333) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2334 = py::len(&v_out.clone())?; Py::Bool({ let t2335 = v_rows.clone(); if py::eq(&t2334, &t2335) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_rows.clone(), "")?, " rows".to_string()].concat()))?;
    }
    if ({ let t2336 = v_cols.clone(); Py::Bool({ let t2337 = Py::None; if !py::is(&t2336, &t2337) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2338 = py::len(&py::getitem(&v_out.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2339 = v_cols.clone(); if py::eq(&t2338, &t2339) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_cols.clone(), "")?, " columns".to_string()].concat()))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_cond: Py, mut v_message: Py) -> R {
    if (py::not(&v_cond.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_message.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nests(mut v_nest: Py, mut v_lam: Py, mut v_n: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lm: Py = Py::None;
    v_k = f__vec(Py::str("nest"), v_nest.clone(), v_n.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1358 in py::iter(&(v_k.clone()))? { let mut v_v: Py; 
v_v = t1358;
out.push({ let t1363 = { let t1359 = v_v.clone(); Py::Bool({ let t1360 = py::int(&v_v.clone())?; if py::eq(&t1359, &t1360) { true } else { false } }) }; if !t1363.truthy() { t1363 } else { { let t1361 = v_v.clone(); Py::Bool({ let t1362 = Py::Int(1i64); if py::ge(&t1361, &t1362)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("nest numbers must be whole numbers 1, 2, ..."))?;
    v_k = { let mut out = vec![]; for t1364 in py::iter(&(v_k.clone()))? { let mut v_v: Py; 
v_v = t1364;
out.push(py::int(&v_v.clone())?); } Py::list(out) };
    v_K = py::minmax(&[v_k.clone()], true)?;
    v_lm = f__vec(Py::str("lam"), v_lam.clone(), v_K.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t1365 = py::set(&v_k.clone())?; Py::Bool({ let t1366 = py::set(&Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?; if py::eq(&t1365, &t1366) { true } else { false } }) }, Py::str("every nest 1..K must hold at least one alternative"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1367 in py::iter(&(v_lm.clone()))? { let mut v_x: Py; 
v_x = t1367;
out.push({ let t1368 = Py::Int(0i64); Py::Bool({ let t1369 = v_x.clone(); if py::lt(&t1368, &t1369)? { { let t1370 = Py::Int(1i64); if py::le(&t1369, &t1370)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("nest parameters must lie in (0, 1]"))?;
    return Ok(Py::tuple(vec![v_k.clone(), v_lm.clone(), v_K.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__finite(v_kw.clone())?;
    for t2563 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t2566 = (t2563).items()?;
        if t2566.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2566[0].clone();
        v_v = t2566[1].clone();
        if ({ let t2564 = v_v.clone(); Py::Bool({ let t2565 = Py::Int(0i64); if py::lt(&t2564, &t2565)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must not be negative".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pct(mut v_sorted_v: Py, mut v_p: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_pos: Py = Py::None;
    v_pos = py::truediv(&py::mul(&py::sub(&py::len(&v_sorted_v.clone())?, &Py::Int(1i64))?, &v_p.clone())?, &Py::Int(100i64))?;
    v_i = py::int(&py::math1("floor", &v_pos.clone())?)?;
    v_f = py::sub(&v_pos.clone(), &v_i.clone())?;
    if ({ let t1921 = py::add(&v_i.clone(), &Py::Int(1i64))?; Py::Bool({ let t1922 = py::len(&v_sorted_v.clone())?; if py::ge(&t1921, &t1922)? { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_sorted_v.clone(), &py::neg(&Py::Int(1i64))?)?);
    }
    return Ok(py::add(&py::getitem(&v_sorted_v.clone(), &v_i.clone())?, &py::mul(&v_f.clone(), &py::sub(&py::getitem(&v_sorted_v.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_sorted_v.clone(), &v_i.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pctl(mut v_xs: Py, mut v_p: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::sorted(&v_xs.clone(), (Py::Bool(false)).truthy())?;
    v_h = py::truediv(&py::mul(&py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?, &v_p.clone())?, &Py::Float(100.0))?;
    v_lo = py::math1("floor", &v_h.clone())?;
    v_hi = py::minmax(&[py::add(&v_lo.clone(), &Py::Int(1i64))?, py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?], false)?;
    return Ok(py::add(&py::getitem(&v_s.clone(), &v_lo.clone())?, &py::mul(&py::sub(&v_h.clone(), &v_lo.clone())?, &py::sub(&py::getitem(&v_s.clone(), &v_hi.clone())?, &py::getitem(&v_s.clone(), &v_lo.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_terms(mut v_lam: Py, mut v_n: Py) -> R {
    let mut v_gt: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_le: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_p_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_lp = { let v_lam = v_lam.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::mul(&v_i.clone(), &py::math1("log", &v_lam.clone())?)?, &v_lam.clone())?, &py::math1("lgamma", &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_p_n = py::math1("exp", &py::call(&v_lp.clone(), &[v_n.clone()])?)?;
    v_le = py::sum(&{ let mut out = vec![]; for t1475 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t1475;
out.push(py::math1("exp", &py::call(&v_lp.clone(), &[v_i.clone()])?)?); } Py::list(out) }, None)?;
    let t1476 = (Py::tuple(vec![Py::Float(0.0), py::add(&v_n.clone(), &Py::Int(1i64))?])).items()?;
    if t1476.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gt = t1476[0].clone();
    v_i = t1476[1].clone();
    while (Py::Bool(true)).truthy() {
        v_t = py::math1("exp", &py::call(&v_lp.clone(), &[v_i.clone()])?)?;
        v_gt = py::iadd(&v_gt, &v_t.clone())?;
        if ({ let t1481 = { let t1477 = v_i.clone(); Py::Bool({ let t1478 = v_lam.clone(); if py::gt(&t1477, &t1478)? { true } else { false } }) }; if !t1481.truthy() { t1481 } else { { let t1479 = v_t.clone(); Py::Bool({ let t1480 = py::mul(&Py::Float(1e-18), &py::minmax(&[v_gt.clone(), Py::Float(1e-300)], true)?)?; if py::lt(&t1479, &t1480)? { true } else { false } }) } } }).truthy() {
            break;
        }
        v_i = py::iadd(&v_i, &Py::Int(1i64))?;
        if ({ let t1482 = v_i.clone(); Py::Bool({ let t1483 = py::add(&v_n.clone(), &Py::Int(100000i64))?; if py::gt(&t1482, &t1483)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(Py::tuple(vec![v_p_n.clone(), v_le.clone(), v_gt.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__finite(v_kw.clone())?;
    for t2558 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t2561 = (t2558).items()?;
        if t2561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2561[0].clone();
        v_v = t2561[1].clone();
        if ({ let t2559 = v_v.clone(); Py::Bool({ let t2560 = Py::Int(0i64); if py::le(&t2559, &t2560)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be positive".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__power_balance(mut v_m: Py, mut v_P: Py, mut v_C_D: Py, mut v_A: Py, mut v_rho: Py, mut v_f_r: Py, mut v_Gr: Py, mut v_eta: Py) -> R {
    let mut v_F0: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_hi: Py = Py::None;
    v_K = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_C_D.clone())?, &v_A.clone())?, &Py::Int(2i64))?;
    v_F0 = f__resist_const(v_m.clone(), v_f_r.clone(), v_Gr.clone(), Py::Float(0.0), Py::Float(1.0))?;
    v_h = { let v_A = v_A.clone(); let v_C_D = v_C_D.clone(); let v_F0 = v_F0.clone(); let v_Gr = v_Gr.clone(); let v_K = v_K.clone(); let v_P = v_P.clone(); let v_eta = v_eta.clone(); let v_f_r = v_f_r.clone(); let v_m = v_m.clone(); let v_rho = v_rho.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::mul(&v_eta.clone(), &v_P.clone())?, &py::mul(&v_F0.clone(), &v_v.clone())?)?, &py::mul(&v_K.clone(), &py::pow(&v_v.clone(), &Py::Int(3i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_hi = Py::Float(1.0);
    while ({ let t44 = py::call(&v_h.clone(), &[v_hi.clone()])?; Py::Bool({ let t45 = Py::Int(0i64); if py::gt(&t44, &t45)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Int(2i64))?;
    }
    return Ok(Py::tuple(vec![v_K.clone(), v_F0.clone(), f__bisect_root(v_h.clone(), Py::Float(0.0), v_hi.clone(), Py::Int(400i64))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t2571 = Py::Bool(matches!(v_x.clone(), Py::Bool(_) | Py::Str(_))); if t2571.truthy() { t2571 } else { { let t2569 = v_x.clone(); Py::Bool({ let t2570 = Py::None; if py::is(&t2569, &t2570) { true } else { false } }) } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t2572 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2572;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    v_x = py::float(&v_x.clone())?;
    if ({ let t2575 = { let t2573 = v_x.clone(); Py::Bool({ let t2574 = Py::Float(0.0); if py::eq(&t2573, &t2574) { true } else { false } }) }; if t2575.truthy() { t2575 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t2576 = v_x.clone(); Py::Bool({ let t2577 = Py::Float(0.0); if py::eq(&t2576, &t2577) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__resist(mut v_m: Py, mut v_v: Py, mut v_C_D: Py, mut v_A: Py, mut v_rho: Py, mut v_f_r: Py, mut v_G_: Py) -> R {
    let mut v_Fa: Py = Py::None;
    let mut v_Fg: Py = Py::None;
    let mut v_Fr: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_th = py::math1("atan", &v_G_.clone())?;
    v_Fr = py::mul(&py::mul(&py::mul(&v_f_r.clone(), &v_m.clone())?, &c_G()?)?, &py::math1("cos", &v_th.clone())?)?;
    v_Fa = py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_C_D.clone())?, &v_A.clone())?, &v_v.clone())?, &v_v.clone())?;
    v_Fg = py::mul(&py::mul(&v_m.clone(), &c_G()?)?, &py::math1("sin", &v_th.clone())?)?;
    return Ok(Py::tuple(vec![v_Fr.clone(), v_Fa.clone(), v_Fg.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__resist_const(mut v_m: Py, mut v_f_r: Py, mut v_Gr: Py, mut v_mu: Py, mut v_eta: Py) -> R {
    let mut v_th: Py = Py::None;
    v_th = py::math1("atan", &v_Gr.clone())?;
    return Ok(py::mul(&py::mul(&v_m.clone(), &Py::Float(9.81))?, &py::add(&py::mul(&py::add(&py::mul(&v_eta.clone(), &v_mu.clone())?, &v_f_r.clone())?, &py::math1("cos", &v_th.clone())?)?, &py::math1("sin", &v_th.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_x: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t2058 = py::abs(&v_x.clone())?; Py::Bool({ let t2059 = py::mul(&Py::Float(1e-12), &py::abs(&v_scale.clone())?)?; if py::le(&t2058, &t2059)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__softmax(mut v_V: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_mx = py::minmax(&[v_V.clone()], true)?;
    v_e = { let mut out = vec![]; for t1377 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t1377;
out.push(py::math1("exp", &py::sub(&v_v.clone(), &v_mx.clone())?)?); } Py::list(out) };
    v_s = py::fsum(&v_e.clone())?;
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t1378 in py::iter(&(v_e.clone()))? { let mut v_x: Py; 
v_x = t1378;
out.push(py::truediv(&v_x.clone(), &v_s.clone())?); } Py::list(out) }, py::add(&v_mx.clone(), &py::math1("log", &v_s.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__speeds(mut v_v: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = f__vec(Py::str("speeds"), v_v.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t2045 = py::len(&v_s.clone())?; Py::Bool({ let t2046 = Py::Int(2i64); if py::ge(&t2045, &t2046)? { true } else { false } }) }, Py::str("give at least two speeds"))?;
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__truthy(mut v_x: Py) -> R {
    return Ok((if (Py::Bool(matches!(v_x.clone(), Py::Bool(_)))).truthy() { v_x.clone() } else { { let t2246 = py::method(&py::method(&Py::str(&py::to_str(&v_x.clone())), "strip", &[])?, "lower", &[])?; Py::Bool({ let t2247 = Py::tuple(vec![Py::str("true"), Py::str("yes"), Py::str("1")]); if py::contains(&t2247, &t2246)? { true } else { false } }) } }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_name: Py, mut v_v: Py, mut v_n: Py, mut v_positive: Py, mut v_nonneg: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t2435 = Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_))); if !t2435.truthy() { t2435 } else { { let t2433 = py::len(&v_v.clone())?; Py::Bool({ let t2434 = Py::Int(0i64); if py::gt(&t2433, &t2434)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of numbers".to_string()].concat()))?;
    for t2436 in py::iter(&(v_v.clone()))? {
        v_x = t2436;
        let _ = f__need({ let t2438 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2438.truthy() { t2438 } else { { let t2437 = py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))); if !t2437.truthy() { t2437 } else { py::math1("isfinite", &v_x.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must hold finite numbers".to_string()].concat()))?;
        if (v_positive.clone()).truthy() {
            let _ = f__need({ let t2439 = v_x.clone(); Py::Bool({ let t2440 = Py::Int(0i64); if py::gt(&t2439, &t2440)? { true } else { false } }) }, Py::str(&["every entry of ".to_string(), py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
        }
        if (v_nonneg.clone()).truthy() {
            let _ = f__need({ let t2441 = v_x.clone(); Py::Bool({ let t2442 = Py::Int(0i64); if py::ge(&t2441, &t2442)? { true } else { false } }) }, Py::str(&["every entry of ".to_string(), py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
        }
    }
    if ({ let t2443 = v_n.clone(); Py::Bool({ let t2444 = Py::None; if !py::is(&t2443, &t2444) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2445 = py::len(&v_v.clone())?; Py::Bool({ let t2446 = v_n.clone(); if py::eq(&t2445, &t2446) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_n.clone(), "")?, " entries".to_string()].concat()))?;
    }
    return Ok({ let mut out = vec![]; for t2447 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2447;
out.push(py::float(&v_x.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vph(mut v_q: Py) -> R {
    return Ok(f__r(py::mul(&v_q.clone(), &Py::Int(3600i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vpkm(mut v_k: Py) -> R {
    return Ok(f__r(py::mul(&v_k.clone(), &Py::Int(1000i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wave_check(mut v_q1: Py, mut v_k1: Py, mut v_q2: Py, mut v_k2: Py) -> R {
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("k1"), v_k1.clone()), (Py::str("k2"), v_k2.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("q1"), v_q1.clone()), (Py::str("q2"), v_q2.clone())]))?;
    let t1656 = (Py::tuple(vec![py::truediv(&v_q1.clone(), &v_k1.clone())?, py::truediv(&v_q2.clone(), &v_k2.clone())?])).items()?;
    if t1656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t1656[0].clone();
    v_v2 = t1656[1].clone();
    let _ = f__need({ let t1661 = { let t1657 = v_k1.clone(); Py::Bool({ let t1658 = v_k2.clone(); if py::ne(&t1657, &t1658) { true } else { false } }) }; if !t1661.truthy() { t1661 } else { { let t1659 = v_v1.clone(); Py::Bool({ let t1660 = v_v2.clone(); if py::ne(&t1659, &t1660) { true } else { false } }) } } }, Py::str("the two states must differ in density and speed"))?;
    let _ = f__need({ let t1662 = py::mul(&py::sub(&v_v1.clone(), &v_v2.clone())?, &py::sub(&v_k2.clone(), &v_k1.clone())?)?; Py::Bool({ let t1663 = Py::Int(0i64); if py::gt(&t1662, &t1663)? { true } else { false } }) }, Py::str("the states are inconsistent: a denser state must be slower"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_accel_distance(mut v_v1: Py, mut v_v2: Py, mut v_a: Py) -> R {
    let _ = f__nonneg(Py::dict(vec![(Py::str("v1"), v_v1.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("v2"), v_v2.clone()), (Py::str("a"), v_a.clone())]))?;
    let _ = f__need({ let t15 = v_v2.clone(); Py::Bool({ let t16 = v_v1.clone(); if py::gt(&t15, &t16)? { true } else { false } }) }, Py::str("the final speed must exceed the initial speed"))?;
    return Ok(Py::dict(vec![(Py::str("d"), f__r(py::truediv(&py::sub(&py::mul(&v_v2.clone(), &v_v2.clone())?, &py::mul(&v_v1.clone(), &v_v1.clone())?)?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?)?), (Py::str("t"), f__r(py::truediv(&py::sub(&v_v2.clone(), &v_v1.clone())?, &v_a.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_aon(mut v_network: Py, mut v_od: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_flows: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vht: Py = Py::None;
    let _ = f__need({ let t94 = Py::Bool(matches!(v_network.clone(), Py::List(_))); if !t94.truthy() { t94 } else { { let t93 = v_network.clone(); if !t93.truthy() { t93 } else { Py::Bool(({ let mut out = vec![]; for t90 in py::iter(&(v_network.clone()))? { let mut v_e: Py; 
v_e = t90;
out.push({ let t91 = py::len(&v_e.clone())?; Py::Bool({ let t92 = Py::Int(3i64); if py::eq(&t91, &t92) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } }, Py::str("give every link with its travel time: 'A-B 4' (min)"))?;
    let _ = f__need({ let t99 = Py::Bool(matches!(v_od.clone(), Py::List(_))); if !t99.truthy() { t99 } else { { let t98 = v_od.clone(); if !t98.truthy() { t98 } else { Py::Bool(({ let mut out = vec![]; for t95 in py::iter(&(v_od.clone()))? { let mut v_e: Py; 
v_e = t95;
out.push({ let t96 = py::len(&v_e.clone())?; Py::Bool({ let t97 = Py::Int(3i64); if py::eq(&t96, &t97) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } } } }, Py::str("give every trip pair with its demand: 'A-D 500'"))?;
    let t100 = (Py::tuple(vec![Py::dict(vec![]), py::set(&Py::list(vec![]))?])).items()?;
    if t100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_adj = t100[0].clone();
    v_seen = t100[1].clone();
    for t101 in py::iter(&(v_network.clone()))? {
        let t109 = (t101).items()?;
        if t109.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t109[0].clone();
        v_v = t109[1].clone();
        v_t = t109[2].clone();
        let _ = f__need({ let t102 = v_t.clone(); Py::Bool({ let t103 = Py::Int(0i64); if py::gt(&t102, &t103)? { true } else { false } }) }, Py::str("travel times must be positive"))?;
        let _ = f__need({ let t108 = { let t104 = Py::tuple(vec![v_u.clone(), v_v.clone()]); Py::Bool({ let t105 = v_seen.clone(); if !py::contains(&t105, &t104)? { true } else { false } }) }; if !t108.truthy() { t108 } else { { let t106 = v_u.clone(); Py::Bool({ let t107 = v_v.clone(); if py::ne(&t106, &t107) { true } else { false } }) } } }, Py::str("each one-way link may be listed once"))?;
        let _ = py::method(&v_seen.clone(), "add", &[Py::tuple(vec![v_u.clone(), v_v.clone()])])?;
        py::setitem(&py::method(&v_adj.clone(), "setdefault", &[v_u.clone(), Py::dict(vec![])])?, &v_v.clone(), Py::tuple(vec![v_t.clone(), Py::Float(0.0)]))?;
        let _ = py::method(&v_adj.clone(), "setdefault", &[v_v.clone(), Py::dict(vec![])])?;
    }
    v_flows = { let mut out = vec![]; for t110 in py::iter(&(v_network.clone()))? { let mut v__t: Py; let mut v_u: Py; let mut v_v: Py; 
let t111 = (t110).items()?;
if t111.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t111[0].clone();
v_v = t111[1].clone();
v__t = t111[2].clone();
out.push((Py::tuple(vec![v_u.clone(), v_v.clone()]), Py::Float(0.0))); } Py::dict(out) };
    for t112 in py::iter(&(v_od.clone()))? {
        let t140 = (t112).items()?;
        if t140.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_o = t140[0].clone();
        v_d = t140[1].clone();
        v_q = t140[2].clone();
        let _ = f__need({ let t113 = v_q.clone(); Py::Bool({ let t114 = Py::Int(0i64); if py::ge(&t113, &t114)? { true } else { false } }) }, Py::str("demands must not be negative"))?;
        let _ = f__need({ let t122 = { let t115 = v_o.clone(); Py::Bool({ let t116 = v_adj.clone(); if py::contains(&t116, &t115)? { true } else { false } }) }; if !t122.truthy() { t122 } else { { let t121 = { let t117 = v_d.clone(); Py::Bool({ let t118 = v_adj.clone(); if py::contains(&t118, &t117)? { true } else { false } }) }; if !t121.truthy() { t121 } else { { let t119 = v_o.clone(); Py::Bool({ let t120 = v_d.clone(); if py::ne(&t119, &t120) { true } else { false } }) } } } } }, Py::str("every trip pair must join two different places of the network"))?;
        v_best = f__dijkstra(v_adj.clone(), v_o.clone())?;
        let _ = f__need({ let t134 = v_d.clone(); Py::Bool({ let t135 = v_best.clone(); if py::contains(&t135, &t134)? { true } else { false } }) }, Py::str(&[py::format(&v_d.clone(), "")?, " cannot be reached from ".to_string(), py::format(&v_o.clone(), "")?].concat()))?;
        v_path = py::getitem(&py::getitem(&v_best.clone(), &v_d.clone())?, &Py::Int(2i64))?;
        for t136 in py::iter(&(py::zip(&[v_path.clone(), py::slice(&v_path.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
            let t139 = (t136).items()?;
            if t139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t139[0].clone();
            v_b = t139[1].clone();
            let t137 = v_flows.clone(); let t138 = Py::tuple(vec![v_a.clone(), v_b.clone()]);
            py::setitem(&t137, &t138, py::iadd(&py::getitem(&t137, &t138)?, &v_q.clone())?)?;
        }
    }
    v_f = { let mut out = vec![]; for t141 in py::iter(&(v_network.clone()))? { let mut v__t: Py; let mut v_u: Py; let mut v_v: Py; 
let t142 = (t141).items()?;
if t142.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t142[0].clone();
v_v = t142[1].clone();
v__t = t142[2].clone();
out.push(py::getitem(&v_flows.clone(), &Py::tuple(vec![v_u.clone(), v_v.clone()]))?); } Py::list(out) };
    v_vht = py::truediv(&py::fsum(&{ let mut out = vec![]; for t143 in py::iter(&(py::zip(&[v_f.clone(), v_network.clone()])?))? { let mut v__u: Py; let mut v__v: Py; let mut v_t: Py; let mut v_x: Py; 
let t144 = (t143).items()?;
if t144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t144[0].clone();
let t145 = (t144[1].clone()).items()?;
if t145.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__u = t145[0].clone();
v__v = t145[1].clone();
v_t = t145[2].clone();
out.push(py::mul(&v_x.clone(), &v_t.clone())?); } Py::list(out) })?, &Py::Int(60i64))?;
    return Ok(Py::dict(vec![(Py::str("flows"), f__r(v_f.clone())?), (Py::str("VHT"), f__r(v_vht.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bottleneck(mut v_lam: Py, mut v_mu: Py, mut v_mu_r: Py, mut v_t_r: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_T: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("mu"), v_mu.clone()), (Py::str("mu_r"), v_mu_r.clone()), (Py::str("t_r"), v_t_r.clone())]))?;
    let _ = f__need({ let t164 = v_mu_r.clone(); Py::Bool({ let t165 = v_lam.clone(); if py::lt(&t164, &t165)? { { let t166 = v_mu.clone(); if py::lt(&t165, &t166)? { true } else { false } } } else { false } }) }, Py::str("need reduced capacity < arrival flow < full capacity"))?;
    v_Q = py::mul(&py::sub(&v_lam.clone(), &v_mu_r.clone())?, &v_t_r.clone())?;
    v_T = py::add(&v_t_r.clone(), &py::truediv(&v_Q.clone(), &py::sub(&v_mu.clone(), &v_lam.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Q_max"), f__r(v_Q.clone())?), (Py::str("T"), f__r(v_T.clone())?), (Py::str("T_min"), f__r(py::truediv(&v_T.clone(), &Py::Int(60i64))?)?), (Py::str("D_total"), f__r(py::truediv(&py::truediv(&py::mul(&v_Q.clone(), &v_T.clone())?, &Py::Int(2i64))?, &Py::Int(3600i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_brake_resist(mut v_m: Py, mut v_v1: Py, mut v_mu: Py, mut v_C_D: Py, mut v_A: Py, mut v_rho: Py, mut v_f_r: Py, mut v_G: Py, mut v_eta: Py, mut v_gamma: Py, mut v_v2: Py) -> R {
    let mut v_F0: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("v1"), v_v1.clone()), (Py::str("mu"), v_mu.clone()), (Py::str("C_D"), v_C_D.clone()), (Py::str("A"), v_A.clone()), (Py::str("rho"), v_rho.clone()), (Py::str("eta"), v_eta.clone()), (Py::str("gamma"), v_gamma.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("f_r"), v_f_r.clone()), (Py::str("v2"), v_v2.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t230 = v_v2.clone(); Py::Bool({ let t231 = v_v1.clone(); if py::lt(&t230, &t231)? { true } else { false } }) }, Py::str("the final speed must be below the initial speed"))?;
    let _ = f__need({ let t232 = v_eta.clone(); Py::Bool({ let t233 = Py::Int(1i64); if py::le(&t232, &t233)? { true } else { false } }) }, Py::str("the braking efficiency must not exceed 1"))?;
    v_F0 = f__resist_const(v_m.clone(), v_f_r.clone(), v_G.clone(), v_mu.clone(), v_eta.clone())?;
    let _ = f__need({ let t234 = v_F0.clone(); Py::Bool({ let t235 = Py::Int(0i64); if py::gt(&t234, &t235)? { true } else { false } }) }, Py::str("the constant retarding force must be positive (the grade is too steep)"))?;
    let t236 = (f__decel_drag(v_m.clone(), v_F0.clone(), py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_C_D.clone())?, &v_A.clone())?, &Py::Int(2i64))?, v_gamma.clone(), v_v1.clone(), v_v2.clone())?).items()?;
    if t236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t236[0].clone();
    v_t = t236[1].clone();
    return Ok(Py::dict(vec![(Py::str("S"), f__r(v_S.clone())?), (Py::str("t"), f__r(v_t.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_brake_transfer(mut v_m: Py, mut v_a: Py, mut v_h: Py, mut v_L: Py, mut v_l_f: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_Wf: Py = Py::None;
    let mut v_Wr: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("a"), v_a.clone()), (Py::str("h"), v_h.clone())]))?;
    let _ = f__check_axles(v_L.clone(), v_l_f.clone())?;
    v_W = py::mul(&v_m.clone(), &c_G_()?)?;
    v_Wf = py::truediv(&py::add(&py::mul(&v_W.clone(), &py::sub(&v_L.clone(), &v_l_f.clone())?)?, &py::mul(&py::mul(&v_m.clone(), &v_a.clone())?, &v_h.clone())?)?, &v_L.clone())?;
    v_Wr = py::sub(&v_W.clone(), &v_Wf.clone())?;
    let _ = f__need({ let t205 = v_Wr.clone(); Py::Bool({ let t206 = Py::Int(0i64); if py::gt(&t205, &t206)? { true } else { false } }) }, Py::str("the rear axle would lift off at this deceleration"))?;
    return Ok(Py::dict(vec![(Py::str("W_f"), f__r(v_Wf.clone())?), (Py::str("W_r"), f__r(v_Wr.clone())?), (Py::str("dW"), f__r(py::truediv(&py::mul(&py::mul(&v_m.clone(), &v_a.clone())?, &v_h.clone())?, &v_L.clone())?)?), (Py::str("front_share"), f__r(py::truediv(&v_Wf.clone(), &v_W.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_braking_distance(mut v_v: Py, mut v_a: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("a"), v_a.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("d"), f__r(py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?)?), (Py::str("t"), f__r(py::truediv(&v_v.clone(), &v_a.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_clothoid(mut v_R: Py, mut v_L_s: Py, mut v_s: Py) -> R {
    let mut v_A2: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_tx: Py = Py::None;
    let mut v_ty: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("L_s"), v_L_s.clone())]))?;
    v_s = (if ({ let t255 = v_s.clone(); Py::Bool({ let t256 = Py::None; if py::is(&t255, &t256) { true } else { false } }) }).truthy() { v_L_s.clone() } else { v_s.clone() });
    let _ = f__pos(Py::dict(vec![(Py::str("s"), v_s.clone())]))?;
    let _ = f__need({ let t257 = v_s.clone(); Py::Bool({ let t258 = py::mul(&v_L_s.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t257, &t258)? { true } else { false } }) }, Py::str("the station must lie on the spiral (s <= L_s)"))?;
    v_A2 = py::mul(&v_R.clone(), &v_L_s.clone())?;
    v_th = py::truediv(&py::mul(&v_s.clone(), &v_s.clone())?, &py::mul(&Py::Int(2i64), &v_A2.clone())?)?;
    let _ = f__need({ let t259 = v_th.clone(); Py::Bool({ let t260 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t259, &t260)? { true } else { false } }) }, Py::str("the tangent angle exceeds 90 degrees (spiral too long for this radius)"))?;
    let t261 = Py::Float(0.0);
    v_x = t261.clone();
    v_y = t261.clone();
    for t262 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
        v_n = t262;
        v_tx = py::truediv(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_n.clone())?, &py::pow(&v_th.clone(), &py::mul(&Py::Int(2i64), &v_n.clone())?)?)?, &py::mul(&py::add(&py::mul(&Py::Int(4i64), &v_n.clone())?, &Py::Int(1i64))?, &py::factorial(&py::mul(&Py::Int(2i64), &v_n.clone())?)?)?)?;
        v_ty = py::truediv(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_n.clone())?, &py::pow(&v_th.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?)?)?, &py::mul(&py::add(&py::mul(&Py::Int(4i64), &v_n.clone())?, &Py::Int(3i64))?, &py::factorial(&py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(1i64))?)?)?)?;
        v_x = py::iadd(&v_x, &v_tx.clone())?;
        v_y = py::iadd(&v_y, &v_ty.clone())?;
        if ({ let t267 = { let t263 = py::abs(&v_tx.clone())?; Py::Bool({ let t264 = Py::Float(1e-18); if py::lt(&t263, &t264)? { true } else { false } }) }; if !t267.truthy() { t267 } else { { let t265 = py::abs(&v_ty.clone())?; Py::Bool({ let t266 = Py::Float(1e-18); if py::lt(&t265, &t266)? { true } else { false } }) } } }).truthy() {
            break;
        }
    }
    return Ok(Py::dict(vec![(Py::str("x"), f__r(py::mul(&v_s.clone(), &v_x.clone())?)?), (Py::str("y"), f__r(py::mul(&v_s.clone(), &v_y.clone())?)?), (Py::str("theta"), f__r(py::math1("degrees", &v_th.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coast_down(mut v_m: Py, mut v_v1: Py, mut v_v2: Py, mut v_C_D: Py, mut v_A: Py, mut v_rho: Py, mut v_f_r: Py, mut v_G: Py, mut v_gamma: Py) -> R {
    let mut v_F0: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("v1"), v_v1.clone()), (Py::str("C_D"), v_C_D.clone()), (Py::str("A"), v_A.clone()), (Py::str("rho"), v_rho.clone()), (Py::str("gamma"), v_gamma.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("f_r"), v_f_r.clone()), (Py::str("v2"), v_v2.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t291 = v_v2.clone(); Py::Bool({ let t292 = v_v1.clone(); if py::lt(&t291, &t292)? { true } else { false } }) }, Py::str("the final speed must be below the initial speed"))?;
    v_F0 = f__resist_const(v_m.clone(), v_f_r.clone(), v_G.clone(), Py::Float(0.0), Py::Float(1.0))?;
    let _ = f__need({ let t293 = v_F0.clone(); Py::Bool({ let t294 = Py::Int(0i64); if py::gt(&t293, &t294)? { true } else { false } }) }, Py::str("rolling resistance plus grade must retard the vehicle (F0 > 0)"))?;
    let t295 = (f__decel_drag(v_m.clone(), v_F0.clone(), py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_C_D.clone())?, &v_A.clone())?, &Py::Int(2i64))?, v_gamma.clone(), v_v1.clone(), v_v2.clone())?).items()?;
    if t295.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t295[0].clone();
    v_t = t295[1].clone();
    return Ok(Py::dict(vec![(Py::str("t"), f__r(v_t.clone())?), (Py::str("d"), f__r(v_S.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cornering(mut v_m: Py, mut v_L: Py, mut v_l_f: Py, mut v_C_f: Py, mut v_C_r: Py, mut v_u: Py, mut v_delta: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_l_r: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1934 = (f__bike_check(v_m.clone(), v_L.clone(), v_l_f.clone(), v_C_f.clone(), v_C_r.clone())?).items()?;
    if t1934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l_r = t1934[0].clone();
    v_K = t1934[1].clone();
    let _ = f__pos(Py::dict(vec![(Py::str("u"), v_u.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("delta"), v_delta.clone())]))?;
    let _ = f__need({ let t1935 = v_delta.clone(); Py::Bool({ let t1936 = Py::Int(0i64); if py::ne(&t1935, &t1936) { true } else { false } }) }, Py::str("give a non-zero steer angle"))?;
    v_den = py::add(&v_L.clone(), &py::mul(&py::mul(&v_K.clone(), &v_u.clone())?, &v_u.clone())?)?;
    let _ = f__need({ let t1937 = v_den.clone(); Py::Bool({ let t1938 = Py::Int(0i64); if py::gt(&t1937, &t1938)? { true } else { false } }) }, Py::str("above the critical speed: the vehicle has no steady turn"))?;
    v_r = py::truediv(&py::mul(&v_u.clone(), &v_delta.clone())?, &v_den.clone())?;
    v_beta = py::truediv(&py::mul(&v_delta.clone(), &py::sub(&v_l_r.clone(), &py::truediv(&py::mul(&py::mul(&py::mul(&v_m.clone(), &v_l_f.clone())?, &v_u.clone())?, &v_u.clone())?, &py::mul(&v_L.clone(), &v_C_r.clone())?)?)?)?, &v_den.clone())?;
    return Ok(Py::dict(vec![(Py::str("r"), f__r(v_r.clone())?), (Py::str("a_y"), f__r(py::mul(&v_u.clone(), &v_r.clone())?)?), (Py::str("R"), f__r(py::abs(&py::truediv(&v_u.clone(), &v_r.clone())?)?)?), (Py::str("beta"), f__r(py::math1("degrees", &v_beta.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crest_length(mut v_g1: Py, mut v_g2: Py, mut v_S: Py, mut v_h1: Py, mut v_h2: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("g1"), v_g1.clone()), (Py::str("g2"), v_g2.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("h1"), v_h1.clone()), (Py::str("h2"), v_h2.clone())]))?;
    v_A = py::sub(&v_g1.clone(), &v_g2.clone())?;
    let _ = f__need({ let t314 = v_A.clone(); Py::Bool({ let t315 = Py::Int(0i64); if py::gt(&t314, &t315)? { true } else { false } }) }, Py::str("a crest curve needs g1 > g2"))?;
    v_k = f__crest_k(v_h1.clone(), v_h2.clone())?;
    v_L = py::truediv(&py::mul(&py::mul(&v_A.clone(), &v_S.clone())?, &v_S.clone())?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?;
    if ({ let t316 = v_L.clone(); Py::Bool({ let t317 = v_S.clone(); if py::ge(&t316, &t317)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("L"), f__r(v_L.clone())?), (Py::str("case"), Py::str("S < L"))]));
    }
    v_L = py::sub(&py::mul(&Py::Int(2i64), &v_S.clone())?, &py::truediv(&py::mul(&Py::Int(2i64), &v_k.clone())?, &v_A.clone())?)?;
    if ({ let t327 = v_L.clone(); Py::Bool({ let t328 = Py::Int(0i64); if py::le(&t327, &t328)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("L"), Py::Float(0.0)), (Py::str("case"), Py::str("no curve needed"))]));
    }
    return Ok(Py::dict(vec![(Py::str("L"), f__r(v_L.clone())?), (Py::str("case"), Py::str("S > L"))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crest_sight(mut v_g1: Py, mut v_g2: Py, mut v_L: Py, mut v_h1: Py, mut v_h2: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("g1"), v_g1.clone()), (Py::str("g2"), v_g2.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("L"), v_L.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("h1"), v_h1.clone()), (Py::str("h2"), v_h2.clone())]))?;
    v_A = py::sub(&v_g1.clone(), &v_g2.clone())?;
    let _ = f__need({ let t343 = v_A.clone(); Py::Bool({ let t344 = Py::Int(0i64); if py::gt(&t343, &t344)? { true } else { false } }) }, Py::str("a crest curve needs g1 > g2"))?;
    v_k = f__crest_k(v_h1.clone(), v_h2.clone())?;
    v_S = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_k.clone())?, &v_L.clone())?, &v_A.clone())?)?;
    if ({ let t349 = { let t345 = v_L.clone(); Py::Bool({ let t346 = Py::Int(0i64); if py::gt(&t345, &t346)? { true } else { false } }) }; if !t349.truthy() { t349 } else { { let t347 = v_S.clone(); Py::Bool({ let t348 = v_L.clone(); if py::le(&t347, &t348)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::dict(vec![(Py::str("S"), f__r(v_S.clone())?), (Py::str("case"), Py::str("S < L"))]));
    }
    return Ok(Py::dict(vec![(Py::str("S"), f__r(py::add(&py::truediv(&v_L.clone(), &Py::Int(2i64))?, &py::truediv(&v_k.clone(), &v_A.clone())?)?)?), (Py::str("case"), Py::str("S > L"))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_curve_radius(mut v_v: Py, mut v_e: Py, mut v_f: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("f"), v_f.clone())]))?;
    let _ = f__e_check(v_e.clone())?;
    let _ = f__need({ let t1282 = py::mul(&v_e.clone(), &v_f.clone())?; Py::Bool({ let t1283 = Py::Int(1i64); if py::lt(&t1282, &t1283)? { true } else { false } }) }, Py::str("e f >= 1: friction and superelevation never limit the speed"))?;
    let _ = f__need({ let t1284 = py::add(&v_e.clone(), &v_f.clone())?; Py::Bool({ let t1285 = Py::Int(0i64); if py::gt(&t1284, &t1285)? { true } else { false } }) }, Py::str("e + f must be positive"))?;
    return Ok(Py::dict(vec![(Py::str("R_min"), f__r(py::truediv(&py::mul(&py::mul(&v_v.clone(), &v_v.clone())?, &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_f.clone())?)?)?, &py::mul(&c_G_()?, &py::add(&v_e.clone(), &v_f.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_curve_speed(mut v_R: Py, mut v_e: Py, mut v_f: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("f"), v_f.clone())]))?;
    let _ = f__e_check(v_e.clone())?;
    let _ = f__need({ let t371 = py::mul(&v_e.clone(), &v_f.clone())?; Py::Bool({ let t372 = Py::Int(1i64); if py::lt(&t371, &t372)? { true } else { false } }) }, Py::str("e f >= 1: friction and superelevation never limit the speed"))?;
    let _ = f__need({ let t373 = py::add(&v_e.clone(), &v_f.clone())?; Py::Bool({ let t374 = Py::Int(0i64); if py::gt(&t373, &t374)? { true } else { false } }) }, Py::str("e + f must be positive"))?;
    v_v = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&c_G_()?, &v_R.clone())?, &py::add(&v_e.clone(), &v_f.clone())?)?, &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_f.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("v"), f__r(v_v.clone())?), (Py::str("v_kmh"), f__kmh(v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_design_hourly_volume(mut v_AADT: Py, mut v_K: Py, mut v_D: Py) -> R {
    let mut v_dhv: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("AADT"), v_AADT.clone()), (Py::str("K"), v_K.clone()), (Py::str("D"), v_D.clone())]))?;
    let _ = f__need({ let t393 = v_K.clone(); Py::Bool({ let t394 = Py::Float(0.3); if py::le(&t393, &t394)? { true } else { false } }) }, Py::str("the K factor (share of the AADT in the design hour) must lie in (0, 0.3]"))?;
    let _ = f__need({ let t395 = Py::Float(0.5); Py::Bool({ let t396 = v_D.clone(); if py::le(&t395, &t396)? { { let t397 = Py::Int(1i64); if py::le(&t396, &t397)? { true } else { false } } } else { false } }) }, Py::str("the directional factor (share of the peak direction) must lie between 0.5 and 1"))?;
    v_dhv = py::mul(&v_K.clone(), &v_AADT.clone())?;
    return Ok(Py::dict(vec![(Py::str("DHV"), f__r(v_dhv.clone())?), (Py::str("DDHV"), f__r(py::mul(&v_D.clone(), &v_dhv.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_erlang_b(mut v_lam: Py, mut v_mu: Py, mut v_c: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("mu"), v_mu.clone())]))?;
    let _ = f__need({ let t473 = Py::Bool(matches!(v_c.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t473.truthy() { t473 } else { { let t470 = Py::Int(1i64); Py::Bool({ let t471 = v_c.clone(); if py::le(&t470, &t471)? { { let t472 = Py::Int(1000i64); if py::le(&t471, &t472)? { true } else { false } } } else { false } }) } } }, Py::str("the number of servers must be a whole number from 1 to 1000"))?;
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_B = Py::Float(1.0);
    for t474 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_c.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t474;
        v_B = py::truediv(&py::mul(&v_a.clone(), &v_B.clone())?, &py::add(&v_k.clone(), &py::mul(&v_a.clone(), &v_B.clone())?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("P_block"), f__r(v_B.clone())?), (Py::str("carried"), f__r(py::mul(&v_a.clone(), &py::sub(&Py::Int(1i64), &v_B.clone())?)?)?), (Py::str("lost_per_h"), f__vph(py::mul(&v_lam.clone(), &v_B.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_esal(mut v_loads: Py, mut v_counts: Py, mut v_Y: Py, mut v_P_s: Py, mut v_n: Py, mut v_g: Py, mut v_f_d: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_day: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_gf: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lef: Py = Py::None;
    let mut v_ps: Py = Py::None;
    v_L = f__vec(Py::str("axle loads"), v_loads.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_C = f__vec(Py::str("daily counts"), v_counts.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t422 = py::len(&v_L.clone())?; Py::Bool({ let t423 = py::len(&v_C.clone())?; if py::eq(&t422, &t423) { true } else { false } }) }, Py::str("give one daily count for each axle load"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("Y"), v_Y.clone())]))?;
    let _ = f__need({ let t433 = { let t428 = Py::Int(1i64); Py::Bool({ let t429 = v_Y.clone(); if py::le(&t428, &t429)? { { let t430 = Py::Int(50i64); if py::le(&t429, &t430)? { true } else { false } } } else { false } }) }; if !t433.truthy() { t433 } else { { let t431 = v_Y.clone(); Py::Bool({ let t432 = py::math1("floor", &v_Y.clone())?; if py::eq(&t431, &t432) { true } else { false } }) } } }, Py::str("the design period must be a whole number of years from 1 to 50"))?;
    let _ = f__pos(Py::dict(vec![(Py::str("P_s"), v_P_s.clone()), (Py::str("n"), v_n.clone()), (Py::str("f_d"), v_f_d.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("g"), v_g.clone())]))?;
    let _ = f__need({ let t439 = Py::Int(2i64); Py::Bool({ let t440 = v_n.clone(); if py::le(&t439, &t440)? { { let t441 = Py::Int(6i64); if py::le(&t440, &t441)? { true } else { false } } } else { false } }) }, Py::str("the load-equivalency exponent must lie between 2 and 6"))?;
    let _ = f__need({ let t442 = Py::Int(0i64); Py::Bool({ let t443 = v_g.clone(); if py::le(&t442, &t443)? { { let t444 = Py::Float(0.2); if py::le(&t443, &t444)? { true } else { false } } } else { false } }) }, Py::str("the annual growth rate must lie between 0 and 20%"))?;
    let _ = f__need({ let t445 = v_f_d.clone(); Py::Bool({ let t446 = Py::Int(1i64); if py::le(&t445, &t446)? { true } else { false } }) }, Py::str("the design-lane factor must lie in (0, 1]"))?;
    v_ps = py::truediv(&v_P_s.clone(), &Py::Float(1000.0))?;
    v_lef = Py::list(vec![]);
    v_day = Py::Float(0.0);
    for t447 in py::range_iter(&[(py::len(&v_L.clone())?).i()?]) {
        v_i = t447;
        v_e = py::pow(&py::truediv(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_ps.clone())?, &v_n.clone())?;
        let _ = py::method(&v_lef.clone(), "append", &[v_e.clone()])?;
        v_day = py::iadd(&v_day, &py::mul(&py::getitem(&v_C.clone(), &v_i.clone())?, &v_e.clone())?)?;
    }
    let _ = f__need({ let t448 = v_day.clone(); Py::Bool({ let t449 = Py::Int(0i64); if py::gt(&t448, &t449)? { true } else { false } }) }, Py::str("the traffic carries no axle loads"))?;
    if ({ let t450 = v_g.clone(); Py::Bool({ let t451 = Py::Int(0i64); if py::eq(&t450, &t451) { true } else { false } }) }).truthy() {
        v_gf = py::float(&v_Y.clone())?;
    } else {
        v_gf = py::truediv(&py::math1("expm1", &py::mul(&v_Y.clone(), &py::math1("log1p", &v_g.clone())?)?)?, &v_g.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("LEF"), f__r(v_lef.clone())?), (Py::str("ESAL_day"), f__r(v_day.clone())?), (Py::str("GF"), f__r(v_gf.clone())?), (Py::str("ESAL"), f__r(py::mul(&py::mul(&py::mul(&v_day.clone(), &Py::Int(365i64))?, &v_gf.clone())?, &v_f_d.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fastest_route(mut v_roads: Py, mut v_speeds: Py, mut v_origin: Py, mut v_destination: Py, mut v_directed: Py) -> R {
    let mut v__n: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_directed = f__truthy(v_directed.clone())?;
    v_adj = f__graph(v_roads.clone(), v_speeds.clone(), v_directed.clone())?;
    let _ = f__need({ let t526 = { let t522 = v_origin.clone(); Py::Bool({ let t523 = v_adj.clone(); if py::contains(&t523, &t522)? { true } else { false } }) }; if !t526.truthy() { t526 } else { { let t524 = v_destination.clone(); Py::Bool({ let t525 = v_adj.clone(); if py::contains(&t525, &t524)? { true } else { false } }) } } }, Py::str("the origin and the destination must be places on the network"))?;
    let _ = f__need({ let t527 = v_origin.clone(); Py::Bool({ let t528 = v_destination.clone(); if py::ne(&t527, &t528) { true } else { false } }) }, Py::str("the origin and the destination must differ"))?;
    v_best = f__dijkstra(v_adj.clone(), v_origin.clone())?;
    let _ = f__need({ let t540 = v_destination.clone(); Py::Bool({ let t541 = v_best.clone(); if py::contains(&t541, &t540)? { true } else { false } }) }, Py::str("the destination cannot be reached from the origin"))?;
    let t542 = (py::getitem(&v_best.clone(), &v_destination.clone())?).items()?;
    if t542.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_t = t542[0].clone();
    v__n = t542[1].clone();
    v_path = t542[2].clone();
    v_dist = t542[3].clone();
    return Ok(Py::dict(vec![(Py::str("route"), py::method(&Py::str("-"), "join", &[v_path.clone()])?), (Py::str("time"), f__r(v_t.clone())?), (Py::str("distance"), f__r(v_dist.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fifo_queue(mut v_arrivals: Py, mut v_service: Py, mut v_c: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dep: Py = Py::None;
    let mut v_events: Py = Py::None;
    let mut v_free: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qmax: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_start: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_waits: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_t = f__vec(Py::str("arrivals"), v_arrivals.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    v_s = f__vec(Py::str("service"), v_service.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t569 = Py::Bool(matches!(v_c.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t569.truthy() { t569 } else { { let t567 = v_c.clone(); Py::Bool({ let t568 = Py::Int(1i64); if py::ge(&t567, &t568)? { true } else { false } }) } } }, Py::str("the number of servers must be a whole number >= 1"))?;
    if ({ let t570 = py::len(&v_s.clone())?; Py::Bool({ let t571 = Py::Int(1i64); if py::eq(&t570, &t571) { true } else { false } }) }).truthy() {
        v_s = py::mul(&v_s.clone(), &py::len(&v_t.clone())?)?;
    }
    let _ = f__need({ let t572 = py::len(&v_s.clone())?; Py::Bool({ let t573 = py::len(&v_t.clone())?; if py::eq(&t572, &t573) { true } else { false } }) }, Py::str("give one service time for every arrival (or one for all)"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t574 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t575 = (t574).items()?;
if t575.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t575[0].clone();
v_b = t575[1].clone();
out.push({ let t576 = v_b.clone(); Py::Bool({ let t577 = v_a.clone(); if py::ge(&t576, &t577)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("arrival times must be in increasing order"))?;
    v_free = py::mul(&Py::list(vec![py::getitem(&v_t.clone(), &Py::Int(0i64))?]), &v_c.clone())?;
    let t578 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t578.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_waits = t578[0].clone();
    v_dep = t578[1].clone();
    for t579 in py::iter(&(py::zip(&[v_t.clone(), v_s.clone()])?))? {
        let t580 = (t579).items()?;
        if t580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t580[0].clone();
        v_x = t580[1].clone();
        v_start = py::minmax(&[v_a.clone(), py::getitem(&v_free.clone(), &Py::Int(0i64))?], true)?;
        let _ = py::method(&v_waits.clone(), "append", &[py::sub(&v_start.clone(), &v_a.clone())?])?;
        let _ = py::method(&v_dep.clone(), "append", &[py::add(&v_start.clone(), &v_x.clone())?])?;
        py::setitem(&v_free.clone(), &Py::Int(0i64), py::add(&v_start.clone(), &v_x.clone())?)?;
        let _ = py::method_sort(&v_free.clone(), None, false)?;
    }
    v_events = py::sorted_key(&py::add(&{ let mut out = vec![]; for t581 in py::iter(&(py::zip(&[v_t.clone(), v_waits.clone()])?))? { let mut v_a: Py; let mut v_w: Py; 
let t582 = (t581).items()?;
if t582.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t582[0].clone();
v_w = t582[1].clone();
if ({ let t583 = v_w.clone(); Py::Bool({ let t584 = Py::Int(0i64); if py::gt(&t583, &t584)? { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![v_a.clone(), Py::Int(1i64)])); }} Py::list(out) }, &{ let mut out = vec![]; for t585 in py::iter(&(py::zip(&[v_t.clone(), v_waits.clone()])?))? { let mut v_a: Py; let mut v_w: Py; 
let t586 = (t585).items()?;
if t586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t586[0].clone();
v_w = t586[1].clone();
if ({ let t587 = v_w.clone(); Py::Bool({ let t588 = Py::Int(0i64); if py::gt(&t587, &t588)? { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![py::add(&v_a.clone(), &v_w.clone())?, py::neg(&Py::Int(1i64))?])); }} Py::list(out) })?, &{ let v_a = v_a.clone(); let v_arrivals = v_arrivals.clone(); let v_c = v_c.clone(); let v_dep = v_dep.clone(); let v_free = v_free.clone(); let v_s = v_s.clone(); let v_service = v_service.clone(); let v_start = v_start.clone(); let v_t = v_t.clone(); let v_waits = v_waits.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_e.clone(), &Py::Int(0i64))?, py::getitem(&v_e.clone(), &Py::Int(1i64))?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    let t589 = Py::Int(0i64);
    v_q = t589.clone();
    v_qmax = t589.clone();
    for t590 in py::iter(&(v_events.clone()))? {
        let t591 = (t590).items()?;
        if t591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v__ = t591[0].clone();
        v_d = t591[1].clone();
        v_q = py::iadd(&v_q, &v_d.clone())?;
        v_qmax = py::minmax(&[v_qmax.clone(), v_q.clone()], true)?;
    }
    v_n = py::len(&v_t.clone())?;
    return Ok(Py::dict(vec![(Py::str("waits"), f__r(v_waits.clone())?), (Py::str("mean_wait"), f__r(py::truediv(&py::fsum(&v_waits.clone())?, &v_n.clone())?)?), (Py::str("max_wait"), f__r(py::minmax(&[v_waits.clone()], true)?)?), (Py::str("mean_time_in_system"), f__r(py::truediv(&py::fsum(&{ let mut out = vec![]; for t601 in py::iter(&(py::zip(&[v_waits.clone(), v_s.clone()])?))? { let mut v_w: Py; let mut v_x: Py; 
let t602 = (t601).items()?;
if t602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_w = t602[0].clone();
v_x = t602[1].clone();
out.push(py::add(&v_w.clone(), &v_x.clone())?); } Py::list(out) })?, &v_n.clone())?)?), (Py::str("last_departure"), f__r(py::minmax(&[v_dep.clone()], true)?)?), (Py::str("max_queue"), py::float(&v_qmax.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_following_gap(mut v_v: Py, mut v_t_r: Py, mut v_a_l: Py, mut v_a_f: Py) -> R {
    let mut v_Tf: Py = Py::None;
    let mut v_Tl: Py = Py::None;
    let mut v_cand: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_xf: Py = Py::None;
    let mut v_xl: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("t_r"), v_t_r.clone()), (Py::str("a_l"), v_a_l.clone()), (Py::str("a_f"), v_a_f.clone())]))?;
    let t1598 = (Py::tuple(vec![py::truediv(&v_v.clone(), &v_a_l.clone())?, py::add(&v_t_r.clone(), &py::truediv(&v_v.clone(), &v_a_f.clone())?)?])).items()?;
    if t1598.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Tl = t1598[0].clone();
    v_Tf = t1598[1].clone();
    v_xl = { let v_Tf = v_Tf.clone(); let v_Tl = v_Tl.clone(); let v_a_f = v_a_f.clone(); let v_a_l = v_a_l.clone(); let v_t_r = v_t_r.clone(); let v_v = v_v.clone(); let v_xl = v_xl.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            v_t = py::minmax(&[v_t.clone(), v_Tl.clone()], false)?;
            return Ok(py::sub(&py::mul(&v_v.clone(), &v_t.clone())?, &py::truediv(&py::mul(&py::mul(&v_a_l.clone(), &v_t.clone())?, &v_t.clone())?, &Py::Int(2i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_xf = { let v_Tf = v_Tf.clone(); let v_Tl = v_Tl.clone(); let v_a_f = v_a_f.clone(); let v_a_l = v_a_l.clone(); let v_t_r = v_t_r.clone(); let v_v = v_v.clone(); let v_xf = v_xf.clone(); let v_xl = v_xl.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_u: Py = Py::None;
            if ({ let t1599 = v_t.clone(); Py::Bool({ let t1600 = v_t_r.clone(); if py::le(&t1599, &t1600)? { true } else { false } }) }).truthy() {
                return Ok(py::mul(&v_v.clone(), &v_t.clone())?);
            }
            v_u = py::sub(&py::minmax(&[v_t.clone(), v_Tf.clone()], false)?, &v_t_r.clone())?;
            return Ok(py::sub(&py::add(&py::mul(&v_v.clone(), &v_t_r.clone())?, &py::mul(&v_v.clone(), &v_u.clone())?)?, &py::truediv(&py::mul(&py::mul(&v_a_f.clone(), &v_u.clone())?, &v_u.clone())?, &Py::Int(2i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_cand = Py::list(vec![v_t_r.clone(), py::minmax(&[v_Tl.clone(), v_Tf.clone()], true)?]);
    if ({ let t1601 = v_a_f.clone(); Py::Bool({ let t1602 = v_a_l.clone(); if py::gt(&t1601, &t1602)? { true } else { false } }) }).truthy() {
        v_ts = py::truediv(&py::mul(&v_a_f.clone(), &v_t_r.clone())?, &py::sub(&v_a_f.clone(), &v_a_l.clone())?)?;
        if ({ let t1603 = v_t_r.clone(); Py::Bool({ let t1604 = v_ts.clone(); if py::lt(&t1603, &t1604)? { { let t1605 = py::minmax(&[v_Tl.clone(), v_Tf.clone()], false)?; if py::lt(&t1604, &t1605)? { true } else { false } } } else { false } }) }).truthy() {
            let _ = py::method(&v_cand.clone(), "append", &[v_ts.clone()])?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("gap"), f__r(py::minmax(&[{ let mut out = vec![]; for t1615 in py::iter(&(v_cand.clone()))? { let mut v_t: Py; 
v_t = t1615;
out.push(py::sub(&py::call(&v_xf.clone(), &[v_t.clone()])?, &py::call(&v_xl.clone(), &[v_t.clone()])?)?); } Py::list(out) }], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_freeway_los(mut v_V: Py, mut v_PHF: Py, mut v_N: Py, mut v_P_T: Py, mut v_E_T: Py, mut v_S: Py, mut v_f_p: Py) -> R {
    let mut v_d_km: Py = Py::None;
    let mut v_d_mi: Py = Py::None;
    let mut v_f_hv: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_los: Py = Py::None;
    let mut v_vp: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("PHF"), v_PHF.clone()), (Py::str("E_T"), v_E_T.clone()), (Py::str("S"), v_S.clone()), (Py::str("f_p"), v_f_p.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("P_T"), v_P_T.clone())]))?;
    let _ = f__need({ let t616 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t616.truthy() { t616 } else { { let t615 = py::not(&Py::Bool(matches!(v_N.clone(), Py::Bool(_)))); if !t615.truthy() { t615 } else { { let t612 = Py::Int(1i64); Py::Bool({ let t613 = v_N.clone(); if py::le(&t612, &t613)? { { let t614 = Py::Int(8i64); if py::le(&t613, &t614)? { true } else { false } } } else { false } }) } } } } }, Py::str("the number of lanes in one direction must be a whole number from 1 to 8"))?;
    let _ = f__need({ let t617 = Py::Float(0.25); Py::Bool({ let t618 = v_PHF.clone(); if py::le(&t617, &t618)? { { let t619 = Py::Int(1i64); if py::le(&t618, &t619)? { true } else { false } } } else { false } }) }, Py::str("the peak-hour factor must lie between 0.25 and 1"))?;
    let _ = f__need({ let t620 = Py::Int(0i64); Py::Bool({ let t621 = v_P_T.clone(); if py::le(&t620, &t621)? { { let t622 = Py::Int(1i64); if py::lt(&t621, &t622)? { true } else { false } } } else { false } }) }, Py::str("the heavy-vehicle share must lie in [0, 1)"))?;
    let _ = f__need({ let t623 = Py::Int(1i64); Py::Bool({ let t624 = v_E_T.clone(); if py::le(&t623, &t624)? { { let t625 = Py::Int(8i64); if py::le(&t624, &t625)? { true } else { false } } } else { false } }) }, Py::str("the passenger-car equivalent of a heavy vehicle must lie between 1 and 8"))?;
    let _ = f__need({ let t626 = Py::Float(0.7); Py::Bool({ let t627 = v_f_p.clone(); if py::le(&t626, &t627)? { { let t628 = Py::Int(1i64); if py::le(&t627, &t628)? { true } else { false } } } else { false } }) }, Py::str("the driver-population factor must lie between 0.7 and 1"))?;
    let _ = f__need({ let t629 = py::truediv(&Py::Int(30i64), &Py::Float(3.6))?; Py::Bool({ let t630 = v_S.clone(); if py::le(&t629, &t630)? { { let t631 = py::truediv(&Py::Int(140i64), &Py::Float(3.6))?; if py::le(&t630, &t631)? { true } else { false } } } else { false } }) }, Py::str("the mean passenger-car speed must lie between 30 and 140 km/h"))?;
    v_f_hv = py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::mul(&v_P_T.clone(), &py::sub(&v_E_T.clone(), &Py::Int(1i64))?)?)?)?;
    v_vp = py::truediv(&py::mul(&v_V.clone(), &Py::Int(3600i64))?, &py::mul(&py::mul(&py::mul(&v_PHF.clone(), &v_N.clone())?, &v_f_hv.clone())?, &v_f_p.clone())?)?;
    v_d_km = py::truediv(&v_vp.clone(), &py::mul(&v_S.clone(), &Py::Float(3.6))?)?;
    v_d_mi = py::mul(&v_d_km.clone(), &Py::Float(1.609344))?;
    v_los = Py::str("F");
    for t632 in py::range_iter(&[(Py::Int(5i64)).i()?]) {
        v_i = t632;
        if ({ let t633 = v_d_mi.clone(); Py::Bool({ let t634 = py::getitem(&c__LOS_LIMITS_MI()?, &v_i.clone())?; if py::le(&t633, &t634)? { true } else { false } }) }).truthy() {
            v_los = py::getitem(&c__LOS_NAMES()?, &v_i.clone())?;
            break;
        }
    }
    return Ok(Py::dict(vec![(Py::str("f_HV"), f__r(v_f_hv.clone())?), (Py::str("v_p"), f__r(v_vp.clone())?), (Py::str("D"), f__r(v_d_km.clone())?), (Py::str("D_mi"), f__r(v_d_mi.clone())?), (Py::str("LOS"), v_los.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_furness(mut v_base: Py, mut v_O: Py, mut v_D: Py) -> R {
    let mut v_t: Py = Py::None;
    let t661 = (f__balance_check(v_O.clone(), v_D.clone())?).items()?;
    if t661.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_O = t661[0].clone();
    v_D = t661[1].clone();
    v_t = f__mat(Py::str("base"), v_base.clone(), py::len(&v_O.clone())?, py::len(&v_D.clone())?, Py::Bool(true), Py::Bool(false))?;
    return Ok(Py::dict(vec![(Py::str("trips"), { let mut out = vec![]; for t694 in py::iter(&(f__furness(v_t.clone(), v_O.clone(), v_D.clone(), Py::Float(1e-13), Py::Int(20000i64))?))? { let mut v_row: Py; 
v_row = t694;
out.push(f__r(v_row.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gap_capacity(mut v_q: Py, mut v_t_c: Py, mut v_t_f: Py, mut v_model: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("t_c"), v_t_c.clone()), (Py::str("t_f"), v_t_f.clone())]))?;
    if ({ let t713 = v_model.clone(); Py::Bool({ let t714 = Py::str("harders"); if py::eq(&t713, &t714) { true } else { false } }) }).truthy() {
        v_c = py::truediv(&py::mul(&v_q.clone(), &py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &v_t_c.clone())?)?)?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_q.clone())?, &v_t_f.clone())?)?)?)?;
    } else {
        let _ = f__need({ let t715 = v_t_c.clone(); Py::Bool({ let t716 = py::truediv(&v_t_f.clone(), &Py::Int(2i64))?; if py::gt(&t715, &t716)? { true } else { false } }) }, Py::str("Siegloch's model needs t_c > t_f / 2"))?;
        v_c = py::truediv(&py::math1("exp", &py::mul(&py::neg(&v_q.clone())?, &py::sub(&v_t_c.clone(), &py::truediv(&v_t_f.clone(), &Py::Int(2i64))?)?)?)?, &v_t_f.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("c"), f__vph(v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gravity(mut v_O: Py, mut v_D: Py, mut v_costs: Py, mut v_beta: Py, mut v_deterrence: Py) -> R {
    let mut v_base: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let t743 = (f__balance_check(v_O.clone(), v_D.clone())?).items()?;
    if t743.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_O = t743[0].clone();
    v_D = t743[1].clone();
    v_c = f__mat(Py::str("costs"), v_costs.clone(), py::len(&v_O.clone())?, py::len(&v_D.clone())?, Py::Bool(true), Py::Bool(false))?;
    let _ = f__finite(Py::dict(vec![(Py::str("beta"), v_beta.clone())]))?;
    let _ = f__need({ let t763 = v_beta.clone(); Py::Bool({ let t764 = Py::Int(0i64); if py::ge(&t763, &t764)? { true } else { false } }) }, Py::str("beta must not be negative"))?;
    if ({ let t765 = v_deterrence.clone(); Py::Bool({ let t766 = Py::str("power"); if py::eq(&t765, &t766) { true } else { false } }) }).truthy() {
        let _ = f__need(Py::Bool(({ let mut out = vec![]; for t767 in py::iter(&(v_c.clone()))? { let mut v_r: Py; 
v_r = t767;
for t768 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t768;
out.push({ let t769 = v_x.clone(); Py::Bool({ let t770 = Py::Int(0i64); if py::gt(&t769, &t770)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the power deterrence function needs positive costs"))?;
        v_f = { let mut out = vec![]; for t771 in py::iter(&(v_c.clone()))? { let mut v_r: Py; 
v_r = t771;
out.push({ let mut out = vec![]; for t772 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t772;
out.push(py::pow(&v_x.clone(), &py::neg(&v_beta.clone())?)?); } Py::list(out) }); } Py::list(out) };
    } else {
        v_f = { let mut out = vec![]; for t773 in py::iter(&(v_c.clone()))? { let mut v_r: Py; 
v_r = t773;
out.push({ let mut out = vec![]; for t774 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t774;
out.push(py::math1("exp", &py::mul(&py::neg(&v_beta.clone())?, &v_x.clone())?)?); } Py::list(out) }); } Py::list(out) };
    }
    v_base = { let mut out = vec![]; for t775 in py::range_iter(&[(py::len(&v_O.clone())?).i()?]) { let mut v_i: Py; 
v_i = t775;
out.push({ let mut out = vec![]; for t776 in py::range_iter(&[(py::len(&v_D.clone())?).i()?]) { let mut v_j: Py; 
v_j = t776;
out.push(py::mul(&py::getitem(&py::getitem(&v_f.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_D.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("trips"), { let mut out = vec![]; for t794 in py::iter(&(f__furness(v_base.clone(), v_O.clone(), v_D.clone(), Py::Float(1e-13), Py::Int(20000i64))?))? { let mut v_row: Py; 
v_row = t794;
out.push(f__r(v_row.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_green_splits(mut v_C: Py, mut v_L: Py, mut v_y: Py) -> R {
    let mut v_Y: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("C"), v_C.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("L"), v_L.clone())]))?;
    v_ys = f__vec(Py::str("y"), v_y.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t1751 = py::len(&v_ys.clone())?; Py::Bool({ let t1752 = Py::Int(2i64); if py::ge(&t1751, &t1752)? { true } else { false } }) }, Py::str("give the flow ratio of at least two critical phases"))?;
    let _ = f__need({ let t1753 = v_C.clone(); Py::Bool({ let t1754 = v_L.clone(); if py::gt(&t1753, &t1754)? { true } else { false } }) }, Py::str("the cycle must be longer than the lost time"))?;
    v_Y = py::sum(&v_ys.clone(), None)?;
    let _ = f__need({ let t1755 = v_Y.clone(); Py::Bool({ let t1756 = Py::Int(1i64); if py::lt(&t1755, &t1756)? { true } else { false } }) }, Py::str("the critical flow ratios add up to 1 or more: no cycle can serve them"))?;
    return Ok(Py::dict(vec![(Py::str("g"), f__r({ let mut out = vec![]; for t1766 in py::iter(&(v_ys.clone()))? { let mut v_yi: Py; 
v_yi = t1766;
out.push(py::mul(&py::truediv(&v_yi.clone(), &v_Y.clone())?, &py::sub(&v_C.clone(), &v_L.clone())?)?); } Py::list(out) })?), (Py::str("X_c"), f__r(py::truediv(&py::mul(&v_Y.clone(), &v_C.clone())?, &py::sub(&v_C.clone(), &v_L.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greenberg_cap(mut v_v0: Py, mut v_kj: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("v0"), v_v0.clone()), (Py::str("kj"), v_kj.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("q_max"), f__vph(py::truediv(&py::mul(&v_v0.clone(), &v_kj.clone())?, &Py::Float(std::f64::consts::E))?)?), (Py::str("k_m"), f__vpkm(py::truediv(&v_kj.clone(), &Py::Float(std::f64::consts::E))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greenberg_fit(mut v_speeds: Py, mut v_densities: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kj: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_v0: Py = Py::None;
    let t841 = (f__check_obs(v_speeds.clone(), v_densities.clone())?).items()?;
    if t841.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t841[0].clone();
    v_k = t841[1].clone();
    let t852 = (f__linfit({ let mut out = vec![]; for t851 in py::iter(&(v_k.clone()))? { let mut v_x: Py; 
v_x = t851;
out.push(py::math1("log", &v_x.clone())?); } Py::list(out) }, v_v.clone())?).items()?;
    if t852.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t852[0].clone();
    v_b = t852[1].clone();
    v_r2 = t852[2].clone();
    let _ = f__need({ let t853 = v_b.clone(); Py::Bool({ let t854 = Py::Int(0i64); if py::lt(&t853, &t854)? { true } else { false } }) }, Py::str("the speeds do not fall with density: the Greenberg model does not fit"))?;
    v_v0 = py::neg(&v_b.clone())?;
    v_kj = py::math1("exp", &py::truediv(&v_a.clone(), &v_v0.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("v0"), f__r(v_v0.clone())?), (Py::str("kj"), f__r(v_kj.clone())?), (Py::str("q_max"), f__r(py::truediv(&py::mul(&v_v0.clone(), &v_kj.clone())?, &Py::Float(std::f64::consts::E))?)?), (Py::str("r2"), f__r(v_r2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greenshields_at_flow(mut v_vf: Py, mut v_kj: Py, mut v_q: Py) -> R {
    let mut v_disc: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("vf"), v_vf.clone()), (Py::str("kj"), v_kj.clone()), (Py::str("q"), v_q.clone())]))?;
    v_disc = py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&Py::Int(4i64), &v_q.clone())?, &py::mul(&v_vf.clone(), &v_kj.clone())?)?)?;
    let _ = f__need({ let t959 = v_disc.clone(); Py::Bool({ let t960 = py::neg(&Py::Float(1e-12))?; if py::ge(&t959, &t960)? { true } else { false } }) }, Py::str("the flow exceeds the capacity vf kj / 4 of this traffic stream"))?;
    v_sd = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), v_disc.clone()], true)?)?;
    v_k1 = py::truediv(&py::mul(&Py::Int(2i64), &v_q.clone())?, &py::mul(&v_vf.clone(), &py::add(&Py::Int(1i64), &v_sd.clone())?)?)?;
    v_k2 = py::truediv(&py::mul(&v_kj.clone(), &py::add(&Py::Int(1i64), &v_sd.clone())?)?, &Py::Int(2i64))?;
    return Ok(Py::dict(vec![(Py::str("k_1"), f__vpkm(v_k1.clone())?), (Py::str("v_1"), f__kmh(py::mul(&v_vf.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&v_k1.clone(), &v_kj.clone())?)?)?)?), (Py::str("k_2"), f__vpkm(v_k2.clone())?), (Py::str("v_2"), f__kmh(py::mul(&v_vf.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&v_k2.clone(), &v_kj.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greenshields_cap(mut v_vf: Py, mut v_kj: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("vf"), v_vf.clone()), (Py::str("kj"), v_kj.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("q_max"), f__vph(py::truediv(&py::mul(&v_vf.clone(), &v_kj.clone())?, &Py::Int(4i64))?)?), (Py::str("k_m"), f__vpkm(py::truediv(&v_kj.clone(), &Py::Int(2i64))?)?), (Py::str("v_m"), f__kmh(py::truediv(&v_vf.clone(), &Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greenshields_fit(mut v_speeds: Py, mut v_densities: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kj: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vf: Py = Py::None;
    let t901 = (f__check_obs(v_speeds.clone(), v_densities.clone())?).items()?;
    if t901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t901[0].clone();
    v_k = t901[1].clone();
    let t911 = (f__linfit(v_k.clone(), v_v.clone())?).items()?;
    if t911.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t911[0].clone();
    v_b = t911[1].clone();
    v_r2 = t911[2].clone();
    let _ = f__need({ let t916 = { let t912 = v_a.clone(); Py::Bool({ let t913 = Py::Int(0i64); if py::gt(&t912, &t913)? { true } else { false } }) }; if !t916.truthy() { t916 } else { { let t914 = v_b.clone(); Py::Bool({ let t915 = Py::Int(0i64); if py::lt(&t914, &t915)? { true } else { false } }) } } }, Py::str("the speeds do not fall with density: the Greenshields line does not fit"))?;
    let t917 = (Py::tuple(vec![v_a.clone(), py::truediv(&py::neg(&v_a.clone())?, &v_b.clone())?])).items()?;
    if t917.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vf = t917[0].clone();
    v_kj = t917[1].clone();
    return Ok(Py::dict(vec![(Py::str("vf"), f__r(v_vf.clone())?), (Py::str("kj"), f__r(v_kj.clone())?), (Py::str("q_max"), f__r(py::truediv(&py::mul(&v_vf.clone(), &v_kj.clone())?, &Py::Int(4i64))?)?), (Py::str("r2"), f__r(v_r2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gs_shockwave(mut v_vf: Py, mut v_kj: Py, mut v_k1: Py, mut v_k2: Py) -> R {
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("vf"), v_vf.clone()), (Py::str("kj"), v_kj.clone()), (Py::str("k1"), v_k1.clone()), (Py::str("k2"), v_k2.clone())]))?;
    let _ = f__need({ let t936 = v_k1.clone(); Py::Bool({ let t937 = v_k2.clone(); if py::lt(&t936, &t937)? { { let t938 = v_kj.clone(); if py::lt(&t937, &t938)? { true } else { false } } } else { false } }) }, Py::str("a Greenshields shock needs k1 < k2 < kj (a lighter state running into a denser one)"))?;
    v_u = py::mul(&v_vf.clone(), &py::sub(&Py::Int(1i64), &py::truediv(&py::add(&v_k1.clone(), &v_k2.clone())?, &v_kj.clone())?)?)?;
    v_u = f__snap(v_u.clone(), v_vf.clone())?;
    return Ok(Py::dict(vec![(Py::str("u"), f__kmh(v_u.clone())?), (Py::str("u_ms"), f__r(v_u.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_headway_prob(mut v_q: Py, mut v_h: Py, mut v_tau: Py) -> R {
    let mut v_mean: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("q"), v_q.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("tau"), v_tau.clone())]))?;
    v_mean = py::truediv(&Py::Int(1i64), &v_q.clone())?;
    let _ = f__need({ let t984 = v_tau.clone(); Py::Bool({ let t985 = v_mean.clone(); if py::lt(&t984, &t985)? { true } else { false } }) }, Py::str("the minimum headway must be shorter than the mean headway 1/q"))?;
    if ({ let t986 = v_h.clone(); Py::Bool({ let t987 = v_tau.clone(); if py::le(&t986, &t987)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("P_ge"), Py::Float(1.0)), (Py::str("P_lt"), Py::Float(0.0))]));
    }
    v_z = py::truediv(&py::sub(&v_h.clone(), &v_tau.clone())?, &py::sub(&v_mean.clone(), &v_tau.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("P_ge"), f__r(py::math1("exp", &py::neg(&v_z.clone())?)?)?), (Py::str("P_lt"), f__r(py::neg(&py::math1("expm1", &py::neg(&v_z.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hso(mut v_R: Py, mut v_S: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("S"), v_S.clone())]))?;
    let _ = f__need({ let t1006 = v_S.clone(); Py::Bool({ let t1007 = py::mul(&Py::Float(std::f64::consts::PI), &v_R.clone())?; if py::lt(&t1006, &t1007)? { true } else { false } }) }, Py::str("the sight distance must be shorter than half the circumference"))?;
    return Ok(Py::dict(vec![(Py::str("M"), f__r(py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &py::pow(&py::math1("sin", &py::truediv(&v_S.clone(), &py::mul(&Py::Int(4i64), &v_R.clone())?)?)?, &Py::Int(2i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hso_sight(mut v_R: Py, mut v_M: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("M"), v_M.clone())]))?;
    let _ = f__need({ let t1711 = v_M.clone(); Py::Bool({ let t1712 = v_R.clone(); if py::lt(&t1711, &t1712)? { true } else { false } }) }, Py::str("the offset must be less than the radius"))?;
    return Ok(Py::dict(vec![(Py::str("S"), f__r(py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &py::math1("acos", &py::sub(&Py::Int(1i64), &py::truediv(&v_M.clone(), &v_R.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_impact_speed(mut v_v: Py, mut v_D: Py, mut v_t_r: Py, mut v_f: Py, mut v_G: Py) -> R {
    let mut v_dec: Py = Py::None;
    let mut v_dr: Py = Py::None;
    let mut v_stop: Py = Py::None;
    let mut v_vi: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("D"), v_D.clone()), (Py::str("f"), v_f.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("t_r"), v_t_r.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    v_dec = py::mul(&Py::Float(9.81), &py::add(&v_f.clone(), &v_G.clone())?)?;
    let _ = f__need({ let t1031 = v_dec.clone(); Py::Bool({ let t1032 = Py::Int(0i64); if py::gt(&t1031, &t1032)? { true } else { false } }) }, Py::str("f + G must be positive"))?;
    v_dr = py::mul(&v_v.clone(), &v_t_r.clone())?;
    v_stop = py::add(&v_dr.clone(), &py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&Py::Int(2i64), &v_dec.clone())?)?)?;
    if ({ let t1033 = v_D.clone(); Py::Bool({ let t1034 = v_dr.clone(); if py::le(&t1033, &t1034)? { true } else { false } }) }).truthy() {
        v_vi = v_v.clone();
    } else {
        if ({ let t1035 = v_D.clone(); Py::Bool({ let t1036 = v_stop.clone(); if py::ge(&t1035, &t1036)? { true } else { false } }) }).truthy() {
            v_vi = Py::Float(0.0);
        } else {
            v_vi = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::sub(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_dec.clone())?, &py::sub(&v_D.clone(), &v_dr.clone())?)?)?], true)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("v_impact"), f__r(v_vi.clone())?), (Py::str("v_impact_kmh"), f__kmh(v_vi.clone())?), (Py::str("d_stop"), f__r(v_stop.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_interval_queue(mut v_arrivals: Py, mut v_capacity: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_nq: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_queue: Py = Py::None;
    let mut v_served: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_a = f__vec(Py::str("arrivals"), v_arrivals.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    v_c = f__vec(Py::str("capacity"), v_capacity.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    if ({ let t1061 = py::len(&v_c.clone())?; Py::Bool({ let t1062 = Py::Int(1i64); if py::eq(&t1061, &t1062) { true } else { false } }) }).truthy() {
        v_c = py::mul(&v_c.clone(), &py::len(&v_a.clone())?)?;
    }
    let _ = f__need({ let t1063 = py::len(&v_c.clone())?; Py::Bool({ let t1064 = py::len(&v_a.clone())?; if py::eq(&t1063, &t1064) { true } else { false } }) }, Py::str("give one capacity for every interval (or a single capacity for all)"))?;
    let t1065 = (Py::tuple(vec![Py::Float(0.0), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1065.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_q = t1065[0].clone();
    v_queue = t1065[1].clone();
    v_served = t1065[2].clone();
    for t1066 in py::iter(&(py::zip(&[v_a.clone(), v_c.clone()])?))? {
        let t1067 = (t1066).items()?;
        if t1067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1067[0].clone();
        v_y = t1067[1].clone();
        v_nq = py::minmax(&[Py::Float(0.0), py::sub(&py::add(&v_q.clone(), &v_x.clone())?, &v_y.clone())?], true)?;
        let _ = py::method(&v_served.clone(), "append", &[py::sub(&py::add(&v_q.clone(), &v_x.clone())?, &v_nq.clone())?])?;
        v_q = v_nq.clone();
        let _ = py::method(&v_queue.clone(), "append", &[v_q.clone()])?;
    }
    return Ok(Py::dict(vec![(Py::str("queue"), f__r(v_queue.clone())?), (Py::str("Q_max"), f__r(py::minmax(&[v_queue.clone()], true)?)?), (Py::str("served"), f__r(v_served.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lateral_transfer(mut v_m: Py, mut v_a_y: Py, mut v_h: Py, mut v_t_w: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_dW: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("a_y"), v_a_y.clone()), (Py::str("h"), v_h.clone()), (Py::str("t_w"), v_t_w.clone())]))?;
    v_W = py::mul(&v_m.clone(), &c_G_()?)?;
    v_dW = py::truediv(&py::mul(&py::mul(&v_m.clone(), &v_a_y.clone())?, &v_h.clone())?, &v_t_w.clone())?;
    let _ = f__need({ let t1086 = py::sub(&py::truediv(&v_W.clone(), &Py::Int(2i64))?, &v_dW.clone())?; Py::Bool({ let t1087 = Py::Int(0i64); if py::ge(&t1086, &t1087)? { true } else { false } }) }, Py::str("the inner wheels would lift off (rollover) at this lateral acceleration"))?;
    return Ok(Py::dict(vec![(Py::str("W_out"), f__r(py::add(&py::truediv(&v_W.clone(), &Py::Int(2i64))?, &v_dW.clone())?)?), (Py::str("W_in"), f__r(f__snap(py::sub(&py::truediv(&v_W.clone(), &Py::Int(2i64))?, &v_dW.clone())?, v_W.clone())?)?), (Py::str("dW"), f__r(v_dW.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logit_attributes(mut v_X: Py, mut v_beta: Py, mut v_asc: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v__ls: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__mat(Py::str("X"), v_X.clone(), Py::None, Py::None, Py::Bool(false), Py::Bool(false))?;
    v_b = f__vec(Py::str("beta"), v_beta.clone(), py::len(&py::getitem(&v_x.clone(), &Py::Int(0i64))?)?, Py::Bool(false), Py::Bool(false))?;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t1168 = v_n.clone(); Py::Bool({ let t1169 = Py::Int(2i64); if py::ge(&t1168, &t1169)? { true } else { false } }) }, Py::str("give at least two alternatives (rows)"))?;
    v_a = (if ({ let t1170 = v_asc.clone(); Py::Bool({ let t1171 = Py::None; if !py::is(&t1170, &t1171) { true } else { false } }) }).truthy() { f__vec(Py::str("asc"), v_asc.clone(), v_n.clone(), Py::Bool(false), Py::Bool(false))? } else { py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())? });
    v_V = { let mut out = vec![]; for t1172 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1172;
out.push(py::add(&py::getitem(&v_a.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t1173 in py::range_iter(&[(py::len(&v_b.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1173;
out.push(py::mul(&py::getitem(&py::getitem(&v_x.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_b.clone(), &v_k.clone())?)?); } Py::list(out) })?)?); } Py::list(out) };
    let t1176 = (f__softmax(v_V.clone())?).items()?;
    if t1176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1176[0].clone();
    v__ls = t1176[1].clone();
    return Ok(Py::dict(vec![(Py::str("V"), f__r(v_V.clone())?), (Py::str("P"), f__r(v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logit_elasticities(mut v_V: Py, mut v_x: Py, mut v_beta: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v__ls: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_v = f__vec(Py::str("utilities"), v_V.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    v_xs = f__vec(Py::str("x"), v_x.clone(), py::len(&v_v.clone())?, Py::Bool(false), Py::Bool(false))?;
    let _ = f__finite(Py::dict(vec![(Py::str("beta"), v_beta.clone())]))?;
    let _ = f__need({ let t1118 = py::len(&v_v.clone())?; Py::Bool({ let t1119 = Py::Int(2i64); if py::ge(&t1118, &t1119)? { true } else { false } }) }, Py::str("give at least two alternatives"))?;
    let t1122 = (f__softmax(v_v.clone())?).items()?;
    if t1122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1122[0].clone();
    v__ls = t1122[1].clone();
    return Ok(Py::dict(vec![(Py::str("P"), f__r(v_P.clone())?), (Py::str("direct"), f__r({ let mut out = vec![]; for t1132 in py::iter(&(py::zip(&[v_xs.clone(), v_P.clone()])?))? { let mut v_p: Py; let mut v_xi: Py; 
let t1133 = (t1132).items()?;
if t1133.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_xi = t1133[0].clone();
v_p = t1133[1].clone();
out.push(f__snap(py::mul(&py::mul(&v_beta.clone(), &v_xi.clone())?, &py::sub(&Py::Int(1i64), &v_p.clone())?)?, Py::Float(1.0))?); } Py::list(out) })?), (Py::str("cross"), f__r({ let mut out = vec![]; for t1136 in py::iter(&(py::zip(&[v_xs.clone(), v_P.clone()])?))? { let mut v_p: Py; let mut v_xi: Py; 
let t1137 = (t1136).items()?;
if t1137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_xi = t1137[0].clone();
v_p = t1137[1].clone();
out.push(f__snap(py::mul(&py::mul(&py::neg(&v_beta.clone())?, &v_xi.clone())?, &v_p.clone())?, Py::Float(1.0))?); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logit_shares(mut v_V: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_ls: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = f__vec(Py::str("utilities"), v_V.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t1201 = py::len(&v_v.clone())?; Py::Bool({ let t1202 = Py::Int(2i64); if py::ge(&t1201, &t1202)? { true } else { false } }) }, Py::str("give the utilities of at least two alternatives"))?;
    let t1205 = (f__softmax(v_v.clone())?).items()?;
    if t1205.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1205[0].clone();
    v_ls = t1205[1].clone();
    return Ok(Py::dict(vec![(Py::str("P"), f__r(v_P.clone())?), (Py::str("logsum"), f__r(v_ls.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logsum_benefit(mut v_V0: Py, mut v_V1: Py, mut v_beta_cost: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_l0: Py = Py::None;
    let mut v_l1: Py = Py::None;
    v_a = f__vec(Py::str("V0"), v_V0.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    v_b = f__vec(Py::str("V1"), v_V1.clone(), py::len(&v_a.clone())?, Py::Bool(false), Py::Bool(false))?;
    let _ = f__finite(Py::dict(vec![(Py::str("beta_cost"), v_beta_cost.clone())]))?;
    let _ = f__need({ let t1234 = v_beta_cost.clone(); Py::Bool({ let t1235 = Py::Int(0i64); if py::lt(&t1234, &t1235)? { true } else { false } }) }, Py::str("the cost coefficient must be negative (utility falls as cost rises)"))?;
    let t1238 = (Py::tuple(vec![py::getitem(&f__softmax(v_a.clone())?, &Py::Int(1i64))?, py::getitem(&f__softmax(v_b.clone())?, &Py::Int(1i64))?])).items()?;
    if t1238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l0 = t1238[0].clone();
    v_l1 = t1238[1].clone();
    return Ok(Py::dict(vec![(Py::str("dCS"), f__r(f__snap(py::truediv(&py::sub(&v_l1.clone(), &v_l0.clone())?, &py::neg(&v_beta_cost.clone())?)?, py::truediv(&py::minmax(&[py::abs(&v_l1.clone())?, py::abs(&v_l0.clone())?, Py::Float(1.0)], true)?, &py::neg(&v_beta_cost.clone())?)?)?)?), (Py::str("logsum0"), f__r(v_l0.clone())?), (Py::str("logsum1"), f__r(v_l1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_md1(mut v_lam: Py, mut v_mu: Py) -> R {
    let mut v_Lq: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("mu"), v_mu.clone())]))?;
    let _ = f__need({ let t1259 = v_lam.clone(); Py::Bool({ let t1260 = v_mu.clone(); if py::lt(&t1259, &t1260)? { true } else { false } }) }, Py::str("the arrival rate must be below the service rate (rho < 1)"))?;
    v_rho = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_Lq = py::truediv(&py::mul(&v_rho.clone(), &v_rho.clone())?, &py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Lq"), f__r(v_Lq.clone())?), (Py::str("Wq"), f__r(py::truediv(&v_rho.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?)?), (Py::str("W"), f__r(py::truediv(&py::sub(&Py::Int(2i64), &v_rho.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_mu.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?)?), (Py::str("L"), f__r(py::add(&v_Lq.clone(), &v_rho.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_speeds(mut v_speeds: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_sms: Py = Py::None;
    let mut v_tms: Py = Py::None;
    v_s = f__speeds(v_speeds.clone())?;
    v_n = py::len(&v_s.clone())?;
    v_tms = py::truediv(&py::fsum(&v_s.clone())?, &v_n.clone())?;
    v_sms = py::truediv(&v_n.clone(), &py::fsum(&{ let mut out = vec![]; for t2047 in py::iter(&(v_s.clone()))? { let mut v_x: Py; 
v_x = t2047;
out.push(py::truediv(&Py::Int(1i64), &v_x.clone())?); } Py::list(out) })?)?;
    v_sd = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t2048 in py::iter(&(v_s.clone()))? { let mut v_x: Py; 
v_x = t2048;
out.push(py::pow(&py::sub(&v_x.clone(), &v_tms.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    return Ok(Py::dict(vec![(Py::str("TMS"), f__r(v_tms.clone())?), (Py::str("SMS"), f__r(v_sms.clone())?), (Py::str("s"), f__r(f__snap(v_sd.clone(), v_tms.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mm1(mut v_lam: Py, mut v_mu: Py) -> R {
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("mu"), v_mu.clone())]))?;
    let _ = f__need({ let t1304 = v_lam.clone(); Py::Bool({ let t1305 = v_mu.clone(); if py::lt(&t1304, &t1305)? { true } else { false } }) }, Py::str("the arrival rate must be below the service rate (rho < 1)"))?;
    v_rho = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    return Ok(Py::dict(vec![(Py::str("rho"), f__r(v_rho.clone())?), (Py::str("L"), f__r(py::truediv(&v_rho.clone(), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?), (Py::str("Lq"), f__r(py::truediv(&py::mul(&v_rho.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?), (Py::str("W"), f__r(py::truediv(&Py::Int(1i64), &py::sub(&v_mu.clone(), &v_lam.clone())?)?)?), (Py::str("Wq"), f__r(py::truediv(&v_rho.clone(), &py::sub(&v_mu.clone(), &v_lam.clone())?)?)?), (Py::str("P0"), f__r(py::sub(&Py::Int(1i64), &v_rho.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mmc(mut v_lam: Py, mut v_mu: Py, mut v_c: Py) -> R {
    let mut v_Lq: Py = Py::None;
    let mut v_Pw: Py = Py::None;
    let mut v_Wq: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("mu"), v_mu.clone())]))?;
    let _ = f__need({ let t1327 = Py::Bool(matches!(v_c.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1327.truthy() { t1327 } else { { let t1324 = Py::Int(1i64); Py::Bool({ let t1325 = v_c.clone(); if py::le(&t1324, &t1325)? { { let t1326 = Py::Int(200i64); if py::le(&t1325, &t1326)? { true } else { false } } } else { false } }) } } }, Py::str("the number of servers must be a whole number from 1 to 200"))?;
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_rho = py::truediv(&v_a.clone(), &v_c.clone())?;
    let _ = f__need({ let t1328 = v_rho.clone(); Py::Bool({ let t1329 = Py::Int(1i64); if py::lt(&t1328, &t1329)? { true } else { false } }) }, Py::str("the arrival rate must be below c mu (rho < 1)"))?;
    let t1330 = (Py::tuple(vec![Py::Float(1.0), Py::Float(1.0)])).items()?;
    if t1330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term = t1330[0].clone();
    v_s = t1330[1].clone();
    for t1331 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_c.clone()).i()?]) {
        v_k = t1331;
        v_term = py::mul(&v_term, &py::truediv(&v_a.clone(), &v_k.clone())?)?;
        v_s = py::iadd(&v_s, &v_term.clone())?;
    }
    v_last = py::truediv(&py::mul(&v_term.clone(), &v_a.clone())?, &v_c.clone())?;
    v_Pw = py::truediv(&py::truediv(&v_last.clone(), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?, &py::add(&v_s.clone(), &py::truediv(&v_last.clone(), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?)?;
    v_Lq = py::truediv(&py::mul(&v_Pw.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?;
    v_Wq = py::truediv(&v_Lq.clone(), &v_lam.clone())?;
    return Ok(Py::dict(vec![(Py::str("P_wait"), f__r(v_Pw.clone())?), (Py::str("Lq"), f__r(v_Lq.clone())?), (Py::str("Wq"), f__r(v_Wq.clone())?), (Py::str("W"), f__r(py::add(&v_Wq.clone(), &py::truediv(&Py::Int(1i64), &v_mu.clone())?)?)?), (Py::str("L"), f__r(py::add(&v_Lq.clone(), &v_a.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nested_logit(mut v_V: Py, mut v_nest: Py, mut v_lam: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_PN: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_v = f__vec(Py::str("utilities"), v_V.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t1356 = py::len(&v_v.clone())?; Py::Bool({ let t1357 = Py::Int(2i64); if py::ge(&t1356, &t1357)? { true } else { false } }) }, Py::str("give at least two alternatives"))?;
    let t1371 = (f__nests(v_nest.clone(), v_lam.clone(), py::len(&v_v.clone())?)?).items()?;
    if t1371.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1371[0].clone();
    v_lm = t1371[1].clone();
    v_K = t1371[2].clone();
    v_I = Py::list(vec![]);
    for t1372 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t1372;
        v_vals = { let mut out = vec![]; for t1373 in py::range_iter(&[(py::len(&v_v.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1373;
if ({ let t1374 = py::getitem(&v_k.clone(), &v_i.clone())?; Py::Bool({ let t1375 = v_m.clone(); if py::eq(&t1374, &t1375) { true } else { false } }) }).truthy() { out.push(py::truediv(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&v_lm.clone(), &py::sub(&v_m.clone(), &Py::Int(1i64))?)?)?); }} Py::list(out) };
        v_mx = py::minmax(&[v_vals.clone()], true)?;
        let _ = py::method(&v_I.clone(), "append", &[py::add(&v_mx.clone(), &py::math1("log", &py::fsum(&{ let mut out = vec![]; for t1376 in py::iter(&(v_vals.clone()))? { let mut v_x: Py; 
v_x = t1376;
out.push(py::math1("exp", &py::sub(&v_x.clone(), &v_mx.clone())?)?); } Py::list(out) })?)?)?])?;
    }
    let t1380 = (f__softmax({ let mut out = vec![]; for t1379 in py::range_iter(&[(v_K.clone()).i()?]) { let mut v_m: Py; 
v_m = t1379;
out.push(py::mul(&py::getitem(&v_lm.clone(), &v_m.clone())?, &py::getitem(&v_I.clone(), &v_m.clone())?)?); } Py::list(out) })?).items()?;
    if t1380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_PN = t1380[0].clone();
    v__ = t1380[1].clone();
    v_P = { let mut out = vec![]; for t1381 in py::range_iter(&[(py::len(&v_v.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1381;
out.push(py::mul(&py::math1("exp", &py::sub(&py::truediv(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&v_lm.clone(), &py::sub(&py::getitem(&v_k.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?, &py::getitem(&v_I.clone(), &py::sub(&py::getitem(&v_k.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?)?, &py::getitem(&v_PN.clone(), &py::sub(&py::getitem(&v_k.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("P"), f__r(v_P.clone())?), (Py::str("P_nest"), f__r(v_PN.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_overtaking(mut v_v: Py, mut v_a: Py, mut v_s: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__nonneg(Py::dict(vec![(Py::str("v"), v_v.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("a"), v_a.clone()), (Py::str("s"), v_s.clone())]))?;
    v_t = py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(2i64), &v_s.clone())?, &v_a.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("t"), f__r(v_t.clone())?), (Py::str("d"), f__r(py::add(&py::mul(&v_v.clone(), &v_t.clone())?, &py::truediv(&py::mul(&py::mul(&v_a.clone(), &v_t.clone())?, &v_t.clone())?, &Py::Int(2i64))?)?)?), (Py::str("v_f"), f__r(py::add(&v_v.clone(), &py::mul(&v_a.clone(), &v_t.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_passing_sight(mut v_v: Py, mut v_m_diff: Py, mut v_a: Py, mut v_t1: Py, mut v_t2: Py, mut v_d3: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_d4: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("a"), v_a.clone()), (Py::str("t1"), v_t1.clone()), (Py::str("t2"), v_t2.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("m_diff"), v_m_diff.clone()), (Py::str("d3"), v_d3.clone())]))?;
    let _ = f__need({ let t1423 = Py::Int(0i64); Py::Bool({ let t1424 = v_m_diff.clone(); if py::lt(&t1423, &t1424)? { { let t1425 = v_v.clone(); if py::lt(&t1424, &t1425)? { true } else { false } } } else { false } }) }, Py::str("the speed difference must be positive and below the passing speed"))?;
    let _ = f__need({ let t1426 = v_d3.clone(); Py::Bool({ let t1427 = Py::Int(0i64); if py::ge(&t1426, &t1427)? { true } else { false } }) }, Py::str("the clearance distance must be zero or positive"))?;
    v_d1 = py::mul(&v_t1.clone(), &py::add(&py::sub(&v_v.clone(), &v_m_diff.clone())?, &py::truediv(&py::mul(&v_a.clone(), &v_t1.clone())?, &Py::Int(2i64))?)?)?;
    v_d2 = py::mul(&v_v.clone(), &v_t2.clone())?;
    v_d4 = py::truediv(&py::mul(&Py::Int(2i64), &v_d2.clone())?, &Py::Int(3i64))?;
    return Ok(Py::dict(vec![(Py::str("PSD"), f__r(py::add(&py::add(&py::add(&v_d1.clone(), &v_d2.clone())?, &v_d3.clone())?, &v_d4.clone())?)?), (Py::str("d1"), f__r(v_d1.clone())?), (Py::str("d2"), f__r(v_d2.clone())?), (Py::str("d3"), f__r(v_d3.clone())?), (Py::str("d4"), f__r(v_d4.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_peak_hour_factor(mut v_counts: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_peak: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_start: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t1440 = Py::Bool(matches!(v_counts.clone(), Py::List(_) | Py::Tuple(_))); if !t1440.truthy() { t1440 } else { { let t1437 = Py::Int(4i64); Py::Bool({ let t1438 = py::len(&v_counts.clone())?; if py::le(&t1437, &t1438)? { { let t1439 = Py::Int(96i64); if py::le(&t1438, &t1439)? { true } else { false } } } else { false } }) } } }, Py::str("give between 4 and 96 consecutive 15-minute counts"))?;
    v_c = Py::list(vec![]);
    for t1441 in py::iter(&(v_counts.clone()))? {
        v_x = t1441;
        let _ = f__need({ let t1446 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1446.truthy() { t1446 } else { { let t1445 = py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))); if !t1445.truthy() { t1445 } else { { let t1444 = py::math1("isfinite", &v_x.clone())?; if !t1444.truthy() { t1444 } else { { let t1442 = v_x.clone(); Py::Bool({ let t1443 = Py::Int(0i64); if py::ge(&t1442, &t1443)? { true } else { false } }) } } } } } } }, Py::str("every 15-minute count must be zero or positive"))?;
        let _ = py::method(&v_c.clone(), "append", &[py::float(&v_x.clone())?])?;
    }
    v_best = py::neg(&Py::Float(1.0))?;
    v_start = Py::Int(0i64);
    for t1447 in py::range_iter(&[(py::sub(&py::len(&v_c.clone())?, &Py::Int(3i64))?).i()?]) {
        v_i = t1447;
        v_s = py::add(&py::add(&py::add(&py::getitem(&v_c.clone(), &v_i.clone())?, &py::getitem(&v_c.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::getitem(&v_c.clone(), &py::add(&v_i.clone(), &Py::Int(2i64))?)?)?, &py::getitem(&v_c.clone(), &py::add(&v_i.clone(), &Py::Int(3i64))?)?)?;
        if ({ let t1448 = v_s.clone(); Py::Bool({ let t1449 = v_best.clone(); if py::gt(&t1448, &t1449)? { true } else { false } }) }).truthy() {
            v_best = v_s.clone();
            v_start = v_i.clone();
        }
    }
    let _ = f__need({ let t1450 = v_best.clone(); Py::Bool({ let t1451 = Py::Int(0i64); if py::gt(&t1450, &t1451)? { true } else { false } }) }, Py::str("the peak hour carries no vehicles"))?;
    v_peak = py::minmax(&[py::getitem(&v_c.clone(), &v_start.clone())?, py::getitem(&v_c.clone(), &py::add(&v_start.clone(), &Py::Int(1i64))?)?, py::getitem(&v_c.clone(), &py::add(&v_start.clone(), &Py::Int(2i64))?)?, py::getitem(&v_c.clone(), &py::add(&v_start.clone(), &Py::Int(3i64))?)?], true)?;
    return Ok(Py::dict(vec![(Py::str("V"), f__r(v_best.clone())?), (Py::str("V15"), f__r(v_peak.clone())?), (Py::str("v15"), f__r(py::mul(&Py::Int(4i64), &v_peak.clone())?)?), (Py::str("PHF"), f__r(py::truediv(&v_best.clone(), &py::mul(&Py::Int(4i64), &v_peak.clone())?)?)?), (Py::str("start"), py::add(&v_start.clone(), &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poisson_arrivals(mut v_q: Py, mut v_t: Py, mut v_n: Py) -> R {
    let mut v_gt: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_le: Py = Py::None;
    let mut v_p_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t1472 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1472.truthy() { t1472 } else { { let t1470 = v_n.clone(); Py::Bool({ let t1471 = Py::Int(0i64); if py::ge(&t1470, &t1471)? { true } else { false } }) } } }, Py::str("n must be a whole number >= 0"))?;
    v_lam = py::mul(&v_q.clone(), &v_t.clone())?;
    let _ = f__need({ let t1473 = v_lam.clone(); Py::Bool({ let t1474 = Py::Int(1000i64); if py::le(&t1473, &t1474)? { true } else { false } }) }, Py::str("the expected count q t must be at most 1000"))?;
    let t1484 = (f__pois_terms(v_lam.clone(), v_n.clone())?).items()?;
    if t1484.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p_n = t1484[0].clone();
    v_le = t1484[1].clone();
    v_gt = t1484[2].clone();
    return Ok(Py::dict(vec![(Py::str("P_n"), f__r(v_p_n.clone())?), (Py::str("P_le"), f__r(v_le.clone())?), (Py::str("P_ge"), f__r(py::add(&v_p_n.clone(), &v_gt.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_accel(mut v_m: Py, mut v_P: Py, mut v_v1: Py, mut v_v2: Py, mut v_C_D: Py, mut v_A: Py, mut v_rho: Py, mut v_f_r: Py, mut v_G: Py, mut v_eta: Py, mut v_gamma: Py) -> R {
    let mut v_F0: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_net: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_veq: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("P"), v_P.clone()), (Py::str("v1"), v_v1.clone()), (Py::str("v2"), v_v2.clone()), (Py::str("C_D"), v_C_D.clone()), (Py::str("A"), v_A.clone()), (Py::str("rho"), v_rho.clone()), (Py::str("eta"), v_eta.clone()), (Py::str("gamma"), v_gamma.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("f_r"), v_f_r.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t40 = v_eta.clone(); Py::Bool({ let t41 = Py::Int(1i64); if py::le(&t40, &t41)? { true } else { false } }) }, Py::str("the drivetrain efficiency must not exceed 1"))?;
    let _ = f__need({ let t42 = v_v1.clone(); Py::Bool({ let t43 = v_v2.clone(); if py::ne(&t42, &t43) { true } else { false } }) }, Py::str("the two speeds must differ"))?;
    let t68 = (f__power_balance(v_m.clone(), v_P.clone(), v_C_D.clone(), v_A.clone(), v_rho.clone(), v_f_r.clone(), v_G.clone(), v_eta.clone())?).items()?;
    if t68.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K = t68[0].clone();
    v_F0 = t68[1].clone();
    v_veq = t68[2].clone();
    let t69 = (Py::tuple(vec![py::minmax(&[v_v1.clone(), v_v2.clone()], false)?, py::minmax(&[v_v1.clone(), v_v2.clone()], true)?])).items()?;
    if t69.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t69[0].clone();
    v_hi = t69[1].clone();
    let _ = f__need({ let t80 = { let t74 = { let t70 = v_v1.clone(); Py::Bool({ let t71 = v_v2.clone(); if py::lt(&t70, &t71)? { true } else { false } }) }; if !t74.truthy() { t74 } else { { let t72 = v_hi.clone(); Py::Bool({ let t73 = py::mul(&v_veq.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::lt(&t72, &t73)? { true } else { false } }) } } }; if t80.truthy() { t80 } else { { let t79 = { let t75 = v_v1.clone(); Py::Bool({ let t76 = v_v2.clone(); if py::gt(&t75, &t76)? { true } else { false } }) }; if !t79.truthy() { t79 } else { { let t77 = v_lo.clone(); Py::Bool({ let t78 = py::mul(&v_veq.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t77, &t78)? { true } else { false } }) } } } } }, Py::str(&["the vehicle cannot go from v1 to v2: its equilibrium speed under this power is ".to_string(), py::format(&v_veq.clone(), ".6g")?, " m/s".to_string()].concat()))?;
    v_net = { let v_A = v_A.clone(); let v_C_D = v_C_D.clone(); let v_F0 = v_F0.clone(); let v_G = v_G.clone(); let v_K = v_K.clone(); let v_P = v_P.clone(); let v_eta = v_eta.clone(); let v_f_r = v_f_r.clone(); let v_gamma = v_gamma.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_rho = v_rho.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_veq = v_veq.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::truediv(&py::mul(&v_eta.clone(), &v_P.clone())?, &v_v.clone())?, &v_F0.clone())?, &py::mul(&py::mul(&v_K.clone(), &v_v.clone())?, &v_v.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(25i64))?;
        v_t = ax_engine::pymp::call("quad", &[{ let v_A = v_A.clone(); let v_C_D = v_C_D.clone(); let v_F0 = v_F0.clone(); let v_G = v_G.clone(); let v_K = v_K.clone(); let v_P = v_P.clone(); let v_eta = v_eta.clone(); let v_f_r = v_f_r.clone(); let v_gamma = v_gamma.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_net = v_net.clone(); let v_rho = v_rho.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_veq = v_veq.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&v_gamma.clone(), &v_m.clone())?, &py::call(&v_net.clone(), &[v_v.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![v_v1.clone(), v_v2.clone()])], &[])?;
        v_d = ax_engine::pymp::call("quad", &[{ let v_A = v_A.clone(); let v_C_D = v_C_D.clone(); let v_F0 = v_F0.clone(); let v_G = v_G.clone(); let v_K = v_K.clone(); let v_P = v_P.clone(); let v_eta = v_eta.clone(); let v_f_r = v_f_r.clone(); let v_gamma = v_gamma.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_m = v_m.clone(); let v_net = v_net.clone(); let v_rho = v_rho.clone(); let v_t = v_t.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_veq = v_veq.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&py::mul(&v_gamma.clone(), &v_m.clone())?, &v_v.clone())?, &py::call(&v_net.clone(), &[v_v.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![v_v1.clone(), v_v2.clone()])], &[])?;
    }
    return Ok(Py::dict(vec![(Py::str("t"), f__r(py::float(&v_t.clone())?)?), (Py::str("d"), f__r(py::float(&v_d.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quarter_car_freq(mut v_m_s: Py, mut v_m_u: Py, mut v_k_s: Py, mut v_k_t: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m_s"), v_m_s.clone()), (Py::str("m_u"), v_m_u.clone()), (Py::str("k_s"), v_k_s.clone()), (Py::str("k_t"), v_k_t.clone())]))?;
    let t1503 = (Py::tuple(vec![py::mul(&v_m_s.clone(), &v_m_u.clone())?, py::neg(&py::add(&py::mul(&v_k_s.clone(), &v_m_u.clone())?, &py::mul(&py::add(&v_k_s.clone(), &v_k_t.clone())?, &v_m_s.clone())?)?)?, py::mul(&v_k_s.clone(), &v_k_t.clone())?])).items()?;
    if t1503.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1503[0].clone();
    v_b = t1503[1].clone();
    v_c = t1503[2].clone();
    v_disc = py::math1("sqrt", &py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?)?)?;
    v_hi = py::truediv(&py::add(&py::neg(&v_b.clone())?, &v_disc.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?;
    v_lo = py::truediv(&v_c.clone(), &py::mul(&v_a.clone(), &v_hi.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("f_body"), f__r(py::truediv(&py::math1("sqrt", &v_lo.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?), (Py::str("f_wheel"), f__r(py::truediv(&py::math1("sqrt", &v_hi.clone())?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quarter_car_tr(mut v_m_s: Py, mut v_m_u: Py, mut v_k_s: Py, mut v_c_s: Py, mut v_k_t: Py, mut v_f: Py) -> R {
    let mut v_a11: Py = Py::None;
    let mut v_a12: Py = Py::None;
    let mut v_a22: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_xu: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m_s"), v_m_s.clone()), (Py::str("m_u"), v_m_u.clone()), (Py::str("k_s"), v_k_s.clone()), (Py::str("k_t"), v_k_t.clone()), (Py::str("f"), v_f.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("c_s"), v_c_s.clone())]))?;
    v_w = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_z = py::complex(&[v_k_s.clone(), py::mul(&v_w.clone(), &v_c_s.clone())?])?;
    let t1527 = (Py::tuple(vec![py::sub(&v_z.clone(), &py::mul(&py::mul(&v_w.clone(), &v_w.clone())?, &v_m_s.clone())?)?, py::neg(&v_z.clone())?, py::sub(&py::add(&v_z.clone(), &v_k_t.clone())?, &py::mul(&py::mul(&v_w.clone(), &v_w.clone())?, &v_m_u.clone())?)?])).items()?;
    if t1527.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a11 = t1527[0].clone();
    v_a12 = t1527[1].clone();
    v_a22 = t1527[2].clone();
    v_det = py::sub(&py::mul(&v_a11.clone(), &v_a22.clone())?, &py::mul(&v_a12.clone(), &v_a12.clone())?)?;
    let _ = f__need({ let t1528 = py::abs(&v_det.clone())?; Py::Bool({ let t1529 = Py::Int(0i64); if py::gt(&t1528, &t1529)? { true } else { false } }) }, Py::str("the frequency is exactly at an undamped resonance"))?;
    v_xs = py::truediv(&py::mul(&py::neg(&v_a12.clone())?, &v_k_t.clone())?, &v_det.clone())?;
    v_xu = py::truediv(&py::mul(&v_a11.clone(), &v_k_t.clone())?, &v_det.clone())?;
    return Ok(Py::dict(vec![(Py::str("T_body"), f__r(py::abs(&v_xs.clone())?)?), (Py::str("T_wheel"), f__r(py::abs(&v_xu.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reliability(mut v_times: Py, mut v_t_free: Py) -> R {
    let mut v_mean: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_t95: Py = Py::None;
    let _ = f__need({ let t2223 = Py::Bool(matches!(v_times.clone(), Py::List(_) | Py::Tuple(_))); if !t2223.truthy() { t2223 } else { { let t2221 = py::len(&v_times.clone())?; Py::Bool({ let t2222 = Py::Int(5i64); if py::ge(&t2221, &t2222)? { true } else { false } }) } } }, Py::str("give at least five travel times"))?;
    for t2224 in py::iter(&(v_times.clone()))? {
        v_t = t2224;
        let _ = f__need({ let t2227 = Py::Bool(matches!(v_t.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2227.truthy() { t2227 } else { { let t2225 = v_t.clone(); Py::Bool({ let t2226 = Py::Int(0i64); if py::gt(&t2225, &t2226)? { true } else { false } }) } } }, Py::str("every travel time must be positive"))?;
    }
    let _ = f__pos(Py::dict(vec![(Py::str("t_free"), v_t_free.clone())]))?;
    v_mean = py::truediv(&py::sum(&v_times.clone(), None)?, &py::len(&v_times.clone())?)?;
    v_t95 = f__pctl(v_times.clone(), Py::Int(95i64))?;
    return Ok(Py::dict(vec![(Py::str("mean"), f__r(v_mean.clone())?), (Py::str("t95"), f__r(v_t95.clone())?), (Py::str("buffer_index"), f__r(py::truediv(&py::sub(&v_t95.clone(), &v_mean.clone())?, &v_mean.clone())?)?), (Py::str("planning_time_index"), f__r(py::truediv(&v_t95.clone(), &v_t_free.clone())?)?), (Py::str("travel_time_index"), f__r(py::truediv(&v_mean.clone(), &v_t_free.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rollover(mut v_t_w: Py, mut v_h: Py, mut v_R: Py, mut v_e: Py) -> R {
    let mut v_ssf: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("t_w"), v_t_w.clone()), (Py::str("h"), v_h.clone()), (Py::str("R"), v_R.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("e"), v_e.clone())]))?;
    let _ = f__need({ let t1573 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t1574 = v_e.clone(); if py::lt(&t1573, &t1574)? { { let t1575 = Py::Int(1i64); if py::lt(&t1574, &t1575)? { true } else { false } } } else { false } }) }, Py::str("the superelevation rate must lie between -1 and 1"))?;
    v_ssf = py::truediv(&v_t_w.clone(), &py::mul(&Py::Int(2i64), &v_h.clone())?)?;
    let _ = f__need({ let t1576 = py::mul(&v_ssf.clone(), &v_e.clone())?; Py::Bool({ let t1577 = Py::Int(1i64); if py::lt(&t1576, &t1577)? { true } else { false } }) }, Py::str("this vehicle cannot roll over on this bank (SSF e >= 1)"))?;
    let _ = f__need({ let t1578 = py::add(&v_ssf.clone(), &v_e.clone())?; Py::Bool({ let t1579 = Py::Int(0i64); if py::gt(&t1578, &t1579)? { true } else { false } }) }, Py::str("the bank is so steep the vehicle tips inward at rest"))?;
    v_v = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&c_G_()?, &v_R.clone())?, &py::add(&v_ssf.clone(), &v_e.clone())?)?, &py::sub(&Py::Int(1i64), &py::mul(&v_ssf.clone(), &v_e.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("SSF"), f__r(v_ssf.clone())?), (Py::str("v_roll"), f__r(v_v.clone())?), (Py::str("v_roll_kmh"), f__kmh(v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_running_time(mut v_D: Py, mut v_v_max: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_da: Py = Py::None;
    let mut v_db: Py = Py::None;
    let mut v_vp: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("v_max"), v_v_max.clone()), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    let t2209 = (Py::tuple(vec![py::truediv(&py::mul(&v_v_max.clone(), &v_v_max.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?, py::truediv(&py::mul(&v_v_max.clone(), &v_v_max.clone())?, &py::mul(&Py::Int(2i64), &v_b.clone())?)?])).items()?;
    if t2209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_da = t2209[0].clone();
    v_db = t2209[1].clone();
    if ({ let t2210 = py::add(&v_da.clone(), &v_db.clone())?; Py::Bool({ let t2211 = v_D.clone(); if py::le(&t2210, &t2211)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("T"), f__r(py::add(&py::add(&py::truediv(&v_D.clone(), &v_v_max.clone())?, &py::truediv(&v_v_max.clone(), &py::mul(&Py::Int(2i64), &v_a.clone())?)?)?, &py::truediv(&v_v_max.clone(), &py::mul(&Py::Int(2i64), &v_b.clone())?)?)?)?), (Py::str("v_peak"), f__r(v_v_max.clone())?), (Py::str("profile"), Py::str("accelerate-cruise-brake"))]));
    }
    v_vp = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_a.clone())?, &v_b.clone())?, &py::add(&v_a.clone(), &v_b.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("T"), f__r(py::add(&py::truediv(&v_vp.clone(), &v_a.clone())?, &py::truediv(&v_vp.clone(), &v_b.clone())?)?)?), (Py::str("v_peak"), f__r(v_vp.clone())?), (Py::str("profile"), Py::str("accelerate-brake"))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sag_headlight(mut v_g1: Py, mut v_g2: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_Ls: Py = Py::None;
    let mut v_case: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("g1"), v_g1.clone()), (Py::str("g2"), v_g2.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("S"), v_S.clone())]))?;
    v_A = py::mul(&py::sub(&v_g2.clone(), &v_g1.clone())?, &Py::Float(100.0))?;
    let _ = f__need({ let t1625 = v_A.clone(); Py::Bool({ let t1626 = Py::Int(0i64); if py::gt(&t1625, &t1626)? { true } else { false } }) }, Py::str("a sag curve needs g2 > g1 (the grade increases)"))?;
    let _ = f__need({ let t1627 = v_S.clone(); Py::Bool({ let t1628 = Py::Int(2000i64); if py::le(&t1627, &t1628)? { true } else { false } }) }, Py::str("the sight distance must be at most 2000 m"))?;
    v_Ls = py::truediv(&py::mul(&py::mul(&v_A.clone(), &v_S.clone())?, &v_S.clone())?, &py::add(&Py::Int(120i64), &py::mul(&Py::Float(3.5), &v_S.clone())?)?)?;
    if ({ let t1629 = v_Ls.clone(); Py::Bool({ let t1630 = v_S.clone(); if py::ge(&t1629, &t1630)? { true } else { false } }) }).truthy() {
        let t1631 = (Py::tuple(vec![v_Ls.clone(), Py::str("S < L")])).items()?;
        if t1631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_L = t1631[0].clone();
        v_case = t1631[1].clone();
    } else {
        let t1632 = (Py::tuple(vec![py::minmax(&[Py::Float(0.0), py::sub(&py::mul(&Py::Int(2i64), &v_S.clone())?, &py::truediv(&py::add(&Py::Int(120i64), &py::mul(&Py::Float(3.5), &v_S.clone())?)?, &v_A.clone())?)?], true)?, Py::str("S > L")])).items()?;
        if t1632.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_L = t1632[0].clone();
        v_case = t1632[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("L"), f__r(v_L.clone())?), (Py::str("A"), f__r(v_A.clone())?), (Py::str("K"), f__r(py::truediv(&v_L.clone(), &v_A.clone())?)?), (Py::str("case"), v_case.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shockwave(mut v_q1: Py, mut v_k1: Py, mut v_q2: Py, mut v_k2: Py) -> R {
    let mut v_u: Py = Py::None;
    let _ = f__wave_check(v_q1.clone(), v_k1.clone(), v_q2.clone(), v_k2.clone())?;
    v_u = py::truediv(&py::sub(&v_q2.clone(), &v_q1.clone())?, &py::sub(&v_k2.clone(), &v_k1.clone())?)?;
    v_u = f__snap(v_u.clone(), py::truediv(&py::minmax(&[v_q1.clone(), v_q2.clone()], true)?, &py::minmax(&[v_k1.clone(), v_k2.clone()], false)?)?)?;
    return Ok(Py::dict(vec![(Py::str("u"), f__kmh(v_u.clone())?), (Py::str("u_ms"), f__r(v_u.clone())?), (Py::str("direction"), (if ({ let t1675 = v_u.clone(); Py::Bool({ let t1676 = Py::Int(0i64); if py::eq(&t1675, &t1676) { true } else { false } }) }).truthy() { Py::str("stationary") } else { (if ({ let t1677 = v_u.clone(); Py::Bool({ let t1678 = Py::Int(0i64); if py::gt(&t1677, &t1678)? { true } else { false } }) }).truthy() { Py::str("downstream") } else { Py::str("upstream") }) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_side_friction(mut v_v: Py, mut v_R: Py, mut v_e: Py) -> R {
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("R"), v_R.clone())]))?;
    let _ = f__e_check(v_e.clone())?;
    v_a = py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&c_G_()?, &v_R.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("f"), f__r(f__snap(py::truediv(&py::sub(&v_a.clone(), &v_e.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_e.clone())?)?)?, Py::Float(1.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_signal_dd1(mut v_lam: Py, mut v_s: Py, mut v_r: Py, mut v_g: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("s"), v_s.clone()), (Py::str("r"), v_r.clone()), (Py::str("g"), v_g.clone())]))?;
    let _ = f__need({ let t1801 = v_lam.clone(); Py::Bool({ let t1802 = v_s.clone(); if py::lt(&t1801, &t1802)? { true } else { false } }) }, Py::str("the arrival flow must be below the saturation flow"))?;
    v_C = py::add(&v_r.clone(), &v_g.clone())?;
    v_tc = py::truediv(&py::mul(&v_lam.clone(), &v_r.clone())?, &py::sub(&v_s.clone(), &v_lam.clone())?)?;
    let _ = f__need({ let t1803 = v_tc.clone(); Py::Bool({ let t1804 = py::mul(&v_g.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::le(&t1803, &t1804)? { true } else { false } }) }, Py::str("the queue does not clear within the green (oversaturated: lam C > s g)"))?;
    v_rho = py::truediv(&v_lam.clone(), &v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("Q_max"), f__r(py::mul(&v_lam.clone(), &v_r.clone())?)?), (Py::str("t_clear"), f__r(v_tc.clone())?), (Py::str("D_total"), f__r(py::truediv(&py::mul(&py::mul(&v_lam.clone(), &v_r.clone())?, &v_r.clone())?, &py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?)?), (Py::str("d_uniform"), f__r(py::truediv(&py::mul(&v_r.clone(), &v_r.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_C.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?)?), (Py::str("P_stopped"), f__r(py::truediv(&py::add(&v_r.clone(), &v_tc.clone())?, &v_C.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_signal_shock_queue(mut v_q_a: Py, mut v_k_a: Py, mut v_k_j: Py, mut v_q_s: Py, mut v_k_s: Py, mut v_r: Py) -> R {
    let mut v_t: Py = Py::None;
    let mut v_w1: Py = Py::None;
    let mut v_w2: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("q_a"), v_q_a.clone()), (Py::str("k_a"), v_k_a.clone()), (Py::str("k_j"), v_k_j.clone()), (Py::str("q_s"), v_q_s.clone()), (Py::str("k_s"), v_k_s.clone()), (Py::str("r"), v_r.clone())]))?;
    let _ = f__need({ let t1780 = { let t1776 = v_k_a.clone(); Py::Bool({ let t1777 = v_k_j.clone(); if py::lt(&t1776, &t1777)? { true } else { false } }) }; if !t1780.truthy() { t1780 } else { { let t1778 = v_k_s.clone(); Py::Bool({ let t1779 = v_k_j.clone(); if py::lt(&t1778, &t1779)? { true } else { false } }) } } }, Py::str("the arrival and discharge densities must be below the jam density"))?;
    v_w1 = py::truediv(&py::neg(&v_q_a.clone())?, &py::sub(&v_k_j.clone(), &v_k_a.clone())?)?;
    v_w2 = py::truediv(&py::neg(&v_q_s.clone())?, &py::sub(&v_k_j.clone(), &v_k_s.clone())?)?;
    let _ = f__need({ let t1781 = v_w2.clone(); Py::Bool({ let t1782 = v_w1.clone(); if py::lt(&t1781, &t1782)? { true } else { false } }) }, Py::str("the discharge wave must be faster than the queue growth (q_s/(k_j - k_s) > q_a/(k_j - k_a))"))?;
    v_t = py::truediv(&py::mul(&v_w2.clone(), &v_r.clone())?, &py::sub(&v_w2.clone(), &v_w1.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("L_max"), f__r(py::mul(&py::neg(&v_w1.clone())?, &v_t.clone())?)?), (Py::str("t_max"), f__r(v_t.clone())?), (Py::str("w_queue"), f__kmh(v_w1.clone())?), (Py::str("w_discharge"), f__kmh(v_w2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_simplified_curve(mut v_e: Py, mut v_f: Py, mut v_v: Py, mut v_R: Py) -> R {
    let mut v_Rm: Py = Py::None;
    let mut v_vm: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("e"), v_e.clone()), (Py::str("f"), v_f.clone())]))?;
    let _ = f__need({ let t1818 = py::neg(&Py::Float(0.1))?; Py::Bool({ let t1819 = v_e.clone(); if py::le(&t1818, &t1819)? { { let t1820 = Py::Float(0.2); if py::le(&t1819, &t1820)? { true } else { false } } } else { false } }) }, Py::str("the superelevation must lie between -10% and 20%"))?;
    let _ = f__need({ let t1821 = Py::Int(0i64); Py::Bool({ let t1822 = v_f.clone(); if py::lt(&t1821, &t1822)? { { let t1823 = Py::Float(0.5); if py::le(&t1822, &t1823)? { true } else { false } } } else { false } }) }, Py::str("the side friction factor must lie in (0, 0.5]"))?;
    let _ = f__need({ let t1824 = py::add(&v_e.clone(), &v_f.clone())?; Py::Bool({ let t1825 = Py::Int(0i64); if py::gt(&t1824, &t1825)? { true } else { false } }) }, Py::str("e + f must be positive"))?;
    let _ = f__need({ let t1826 = { let t1828 = v_v.clone(); Py::Bool({ let t1829 = Py::None; if py::is(&t1828, &t1829) { true } else { false } }) }; Py::Bool({ let t1827 = { let t1830 = v_R.clone(); Py::Bool({ let t1831 = Py::None; if py::is(&t1830, &t1831) { true } else { false } }) }; if py::ne(&t1826, &t1827) { true } else { false } }) }, Py::str("give the speed or the radius (one of them)"))?;
    if ({ let t1832 = v_v.clone(); Py::Bool({ let t1833 = Py::None; if !py::is(&t1832, &t1833) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone())]))?;
        v_Rm = py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&c_G()?, &py::add(&v_e.clone(), &v_f.clone())?)?)?;
        return Ok(Py::dict(vec![(Py::str("R"), f__r(v_Rm.clone())?), (Py::str("v"), f__r(v_v.clone())?), (Py::str("v_kmh"), f__r(py::mul(&v_v.clone(), &Py::Float(3.6))?)?)]));
    }
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone())]))?;
    v_vm = py::math1("sqrt", &py::mul(&py::mul(&c_G()?, &v_R.clone())?, &py::add(&v_e.clone(), &v_f.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("R"), f__r(v_R.clone())?), (Py::str("v"), f__r(v_vm.clone())?), (Py::str("v_kmh"), f__r(py::mul(&v_vm.clone(), &Py::Float(3.6))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_skid_speed(mut v_d: Py, mut v_f: Py, mut v_G: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("f"), v_f.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t1857 = py::add(&v_f.clone(), &v_G.clone())?; Py::Bool({ let t1858 = Py::Int(0i64); if py::gt(&t1857, &t1858)? { true } else { false } }) }, Py::str("f + G must be positive"))?;
    v_v = py::math1("sqrt", &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &c_G_()?)?, &py::add(&v_f.clone(), &v_G.clone())?)?, &v_d.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("v"), f__r(v_v.clone())?), (Py::str("v_kmh"), f__kmh(v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_skim(mut v_roads: Py, mut v_speeds: Py, mut v_directed: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_mat: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_o: Py = Py::None;
    v_directed = f__truthy(v_directed.clone())?;
    v_adj = f__graph(v_roads.clone(), v_speeds.clone(), v_directed.clone())?;
    v_nodes = py::sorted(&v_adj.clone(), (Py::Bool(false)).truthy())?;
    let _ = f__need({ let t2284 = py::len(&v_nodes.clone())?; Py::Bool({ let t2285 = Py::Int(60i64); if py::le(&t2284, &t2285)? { true } else { false } }) }, Py::str("at most 60 places"))?;
    v_mat = Py::list(vec![]);
    for t2286 in py::iter(&(v_nodes.clone()))? {
        v_o = t2286;
        v_best = f__dijkstra(v_adj.clone(), v_o.clone())?;
        let _ = f__need({ let t2298 = py::len(&v_best.clone())?; Py::Bool({ let t2299 = py::len(&v_nodes.clone())?; if py::eq(&t2298, &t2299) { true } else { false } }) }, Py::str("every place must be reachable from every other"))?;
        let _ = py::method(&v_mat.clone(), "append", &[{ let mut out = vec![]; for t2300 in py::iter(&(v_nodes.clone()))? { let mut v_d: Py; 
v_d = t2300;
out.push(f__r(py::getitem(&py::getitem(&v_best.clone(), &v_d.clone())?, &Py::Int(0i64))?)?); } Py::list(out) }])?;
    }
    return Ok(Py::dict(vec![(Py::str("places"), py::method(&Py::str(", "), "join", &[v_nodes.clone()])?), (Py::str("times"), v_mat.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_slow_distance(mut v_v1: Py, mut v_v2: Py, mut v_f: Py, mut v_G: Py) -> R {
    let mut v_dec: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v1"), v_v1.clone()), (Py::str("f"), v_f.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("v2"), v_v2.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t1882 = v_v2.clone(); Py::Bool({ let t1883 = v_v1.clone(); if py::lt(&t1882, &t1883)? { true } else { false } }) }, Py::str("the final speed must be below the initial speed"))?;
    v_dec = py::mul(&Py::Float(9.81), &py::add(&v_f.clone(), &v_G.clone())?)?;
    let _ = f__need({ let t1884 = v_dec.clone(); Py::Bool({ let t1885 = Py::Int(0i64); if py::gt(&t1884, &t1885)? { true } else { false } }) }, Py::str("f + G must be positive"))?;
    return Ok(Py::dict(vec![(Py::str("d"), f__r(py::truediv(&py::sub(&py::mul(&v_v1.clone(), &v_v1.clone())?, &py::mul(&v_v2.clone(), &v_v2.clone())?)?, &py::mul(&Py::Int(2i64), &v_dec.clone())?)?)?), (Py::str("t"), f__r(py::truediv(&py::sub(&v_v1.clone(), &v_v2.clone())?, &v_dec.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_speed_percentiles(mut v_speeds: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::sorted(&f__speeds(v_speeds.clone())?, (Py::Bool(false)).truthy())?;
    return Ok(Py::dict(vec![(Py::str("p85"), f__r(f__pct(v_s.clone(), Py::Int(85i64))?)?), (Py::str("p50"), f__r(f__pct(v_s.clone(), Py::Int(50i64))?)?), (Py::str("p15"), f__r(f__pct(v_s.clone(), Py::Int(15i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ssd(mut v_v: Py, mut v_t_r: Py, mut v_f: Py, mut v_a: Py, mut v_G: Py) -> R {
    let mut v_db: Py = Py::None;
    let mut v_dec: Py = Py::None;
    let mut v_dr: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("t_r"), v_t_r.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t1985 = { let t1987 = v_f.clone(); Py::Bool({ let t1988 = Py::None; if py::is(&t1987, &t1988) { true } else { false } }) }; Py::Bool({ let t1986 = { let t1989 = v_a.clone(); Py::Bool({ let t1990 = Py::None; if py::is(&t1989, &t1990) { true } else { false } }) }; if py::ne(&t1985, &t1986) { true } else { false } }) }, Py::str("give either the friction coefficient f or the deceleration a (not both)"))?;
    if ({ let t1991 = v_f.clone(); Py::Bool({ let t1992 = Py::None; if !py::is(&t1991, &t1992) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::dict(vec![(Py::str("f"), v_f.clone())]))?;
        v_dec = py::mul(&c_G_()?, &py::add(&v_f.clone(), &v_G.clone())?)?;
    } else {
        let _ = f__pos(Py::dict(vec![(Py::str("a"), v_a.clone())]))?;
        v_dec = py::add(&v_a.clone(), &py::mul(&Py::Float(9.81), &v_G.clone())?)?;
    }
    let _ = f__need({ let t1993 = v_dec.clone(); Py::Bool({ let t1994 = Py::Int(0i64); if py::gt(&t1993, &t1994)? { true } else { false } }) }, Py::str("the grade is too steep: the vehicle cannot stop (f + G must be positive)"))?;
    let t1995 = (Py::tuple(vec![py::mul(&v_v.clone(), &v_t_r.clone())?, py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&Py::Int(2i64), &v_dec.clone())?)?])).items()?;
    if t1995.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dr = t1995[0].clone();
    v_db = t1995[1].clone();
    return Ok(Py::dict(vec![(Py::str("SSD"), f__r(py::add(&v_dr.clone(), &v_db.clone())?)?), (Py::str("d_r"), f__r(v_dr.clone())?), (Py::str("d_b"), f__r(v_db.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_station_headway(mut v_v: Py, mut v_a: Py, mut v_b: Py, mut v_L_t: Py, mut v_t_d: Py, mut v_s_m: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone()), (Py::str("L_t"), v_L_t.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("t_d"), v_t_d.clone()), (Py::str("s_m"), v_s_m.clone())]))?;
    v_s = py::add(&v_L_t.clone(), &v_s_m.clone())?;
    v_tc = (if ({ let t1553 = v_s.clone(); Py::Bool({ let t1554 = py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?; if py::le(&t1553, &t1554)? { true } else { false } }) }).truthy() { py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(2i64), &v_s.clone())?, &v_a.clone())?)? } else { py::add(&py::truediv(&v_v.clone(), &py::mul(&Py::Int(2i64), &v_a.clone())?)?, &py::truediv(&v_s.clone(), &v_v.clone())?)? });
    v_h = py::add(&py::add(&v_t_d.clone(), &v_tc.clone())?, &py::truediv(&v_v.clone(), &v_b.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("h"), f__r(v_h.clone())?), (Py::str("trains_per_h"), f__r(py::truediv(&Py::Int(3600i64), &v_h.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_steer_for_radius(mut v_m: Py, mut v_L: Py, mut v_l_f: Py, mut v_C_f: Py, mut v_C_r: Py, mut v_u: Py, mut v_R: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_l_r: Py = Py::None;
    let t1959 = (f__bike_check(v_m.clone(), v_L.clone(), v_l_f.clone(), v_C_f.clone(), v_C_r.clone())?).items()?;
    if t1959.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l_r = t1959[0].clone();
    v_K = t1959[1].clone();
    let _ = f__pos(Py::dict(vec![(Py::str("u"), v_u.clone()), (Py::str("R"), v_R.clone())]))?;
    v_d = py::truediv(&py::add(&v_L.clone(), &py::mul(&py::mul(&v_K.clone(), &v_u.clone())?, &v_u.clone())?)?, &v_R.clone())?;
    let _ = f__need({ let t1960 = py::add(&v_L.clone(), &py::mul(&py::mul(&v_K.clone(), &v_u.clone())?, &v_u.clone())?)?; Py::Bool({ let t1961 = Py::Int(0i64); if py::gt(&t1960, &t1961)? { true } else { false } }) }, Py::str("above the critical speed: no steady turn"))?;
    return Ok(Py::dict(vec![(Py::str("delta"), f__r(py::math1("degrees", &v_d.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_superelevation(mut v_v: Py, mut v_R: Py, mut v_f: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("R"), v_R.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("f"), v_f.clone())]))?;
    let _ = f__need({ let t2014 = v_f.clone(); Py::Bool({ let t2015 = Py::Int(0i64); if py::ge(&t2014, &t2015)? { true } else { false } }) }, Py::str("the side friction factor must not be negative"))?;
    v_a = py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&c_G_()?, &v_R.clone())?)?;
    v_e = py::truediv(&py::sub(&v_a.clone(), &v_f.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_f.clone())?)?)?;
    let _ = f__need({ let t2016 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t2017 = v_e.clone(); if py::lt(&t2016, &t2017)? { { let t2018 = Py::Int(1i64); if py::lt(&t2017, &t2018)? { true } else { false } } } else { false } }) }, Py::str("the required superelevation is outside -1 .. 1"))?;
    return Ok(Py::dict(vec![(Py::str("e"), f__r(f__snap(v_e.clone(), Py::Float(1.0))?)?), (Py::str("e_percent"), f__r(f__snap(py::mul(&Py::Int(100i64), &v_e.clone())?, Py::Float(100.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_top_speed(mut v_m: Py, mut v_P: Py, mut v_C_D: Py, mut v_A: Py, mut v_rho: Py, mut v_f_r: Py, mut v_G: Py, mut v_eta: Py) -> R {
    let mut v_Pw: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_a_: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b_: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("P"), v_P.clone()), (Py::str("C_D"), v_C_D.clone()), (Py::str("A"), v_A.clone()), (Py::str("rho"), v_rho.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("f_r"), v_f_r.clone()), (Py::str("G"), v_G.clone()), (Py::str("eta"), v_eta.clone())]))?;
    let _ = f__need({ let t2074 = { let t2069 = Py::Int(0i64); Py::Bool({ let t2070 = v_f_r.clone(); if py::le(&t2069, &t2070)? { { let t2071 = Py::Float(0.1); if py::le(&t2070, &t2071)? { true } else { false } } } else { false } }) }; if !t2074.truthy() { t2074 } else { { let t2072 = py::abs(&v_G.clone())?; Py::Bool({ let t2073 = Py::Float(0.3); if py::le(&t2072, &t2073)? { true } else { false } }) } } }, Py::str("need 0 <= f_r <= 0.1 and |G| <= 0.3"))?;
    let _ = f__need({ let t2075 = Py::Int(0i64); Py::Bool({ let t2076 = v_eta.clone(); if py::lt(&t2075, &t2076)? { { let t2077 = Py::Int(1i64); if py::le(&t2076, &t2077)? { true } else { false } } } else { false } }) }, Py::str("the drivetrain efficiency must lie in (0, 1]"))?;
    v_th = py::math1("atan", &v_G.clone())?;
    v_a = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_C_D.clone())?, &v_A.clone())?;
    v_b = py::mul(&py::mul(&v_m.clone(), &Py::Float(9.81))?, &py::add(&py::mul(&v_f_r.clone(), &py::math1("cos", &v_th.clone())?)?, &py::math1("sin", &v_th.clone())?)?)?;
    v_Pw = py::mul(&v_eta.clone(), &v_P.clone())?;
    v_f = { let v_A = v_A.clone(); let v_C_D = v_C_D.clone(); let v_G = v_G.clone(); let v_P = v_P.clone(); let v_Pw = v_Pw.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_eta = v_eta.clone(); let v_f_r = v_f_r.clone(); let v_m = v_m.clone(); let v_rho = v_rho.clone(); let v_th = v_th.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::mul(&v_a.clone(), &py::pow(&v_v.clone(), &Py::Int(3i64))?)?, &py::mul(&v_b.clone(), &v_v.clone())?)?, &v_Pw.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_lo = (if ({ let t2078 = v_b.clone(); Py::Bool({ let t2079 = Py::Int(0i64); if py::lt(&t2078, &t2079)? { true } else { false } }) }).truthy() { py::math1("sqrt", &py::truediv(&py::neg(&v_b.clone())?, &py::mul(&Py::Int(3i64), &v_a.clone())?)?)? } else { Py::Float(0.0) });
    v_hi = py::minmax(&[Py::Float(1.0), py::mul(&Py::Int(2i64), &v_lo.clone())?, py::pow(&py::truediv(&v_Pw.clone(), &v_a.clone())?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?], true)?;
    while ({ let t2080 = py::call(&v_f.clone(), &[v_hi.clone()])?; Py::Bool({ let t2081 = Py::Int(0i64); if py::lt(&t2080, &t2081)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Int(2i64))?;
    }
    let t2082 = (Py::tuple(vec![v_lo.clone(), v_hi.clone()])).items()?;
    if t2082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a_ = t2082[0].clone();
    v_b_ = t2082[1].clone();
    for t2083 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t2083;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_a_.clone(), &v_b_.clone())?)?;
        if ({ let t2084 = v_mid.clone(); Py::Bool({ let t2085 = Py::tuple(vec![v_a_.clone(), v_b_.clone()]); if py::contains(&t2085, &t2084)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t2086 = py::call(&v_f.clone(), &[v_mid.clone()])?; Py::Bool({ let t2087 = Py::Int(0i64); if py::lt(&t2086, &t2087)? { true } else { false } }) }).truthy() {
            v_a_ = v_mid.clone();
        } else {
            v_b_ = v_mid.clone();
        }
    }
    v_v = py::mul(&Py::Float(0.5), &py::add(&v_a_.clone(), &v_b_.clone())?)?;
    let _ = f__need({ let t2088 = v_v.clone(); Py::Bool({ let t2089 = Py::Float(0.5); if py::gt(&t2088, &t2089)? { true } else { false } }) }, Py::str("no steady top speed (the power does not overcome the resistance)"))?;
    return Ok(Py::dict(vec![(Py::str("v_max"), f__r(v_v.clone())?), (Py::str("v_max_kmh"), f__r(py::mul(&v_v.clone(), &Py::Float(3.6))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_traction_limit(mut v_m: Py, mut v_L: Py, mut v_l_f: Py, mut v_h: Py, mut v_mu: Py, mut v_f_r: Py, mut v_drive: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_Wf: Py = Py::None;
    let mut v_Wr: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_l_r: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("h"), v_h.clone()), (Py::str("mu"), v_mu.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("f_r"), v_f_r.clone())]))?;
    let _ = f__check_axles(v_L.clone(), v_l_f.clone())?;
    let t2115 = (Py::tuple(vec![py::mul(&v_m.clone(), &c_G_()?)?, py::sub(&v_L.clone(), &v_l_f.clone())?])).items()?;
    if t2115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t2115[0].clone();
    v_l_r = t2115[1].clone();
    if ({ let t2116 = v_drive.clone(); Py::Bool({ let t2117 = Py::str("rear"); if py::eq(&t2116, &t2117) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2118 = py::mul(&v_mu.clone(), &v_h.clone())?; Py::Bool({ let t2119 = v_L.clone(); if py::lt(&t2118, &t2119)? { true } else { false } }) }, Py::str("mu h must be less than the wheelbase"))?;
        v_F = py::truediv(&py::truediv(&py::mul(&py::mul(&v_mu.clone(), &v_W.clone())?, &py::sub(&v_l_f.clone(), &py::mul(&v_f_r.clone(), &v_h.clone())?)?)?, &v_L.clone())?, &py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&v_mu.clone(), &v_h.clone())?, &v_L.clone())?)?)?;
        v_Wr = py::truediv(&v_F.clone(), &v_mu.clone())?;
        v_Wf = py::sub(&v_W.clone(), &v_Wr.clone())?;
    } else {
        if ({ let t2120 = v_drive.clone(); Py::Bool({ let t2121 = Py::str("front"); if py::eq(&t2120, &t2121) { true } else { false } }) }).truthy() {
            v_F = py::truediv(&py::truediv(&py::mul(&py::mul(&v_mu.clone(), &v_W.clone())?, &py::add(&v_l_r.clone(), &py::mul(&v_f_r.clone(), &v_h.clone())?)?)?, &v_L.clone())?, &py::add(&Py::Int(1i64), &py::truediv(&py::mul(&v_mu.clone(), &v_h.clone())?, &v_L.clone())?)?)?;
            v_Wf = py::truediv(&v_F.clone(), &v_mu.clone())?;
            v_Wr = py::sub(&v_W.clone(), &v_Wf.clone())?;
        } else {
            v_F = py::mul(&v_mu.clone(), &v_W.clone())?;
            v_Wr = py::truediv(&py::add(&py::mul(&v_W.clone(), &v_l_f.clone())?, &py::mul(&py::sub(&v_F.clone(), &py::mul(&v_f_r.clone(), &v_W.clone())?)?, &v_h.clone())?)?, &v_L.clone())?;
            v_Wf = py::sub(&v_W.clone(), &v_Wr.clone())?;
        }
    }
    v_a = py::truediv(&py::sub(&v_F.clone(), &py::mul(&v_f_r.clone(), &v_W.clone())?)?, &v_m.clone())?;
    let _ = f__need({ let t2122 = v_a.clone(); Py::Bool({ let t2123 = Py::Int(0i64); if py::gt(&t2122, &t2123)? { true } else { false } }) }, Py::str("the friction-limited tractive force does not exceed the rolling resistance"))?;
    let _ = f__need({ let t2128 = { let t2124 = v_Wf.clone(); Py::Bool({ let t2125 = Py::Int(0i64); if py::gt(&t2124, &t2125)? { true } else { false } }) }; if !t2128.truthy() { t2128 } else { { let t2126 = v_Wr.clone(); Py::Bool({ let t2127 = Py::Int(0i64); if py::gt(&t2126, &t2127)? { true } else { false } }) } } }, Py::str("an axle would lift off"))?;
    return Ok(Py::dict(vec![(Py::str("F_max"), f__r(v_F.clone())?), (Py::str("a_max"), f__r(v_a.clone())?), (Py::str("W_f"), f__r(v_Wf.clone())?), (Py::str("W_r"), f__r(v_Wr.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_train_accel(mut v_m: Py, mut v_v1: Py, mut v_v2: Py, mut v_F_max: Py, mut v_P: Py, mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_G: Py, mut v_gamma: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_gr: Py = Py::None;
    let mut v_net: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_vc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("v2"), v_v2.clone()), (Py::str("F_max"), v_F_max.clone()), (Py::str("P"), v_P.clone()), (Py::str("gamma"), v_gamma.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("v1"), v_v1.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t2152 = v_v2.clone(); Py::Bool({ let t2153 = v_v1.clone(); if py::gt(&t2152, &t2153)? { true } else { false } }) }, Py::str("the final speed must exceed the initial speed"))?;
    v_R = f__davis(v_A.clone(), v_B.clone(), v_C.clone())?;
    v_gr = py::mul(&py::mul(&v_m.clone(), &c_G_()?)?, &py::math1("sin", &py::math1("atan", &v_G.clone())?)?)?;
    v_vc = py::truediv(&v_P.clone(), &v_F_max.clone())?;
    v_net = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_F_max = v_F_max.clone(); let v_G = v_G.clone(); let v_P = v_P.clone(); let v_R = v_R.clone(); let v_gamma = v_gamma.clone(); let v_gr = v_gr.clone(); let v_m = v_m.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_vc = v_vc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&(if ({ let t2154 = v_v.clone(); Py::Bool({ let t2155 = v_vc.clone(); if py::le(&t2154, &t2155)? { true } else { false } }) }).truthy() { v_F_max.clone() } else { py::truediv(&v_P.clone(), &v_v.clone())? }), &py::call(&v_R.clone(), &[v_v.clone()])?)?, &v_gr.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let _ = f__need({ let t2160 = { let t2156 = py::call(&v_net.clone(), &[v_v2.clone()])?; Py::Bool({ let t2157 = py::mul(&Py::Float(1e-09), &v_F_max.clone())?; if py::gt(&t2156, &t2157)? { true } else { false } }) }; if !t2160.truthy() { t2160 } else { { let t2158 = py::call(&v_net.clone(), &[v_v1.clone()])?; Py::Bool({ let t2159 = Py::Int(0i64); if py::gt(&t2158, &t2159)? { true } else { false } }) } } }, Py::str("the tractive effort does not exceed the resistance up to the final speed"))?;
    v_pts = py::add(&py::add(&Py::list(vec![v_v1.clone()]), &(if ({ let t2161 = v_v1.clone(); Py::Bool({ let t2162 = v_vc.clone(); if py::lt(&t2161, &t2162)? { { let t2163 = v_v2.clone(); if py::lt(&t2162, &t2163)? { true } else { false } } } else { false } }) }).truthy() { Py::list(vec![v_vc.clone()]) } else { Py::list(vec![]) }))?, &Py::list(vec![v_v2.clone()]))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(25i64))?;
        v_t = ax_engine::pymp::call("quad", &[{ let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_F_max = v_F_max.clone(); let v_G = v_G.clone(); let v_P = v_P.clone(); let v_R = v_R.clone(); let v_gamma = v_gamma.clone(); let v_gr = v_gr.clone(); let v_m = v_m.clone(); let v_net = v_net.clone(); let v_pts = v_pts.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_vc = v_vc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&v_gamma.clone(), &v_m.clone())?, &py::call(&v_net.clone(), &[v_v.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_pts.clone()], &[])?;
        v_d = ax_engine::pymp::call("quad", &[{ let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_F_max = v_F_max.clone(); let v_G = v_G.clone(); let v_P = v_P.clone(); let v_R = v_R.clone(); let v_gamma = v_gamma.clone(); let v_gr = v_gr.clone(); let v_m = v_m.clone(); let v_net = v_net.clone(); let v_pts = v_pts.clone(); let v_t = v_t.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_vc = v_vc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&py::mul(&v_gamma.clone(), &v_m.clone())?, &v_v.clone())?, &py::call(&v_net.clone(), &[v_v.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_pts.clone()], &[])?;
    }
    return Ok(Py::dict(vec![(Py::str("t"), f__r(py::float(&v_t.clone())?)?), (Py::str("d"), f__r(py::float(&v_d.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_train_braking(mut v_m: Py, mut v_v1: Py, mut v_F_b: Py, mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_G: Py, mut v_gamma: Py, mut v_v2: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_gr: Py = Py::None;
    let mut v_ret: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("v1"), v_v1.clone()), (Py::str("F_b"), v_F_b.clone()), (Py::str("gamma"), v_gamma.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("v2"), v_v2.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t2187 = v_v2.clone(); Py::Bool({ let t2188 = v_v1.clone(); if py::lt(&t2187, &t2188)? { true } else { false } }) }, Py::str("the final speed must be below the initial speed"))?;
    v_R = f__davis(v_A.clone(), v_B.clone(), v_C.clone())?;
    v_gr = py::mul(&py::mul(&v_m.clone(), &c_G_()?)?, &py::math1("sin", &py::math1("atan", &v_G.clone())?)?)?;
    v_ret = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_F_b = v_F_b.clone(); let v_G = v_G.clone(); let v_R = v_R.clone(); let v_gamma = v_gamma.clone(); let v_gr = v_gr.clone(); let v_m = v_m.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&v_F_b.clone(), &py::call(&v_R.clone(), &[v_v.clone()])?)?, &v_gr.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let _ = f__need({ let t2189 = py::call(&v_ret.clone(), &[v_v2.clone()])?; Py::Bool({ let t2190 = Py::Int(0i64); if py::gt(&t2189, &t2190)? { true } else { false } }) }, Py::str("the train does not slow down on this downgrade with this braking force"))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(25i64))?;
        v_S = ax_engine::pymp::call("quad", &[{ let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_F_b = v_F_b.clone(); let v_G = v_G.clone(); let v_R = v_R.clone(); let v_gamma = v_gamma.clone(); let v_gr = v_gr.clone(); let v_m = v_m.clone(); let v_ret = v_ret.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&py::mul(&v_gamma.clone(), &v_m.clone())?, &v_v.clone())?, &py::call(&v_ret.clone(), &[v_v.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![v_v2.clone(), v_v1.clone()])], &[])?;
        v_t = ax_engine::pymp::call("quad", &[{ let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_F_b = v_F_b.clone(); let v_G = v_G.clone(); let v_R = v_R.clone(); let v_S = v_S.clone(); let v_gamma = v_gamma.clone(); let v_gr = v_gr.clone(); let v_m = v_m.clone(); let v_ret = v_ret.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&v_gamma.clone(), &v_m.clone())?, &py::call(&v_ret.clone(), &[v_v.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![v_v2.clone(), v_v1.clone()])], &[])?;
    }
    return Ok(Py::dict(vec![(Py::str("S"), f__r(py::float(&v_S.clone())?)?), (Py::str("t"), f__r(py::float(&v_t.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trip_regression(mut v_X: Py, mut v_y: Py, mut v_x_new: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_coef: Py = Py::None;
    let mut v_fit: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_sst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xn: Py = Py::None;
    let mut v_yy: Py = Py::None;
    v_x = f__mat(Py::str("X"), v_X.clone(), Py::None, Py::None, Py::Bool(false), Py::Bool(false))?;
    v_yy = f__vec(Py::str("y"), v_y.clone(), py::len(&v_x.clone())?, Py::Bool(false), Py::Bool(false))?;
    v_p = py::len(&py::getitem(&v_x.clone(), &Py::Int(0i64))?)?;
    let _ = f__need({ let t2340 = py::len(&v_x.clone())?; Py::Bool({ let t2341 = py::add(&v_p.clone(), &Py::Int(1i64))?; if py::gt(&t2340, &t2341)? { true } else { false } }) }, Py::str("need more zones than coefficients"))?;
    v_A = { let mut out = vec![]; for t2342 in py::iter(&(v_x.clone()))? { let mut v_row: Py; 
v_row = t2342;
out.push(py::add(&Py::list(vec![Py::Float(1.0)]), &v_row.clone())?); } Py::list(out) };
    v_coef = f__lstsq_qr(v_A.clone(), v_yy.clone())?;
    v_fit = { let mut out = vec![]; for t2369 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t2369;
out.push(py::fsum(&{ let mut out = vec![]; for t2370 in py::iter(&(py::zip(&[v_row.clone(), v_coef.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t2371 = (t2370).items()?;
if t2371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2371[0].clone();
v_c = t2371[1].clone();
out.push(py::mul(&v_a.clone(), &v_c.clone())?); } Py::list(out) })?); } Py::list(out) };
    v_my = py::truediv(&py::fsum(&v_yy.clone())?, &py::len(&v_yy.clone())?)?;
    v_sst = py::fsum(&{ let mut out = vec![]; for t2372 in py::iter(&(v_yy.clone()))? { let mut v_v: Py; 
v_v = t2372;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    let _ = f__need({ let t2373 = v_sst.clone(); Py::Bool({ let t2374 = Py::Int(0i64); if py::gt(&t2373, &t2374)? { true } else { false } }) }, Py::str("the trips must not all be equal"))?;
    v_r2 = py::sub(&Py::Int(1i64), &py::truediv(&py::fsum(&{ let mut out = vec![]; for t2375 in py::iter(&(py::zip(&[v_yy.clone(), v_fit.clone()])?))? { let mut v_f: Py; let mut v_v: Py; 
let t2376 = (t2375).items()?;
if t2376.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2376[0].clone();
v_f = t2376[1].clone();
out.push(py::pow(&py::sub(&v_v.clone(), &v_f.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_sst.clone())?)?;
    v_out = Py::dict(vec![(Py::str("coef"), f__r(v_coef.clone())?), (Py::str("r2"), f__r(v_r2.clone())?)]);
    if ({ let t2386 = v_x_new.clone(); Py::Bool({ let t2387 = Py::None; if !py::is(&t2386, &t2387) { true } else { false } }) }).truthy() {
        v_xn = f__vec(Py::str("x_new"), v_x_new.clone(), v_p.clone(), Py::Bool(false), Py::Bool(false))?;
        py::setitem(&v_out.clone(), &Py::str("y_new"), f__r(py::add(&py::getitem(&v_coef.clone(), &Py::Int(0i64))?, &py::fsum(&{ let mut out = vec![]; for t2388 in py::iter(&(py::zip(&[v_xn.clone(), py::slice(&v_coef.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_c: Py; 
let t2389 = (t2388).items()?;
if t2389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2389[0].clone();
v_c = t2389[1].clone();
out.push(py::mul(&v_a.clone(), &v_c.clone())?); } Py::list(out) })?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_understeer(mut v_m: Py, mut v_L: Py, mut v_l_f: Py, mut v_C_f: Py, mut v_C_r: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_l_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2401 = (f__bike_check(v_m.clone(), v_L.clone(), v_l_f.clone(), v_C_f.clone(), v_C_r.clone())?).items()?;
    if t2401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l_r = t2401[0].clone();
    v_K = t2401[1].clone();
    let _ = f__need({ let t2402 = py::abs(&v_K.clone())?; Py::Bool({ let t2403 = py::truediv(&py::mul(&Py::Float(1e-12), &v_m.clone())?, &py::minmax(&[v_C_f.clone(), v_C_r.clone()], false)?)?; if py::gt(&t2402, &t2403)? { true } else { false } }) }, Py::str("neutral steer (K = 0): no characteristic or critical speed"))?;
    v_v = py::math1("sqrt", &py::truediv(&v_L.clone(), &py::abs(&v_K.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("K"), f__r(v_K.clone())?), (Py::str("K_deg_per_g"), f__r(py::math1("degrees", &py::mul(&v_K.clone(), &c_G_()?)?)?)?), (Py::str("behaviour"), (if ({ let t2413 = v_K.clone(); Py::Bool({ let t2414 = Py::Int(0i64); if py::gt(&t2413, &t2414)? { true } else { false } }) }).truthy() { Py::str("understeer") } else { Py::str("oversteer") })), (Py::str("v_c"), f__r(v_v.clone())?), (Py::str("v_c_kmh"), f__kmh(v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_underwood_cap(mut v_vf: Py, mut v_k0: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("vf"), v_vf.clone()), (Py::str("k0"), v_k0.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("q_max"), f__vph(py::truediv(&py::mul(&v_vf.clone(), &v_k0.clone())?, &Py::Float(std::f64::consts::E))?)?), (Py::str("v_m"), f__kmh(py::truediv(&v_vf.clone(), &Py::Float(std::f64::consts::E))?)?), (Py::str("k_m"), f__vpkm(v_k0.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_underwood_fit(mut v_speeds: Py, mut v_densities: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vf: Py = Py::None;
    let t2452 = (f__check_obs(v_speeds.clone(), v_densities.clone())?).items()?;
    if t2452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2452[0].clone();
    v_k = t2452[1].clone();
    let t2463 = (f__linfit(v_k.clone(), { let mut out = vec![]; for t2462 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2462;
out.push(py::math1("log", &v_x.clone())?); } Py::list(out) })?).items()?;
    if t2463.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2463[0].clone();
    v_b = t2463[1].clone();
    v_r2 = t2463[2].clone();
    let _ = f__need({ let t2464 = v_b.clone(); Py::Bool({ let t2465 = Py::Int(0i64); if py::lt(&t2464, &t2465)? { true } else { false } }) }, Py::str("the speeds do not fall with density: the Underwood model does not fit"))?;
    let t2466 = (Py::tuple(vec![py::math1("exp", &v_a.clone())?, py::truediv(&py::neg(&Py::Int(1i64))?, &v_b.clone())?])).items()?;
    if t2466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vf = t2466[0].clone();
    v_k0 = t2466[1].clone();
    return Ok(Py::dict(vec![(Py::str("vf"), f__r(v_vf.clone())?), (Py::str("k0"), f__r(v_k0.clone())?), (Py::str("q_max"), f__r(py::truediv(&py::mul(&v_vf.clone(), &v_k0.clone())?, &Py::Float(std::f64::consts::E))?)?), (Py::str("r2"), f__r(v_r2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vehicle_resistance(mut v_m: Py, mut v_v: Py, mut v_C_D: Py, mut v_A: Py, mut v_rho: Py, mut v_f_r: Py, mut v_G: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Fa: Py = Py::None;
    let mut v_Fg: Py = Py::None;
    let mut v_Fr: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("C_D"), v_C_D.clone()), (Py::str("A"), v_A.clone()), (Py::str("rho"), v_rho.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("f_r"), v_f_r.clone()), (Py::str("G"), v_G.clone())]))?;
    let _ = f__need({ let t2485 = v_v.clone(); Py::Bool({ let t2486 = Py::Int(0i64); if py::ge(&t2485, &t2486)? { true } else { false } }) }, Py::str("the speed must be zero or positive"))?;
    let _ = f__need({ let t2487 = Py::Int(0i64); Py::Bool({ let t2488 = v_f_r.clone(); if py::le(&t2487, &t2488)? { { let t2489 = Py::Float(0.1); if py::le(&t2488, &t2489)? { true } else { false } } } else { false } }) }, Py::str("the rolling resistance coefficient must lie between 0 and 0.1"))?;
    let _ = f__need({ let t2490 = py::abs(&v_G.clone())?; Py::Bool({ let t2491 = Py::Float(0.3); if py::le(&t2490, &t2491)? { true } else { false } }) }, Py::str("the grade must lie between -30% and 30%"))?;
    let t2492 = (f__resist(v_m.clone(), v_v.clone(), v_C_D.clone(), v_A.clone(), v_rho.clone(), v_f_r.clone(), v_G.clone())?).items()?;
    if t2492.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Fr = t2492[0].clone();
    v_Fa = t2492[1].clone();
    v_Fg = t2492[2].clone();
    v_F = py::add(&py::add(&v_Fr.clone(), &v_Fa.clone())?, &v_Fg.clone())?;
    return Ok(Py::dict(vec![(Py::str("F_roll"), f__r(v_Fr.clone())?), (Py::str("F_aero"), f__r(v_Fa.clone())?), (Py::str("F_grade"), f__r(v_Fg.clone())?), (Py::str("F_total"), f__r(v_F.clone())?), (Py::str("P"), f__r(py::mul(&v_F.clone(), &v_v.clone())?)?), (Py::str("P_kW"), f__r(py::truediv(&py::mul(&v_F.clone(), &v_v.clone())?, &Py::Float(1000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_webster_cycle(mut v_L: Py, mut v_y: Py) -> R {
    let mut v_C0: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("L"), v_L.clone())]))?;
    let _ = f__need({ let t2513 = Py::Bool(matches!(v_y.clone(), Py::List(_) | Py::Tuple(_))); if !t2513.truthy() { t2513 } else { { let t2511 = py::len(&v_y.clone())?; Py::Bool({ let t2512 = Py::Int(2i64); if py::ge(&t2511, &t2512)? { true } else { false } }) } } }, Py::str("give the critical flow ratio of at least two phases"))?;
    for t2514 in py::iter(&(v_y.clone()))? {
        v_v = t2514;
        let _ = f__need({ let t2518 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2518.truthy() { t2518 } else { { let t2515 = Py::Int(0i64); Py::Bool({ let t2516 = v_v.clone(); if py::lt(&t2515, &t2516)? { { let t2517 = Py::Int(1i64); if py::lt(&t2516, &t2517)? { true } else { false } } } else { false } }) } } }, Py::str("each flow ratio must lie between 0 and 1"))?;
    }
    v_Y = py::sum(&v_y.clone(), None)?;
    let _ = f__need({ let t2519 = v_Y.clone(); Py::Bool({ let t2520 = Py::Float(0.95); if py::lt(&t2519, &t2520)? { true } else { false } }) }, Py::str("the sum of the critical flow ratios must be below 0.95 (the intersection is near or over capacity)"))?;
    v_C0 = py::truediv(&py::add(&py::mul(&Py::Float(1.5), &v_L.clone())?, &Py::Int(5i64))?, &py::sub(&Py::Int(1i64), &v_Y.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("C0"), f__r(v_C0.clone())?), (Py::str("Y"), f__r(v_Y.clone())?), (Py::str("C_min"), f__r(py::truediv(&v_L.clone(), &py::sub(&Py::Int(1i64), &v_Y.clone())?)?)?), (Py::str("green_total"), f__r(py::sub(&v_C0.clone(), &v_L.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_yaw_modes(mut v_m: Py, mut v_Iz: Py, mut v_L: Py, mut v_l_f: Py, mut v_C_f: Py, mut v_C_r: Py, mut v_u: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_l_r: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let t2541 = (f__bike_check(v_m.clone(), v_L.clone(), v_l_f.clone(), v_C_f.clone(), v_C_r.clone())?).items()?;
    if t2541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l_r = t2541[0].clone();
    v_K = t2541[1].clone();
    let _ = f__pos(Py::dict(vec![(Py::str("Iz"), v_Iz.clone()), (Py::str("u"), v_u.clone())]))?;
    v_det = py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&v_C_f.clone(), &v_C_r.clone())?, &v_L.clone())?, &v_L.clone())?, &py::mul(&py::mul(&py::mul(&v_m.clone(), &v_Iz.clone())?, &v_u.clone())?, &v_u.clone())?)?, &py::add(&Py::Int(1i64), &py::truediv(&py::mul(&py::mul(&v_K.clone(), &v_u.clone())?, &v_u.clone())?, &v_L.clone())?)?)?;
    let _ = f__need({ let t2542 = v_det.clone(); Py::Bool({ let t2543 = Py::Int(0i64); if py::gt(&t2542, &t2543)? { true } else { false } }) }, Py::str("above the critical speed: the yaw motion is unstable"))?;
    v_tr = py::sub(&py::truediv(&py::neg(&py::add(&v_C_f.clone(), &v_C_r.clone())?)?, &py::mul(&v_m.clone(), &v_u.clone())?)?, &py::truediv(&py::add(&py::mul(&v_C_f.clone(), &py::pow(&v_l_f.clone(), &Py::Int(2i64))?)?, &py::mul(&v_C_r.clone(), &py::pow(&v_l_r.clone(), &Py::Int(2i64))?)?)?, &py::mul(&v_Iz.clone(), &v_u.clone())?)?)?;
    v_wn = py::math1("sqrt", &v_det.clone())?;
    return Ok(Py::dict(vec![(Py::str("wn"), f__r(v_wn.clone())?), (Py::str("zeta"), f__r(py::truediv(&py::neg(&v_tr.clone())?, &py::mul(&Py::Int(2i64), &v_wn.clone())?)?)?), (Py::str("f_n"), f__r(py::truediv(&v_wn.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_yellow_interval(mut v_v: Py, mut v_t_r: Py, mut v_a: Py, mut v_G: Py) -> R {
    let mut v_dec: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("a"), v_a.clone())]))?;
    let _ = f__nonneg(Py::dict(vec![(Py::str("t_r"), v_t_r.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("G"), v_G.clone())]))?;
    v_dec = py::add(&v_a.clone(), &py::mul(&Py::Float(9.81), &v_G.clone())?)?;
    let _ = f__need({ let t2567 = v_dec.clone(); Py::Bool({ let t2568 = Py::Int(0i64); if py::gt(&t2567, &t2568)? { true } else { false } }) }, Py::str("a + g G must be positive"))?;
    return Ok(Py::dict(vec![(Py::str("Y"), f__r(py::add(&v_t_r.clone(), &py::truediv(&v_v.clone(), &py::mul(&Py::Int(2i64), &v_dec.clone())?)?)?)?), (Py::str("x_c"), f__r(py::add(&py::mul(&v_v.clone(), &v_t_r.clone())?, &py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &py::mul(&Py::Int(2i64), &v_dec.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}
