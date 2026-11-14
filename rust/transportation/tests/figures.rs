//! Every Transportation capability's live figures (src/figures*.rs and the generic engine): its worked example gives
//! each published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use transportation_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/transportation-ai999x/figures, except the
/// generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("transport_all_or_nothing_assignment", &["graph", "series"]),
    ("transport_bottleneck_queue", &["queue"]),
    ("transport_braking_distance", &["field"]),
    ("transport_braking_weight_transfer", &["fbd"]),
    ("transport_clothoid_coordinates", &["geometry"]),
    ("transport_crest_curve_length", &["decision", "profile"]),
    ("transport_crest_curve_sight_distance", &["decision", "profile"]),
    ("transport_curve_safe_speed", &["fbd"]),
    ("transport_erlang_b_blocking", &["curve-2"]),
    ("transport_fastest_route", &["graph"]),
    ("transport_fifo_queue_simulation", &["queue"]),
    ("transport_furness_balancing", &["matrix"]),
    ("transport_gravity_model", &["matrix"]),
    ("transport_greenberg_capacity", &["diagram"]),
    ("transport_greenberg_fit", &["fit"]),
    ("transport_greenshields_capacity", &["diagram", "field"]),
    ("transport_greenshields_fit", &["fit", "xy"]),
    ("transport_greenshields_shockwave", &["diagram"]),
    ("transport_greenshields_speed_at_flow", &["diagram"]),
    ("transport_headway_probability", &["field"]),
    ("transport_horizontal_sight_offset", &["geometry"]),
    ("transport_interval_queue", &["queue"]),
    ("transport_lateral_load_transfer", &["fbd"]),
    ("transport_logit_elasticities", &["series"]),
    ("transport_logit_from_attributes", &["bars"]),
    ("transport_logit_mode_shares", &["bars"]),
    ("transport_md1_queue", &["field"]),
    ("transport_mm1_queue", &["field"]),
    ("transport_nested_logit", &["tree"]),
    ("transport_passing_sight_distance", &["breakdown"]),
    ("transport_quarter_car_frequencies", &["schematic"]),
    ("transport_quarter_car_transmissibility", &["response"]),
    ("transport_rollover_threshold", &["fbd"]),
    ("transport_sag_curve_headlight", &["decision", "profile"]),
    ("transport_shockwave_speed", &["decision", "diagram"]),
    ("transport_sight_distance_from_offset", &["geometry"]),
    ("transport_signal_green_splits", &["timing"]),
    ("transport_signal_queue_shockwave", &["timespace"]),
    ("transport_signal_queue_uniform_delay", &["queue"]),
    ("transport_skid_speed", &["field"]),
    ("transport_spot_speed_percentiles", &["distribution"]),
    ("transport_steady_state_cornering", &["schematic"]),
    ("transport_stopping_sight_distance", &["breakdown", "diagram"]),
    ("transport_superelevation_required", &["fbd"]),
    ("transport_time_and_space_mean_speed", &["distribution"]),
    ("transport_traction_limited_acceleration", &["fbd"]),
    ("transport_train_running_time", &["profile"]),
    ("transport_travel_time_reliability", &["data"]),
    ("transport_travel_time_skim", &["graph", "matrix"]),
    ("transport_trip_generation_regression", &["fit"]),
    ("transport_understeer_gradient", &["decision"]),
    ("transport_underwood_capacity", &["diagram"]),
    ("transport_underwood_fit", &["fit"]),
    ("transport_vehicle_resistance", &["breakdown"]),
    ("transport_webster_optimum_cycle", &["timing"]),
    ("transport_yellow_change_interval", &["schematic"]),
];

/// Inputs to change for a capability whose inputs are lists (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("transport_fifo_queue_simulation", "service", "[9, 5, 8, 6, 7, 4, 6]"),
    ("transport_interval_queue", "arrivals", "[80, 130, 150, 100, 70, 60]"),
    ("transport_logit_mode_shares", "V", "[-1.0, -1.5, -0.6]"),
    ("transport_logit_from_attributes", "beta", "[-0.04, -0.5]"),
    ("transport_nested_logit", "V", "[-0.8, -1.0, -0.6]"),
    ("transport_spot_speed_percentiles", "speeds", "[50, 64, 47, 75, 57, 62, 59, 69, 51, 70]"),
    ("transport_time_and_space_mean_speed", "speeds", "[50, 66, 45, 72, 58, 61]"),
    ("transport_travel_time_reliability", "times", "[21, 26, 25, 33, 27, 22, 38, 25, 24, 29]"),
    ("transport_signal_green_splits", "y", "[0.35, 0.2, 0.15]"),
    ("transport_webster_optimum_cycle", "y", "[0.25, 0.3, 0.1]"),
    ("transport_greenshields_fit", "speeds", "[85, 72, 60, 45, 30, 20]"),
    ("transport_greenberg_fit", "speeds", "[85, 72, 60, 45, 30, 20]"),
    ("transport_underwood_fit", "speeds", "[85, 72, 60, 45, 30, 20]"),
    ("transport_trip_generation_regression", "y", "[1500, 2250, 3150, 2000, 4300, 2150, 3550]"),
];

/// The arguments of a structured example: solve('key', a='1 m', b=[1, 2], c="x").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            let mut depth = 0i32;
            while i < cs.len() && !(depth == 0 && (cs[i] == ',' || cs[i] == ')')) {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                }
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        assert!(ax_engine::solve(p, key, &given, None).solved(), "{key}: the worked example does not solve: {given:?}");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    if !f.slug.contains("-live-") {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    }
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    // the model's own figures; the generic engine's are checked by its own tests
                    if !f.slug.contains("-live-") {
                        check_texts(f);
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 m" -> "16.44 m"; "[3, 6]" -> "[4.11, 8.22]").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    // a quantity's own number only ("0.5 1/s": the 1 of the unit stays); every entry of a list
    let all = t.trim_start().starts_with('[');
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 && (all || !*changed) => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.17, 0.83, 1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // a flow above the capacity of the stream is refused by the model: no figure at all, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = [("vf", "27.7778 m/s"), ("kj", "0.12 1/m"), ("q", "2 1/s")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "transport_greenshields_speed_at_flow", &given, None);
    assert!(!a.solved());
    assert!(figures(p, "transport_greenshields_speed_at_flow", &given, None, None).is_empty());
}
