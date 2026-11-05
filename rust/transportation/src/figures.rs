//! Transportation's own live figures (ax_engine::figkit): every capability's published figure kinds, drawn for the
//! visitor's inputs from what the model computed: traffic-flow diagrams traced by further solves of the model,
//! queues, signal timings, distributions and fits (this file), vehicle and road drawings labelled with the solved
//! values (figures_draw.rs). Written by hand for the Rust build. A figure the inputs make impossible is left out.

use crate::figures_draw as dr;
use crate::figures_plot::{put, Plot};
use ax_engine::figkit::{self as k, num, q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The figures for one solved request (Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // value fields over two inputs
        "transport_braking_distance" => vec![field2(s, "v", Rng::Near, "a", Rng::Near, "d",
                                                    "Braking distance over initial speed and deceleration")],
        "transport_skid_speed" => vec![field2(s, "d", Rng::Near, "f", Rng::Near, "v",
                                              "Speed estimated from skid marks over their length and the friction")],
        "transport_headway_probability" => vec![field2(s, "q", Rng::Decade, "h", Rng::Decade, "P_ge",
                                                       "Probability of a headway at least h over flow and headway")],
        "transport_md1_queue" => vec![field2(s, "mu", Rng::Near, "lam", Rng::Near, "Lq",
                                             "Mean number waiting (M/D/1) over service and arrival rates")],
        "transport_mm1_queue" => vec![field2(s, "mu", Rng::Near, "lam", Rng::Near, "rho",
                                             "Utilisation of a single random server (M/M/1) over service and arrival rates")],
        // traffic flow
        "transport_greenshields_capacity" => vec![gs_capacity(s), field2(s, "vf", Rng::Near, "kj", Rng::Near, "q_max",
                                                  "Greenshields capacity over free-flow speed and jam density")],
        "transport_greenshields_speed_at_flow" => vec![gs_speed_at_flow(s)],
        "transport_greenshields_shockwave" => vec![gs_shockwave(s)],
        "transport_greenshields_fit" => vec![gs_fit(s), data_xy(s)],
        "transport_greenberg_capacity" => vec![capacity_locus(s, "v0", "kj", "Greenberg")],
        "transport_underwood_capacity" => vec![capacity_locus(s, "vf", "k0", "Underwood")],
        "transport_greenberg_fit" => vec![states_fit(s, "Greenberg")],
        "transport_underwood_fit" => vec![states_fit(s, "Underwood")],
        "transport_shockwave_speed" => vec![shockwave(s)],
        // queues and signals
        "transport_bottleneck_queue" => vec![bottleneck(s)],
        "transport_fifo_queue_simulation" => vec![fifo(s)],
        "transport_interval_queue" => vec![interval_queue(s)],
        "transport_signal_queue_uniform_delay" => vec![signal_queue(s)],
        "transport_signal_queue_shockwave" => vec![timespace(s)],
        "transport_signal_green_splits" => vec![green_splits(s)],
        "transport_webster_optimum_cycle" => vec![webster(s)],
        "transport_erlang_b_blocking" => vec![erlang(s)],
        // choice, data and fits
        "transport_logit_mode_shares" => vec![logit_bars(s, false)],
        "transport_logit_from_attributes" => vec![logit_bars(s, true)],
        "transport_nested_logit" => vec![dr::nested_tree(s)],
        "transport_spot_speed_percentiles" => vec![spot_speeds(s)],
        "transport_time_and_space_mean_speed" => vec![mean_speeds(s)],
        "transport_travel_time_reliability" => vec![reliability(s)],
        "transport_trip_generation_regression" => vec![trip_fit(s)],
        // distances and profiles
        "transport_stopping_sight_distance" => vec![dr::breakdown_ssd(s), ssd_diagram(s)],
        "transport_passing_sight_distance" => vec![dr::breakdown_psd(s)],
        "transport_vehicle_resistance" => vec![dr::breakdown_resistance(s)],
        "transport_train_running_time" => vec![train_profile(s)],
        "transport_crest_curve_length" | "transport_crest_curve_sight_distance" => vec![crest_profile(s)],
        "transport_sag_curve_headlight" => vec![sag_profile(s)],
        "transport_clothoid_coordinates" => vec![clothoid(s)],
        "transport_quarter_car_transmissibility" => vec![transmissibility(s)],
        // drawings
        _ => dr::live(s),
    };
    v.into_iter().flatten().collect()
}

pub fn f(x: f64) -> Input {
    Input::Float(x)
}

// ------------------------------------------------------------------------------------------------ fields
enum Rng {
    /// half to one and a half times the value
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
}

fn in_unit(s: &Solved, sym: &str) -> String {
    let kind = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.kind.clone());
    match kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())) {
        Some(u) if u != "1" => u,
        _ => String::new(),
    }
}

fn axis_label(s: &Solved, sym: &str) -> String {
    let u = in_unit(s, sym);
    let meaning = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym))
        .map(|i| i.meaning.split(" (").next().unwrap_or("").to_string()).unwrap_or_default();
    let base = if meaning.is_empty() || meaning.len() > 30 { sym.to_string() } else { format!("{sym}, {meaning}") };
    if u.is_empty() { base } else { format!("{base} ({u})") }
}

fn field2(s: &Solved, xs_: &str, xr: Rng, ys_: &str, yr: Rng, out: &str, title: &str) -> Option<Figure> {
    let (x0, y0) = (s.inp(xs_)?, s.inp(ys_)?);
    let z0 = s.out(out)?;
    let range = |v: f64, r: &Rng| -> Option<(f64, f64, bool)> {
        match r {
            Rng::Near if v != 0.0 => Some(((v * 0.5).min(v * 1.5), (v * 0.5).max(v * 1.5), false)),
            Rng::Decade if v > 0.0 => Some((v / 10.0, v * 10.0, true)),
            _ => None,
        }
    };
    let (xl, xh, xlog) = range(x0, &xr)?;
    let (yl, yh, ylog) = range(y0, &yr)?;
    let xs = k::span(xl, xh, 13, xlog);
    let ys = k::span(yl, yh, 11, ylog);
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with(&[(xs_, *x), (ys_, *y)]).and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let meaning = s.p.procedure(s.key).and_then(|p| p.outputs.iter().find(|o| o.symbol == out))
        .map(|o| o.meaning.clone()).unwrap_or_else(|| out.to_string());
    let body = k::field(&xs, &ys, &vals, (x0, y0), &axis_label(s, xs_), &axis_label(s, ys_),
                        &format!("{out} = {}", num(z0)), (xlog, ylog));
    Some(s.chart("field", title,
                 &format!("{meaning} ({out}) over {xs_} from {} to {} and {ys_} from {} to {}{}: each of the {n} cells is \
                           one computation of the model, shaded by its value (darker is larger; a blank cell: inputs \
                           the model refuses). Marked: the inputs given, {out} = {}.", num(xl), num(xh), num(yl), num(yh),
                          if xlog || ylog { " (logarithmic axes)" } else { "" }, num(z0)), &body))
}

// ------------------------------------------------------------------------------------------------ traffic flow
/// The Greenshields flow-density curve traced by the model: for flows up to the capacity it returns, the two
/// densities that carry each flow (speed_at_flow). Points (k veh/km, q veh/h); the capacity (q_max, k_m, v_m).
fn gs_curve(s: &Solved, vf: f64, kj: f64) -> Option<(Vec<(f64, f64)>, (f64, f64, f64))> {
    let cap = s.solve_other("transport_greenshields_capacity", &[("vf", f(vf)), ("kj", f(kj))])?;
    let (qm, km, vm) = (k::num_of(&cap, "q_max")?, k::num_of(&cap, "k_m")?, k::num_of(&cap, "v_m")?);
    let mut up = vec![];
    let mut down = vec![];
    for i in 1..=24 {
        let t = i as f64 / 24.0;
        let qh = qm * (1.0 - (1.0 - t).powi(2)) * (1.0 - 1e-9);
        if let Some(a) = s.solve_other("transport_greenshields_speed_at_flow",
                                       &[("vf", f(vf)), ("kj", f(kj)), ("q", f(qh / 3600.0))]) {
            if let (Some(k1), Some(k2)) = (k::num_of(&a, "k_1"), k::num_of(&a, "k_2")) {
                up.push((k1, qh));
                down.push((k2, qh));
            }
        }
    }
    if up.len() < 4 {
        return None;
    }
    down.reverse();
    up.extend(down);
    Some((up, (qm, km, vm)))
}

/// The flow of a Greenshields state of density k (SI): its density times the speed the model returns for it (the
/// shockwave capability from a nearly empty road, whose speed is the speed of the state).
fn gs_state(s: &Solved, vf: f64, kj: f64, kk: f64) -> Option<(f64, f64)> {
    let a = s.solve_other("transport_greenshields_shockwave", &[("vf", f(vf)), ("kj", f(kj)), ("k1", f(kk * 1e-9)),
                                                                ("k2", f(kk))])?;
    let u = k::num_of(&a, "u")?;
    Some((kk * 1000.0, kk * 1000.0 * u))
}

fn gs_capacity(s: &Solved) -> Option<Figure> {
    let (vf, kj) = (s.inp("vf")?, s.inp("kj")?);
    let (qm, km) = (s.out("q_max")?, s.out("k_m")?);
    let (curve, _) = gs_curve(s, vf, kj)?;
    let xs: Vec<f64> = curve.iter().map(|p| p.0).chain([0.0]).collect();
    let ys: Vec<f64> = curve.iter().map(|p| p.1).chain([0.0]).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.legend_bottom = true;
    p.legend_center = true;
    p.area(&[(curve[0].0, 0.0)].iter().chain(curve.iter()).chain([(curve[curve.len() - 1].0, 0.0)].iter()).copied()
        .collect::<Vec<_>>(), "axp-a");
    p.line(&curve, "axp-c", "flow-density curve (each point solved)");
    p.dashed(&[(km, 0.0), (km, qm)], "");
    p.mark(km, qm, &format!("capacity {} veh/h at {} veh/km", num(qm), num(km)), true);
    let body = p.finish("k, density (veh/km)", "q, flow (veh/h)", &format!("vf = {} km/h, kj = {} veh/km",
                                                                         num(vf * 3.6), num(kj * 1000.0)));
    Some(s.chart("diagram", "The flow-density fundamental diagram and capacity",
                 &format!("Greenshields fundamental diagram for free-flow speed {} km/h and jam density {} veh/km: for \
                           24 flows up to the capacity the model returns the two densities that carry each flow \
                           (uncongested left, congested right). Marked: the capacity it returned, q = {} veh/h at the \
                           optimum density k = {} veh/km.", num(vf * 3.6), num(kj * 1000.0), num(qm), num(km)), &body))
}

fn gs_speed_at_flow(s: &Solved) -> Option<Figure> {
    let (vf, kj, qq) = (s.inp("vf")?, s.inp("kj")?, s.inp("q")?);
    let (k1, v1, k2, v2) = (s.out("k_1")?, s.out("v_1")?, s.out("k_2")?, s.out("v_2")?);
    let (curve, _) = gs_curve(s, vf, kj)?;
    let qh = qq * 3600.0;
    let xs: Vec<f64> = curve.iter().map(|p| p.0).chain([0.0]).collect();
    let ys: Vec<f64> = curve.iter().map(|p| p.1).chain([0.0, qh]).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.legend_bottom = true;
    p.legend_center = true;
    p.line(&curve, "axp-c", "flow-density curve");
    p.dashed(&[(0.0, qh), (xs.iter().cloned().fold(0.0, f64::max), qh)], "the flow given");
    p.dots(&[(k1, qh), (k2, qh)], "axp-d2", "");
    p.value(k1, qh, "end", &format!("{} km/h ", num(v1)));
    p.mark(k2, qh, &format!("{} km/h", num(v2)), false);
    let body = p.finish("k, density (veh/km)", "q, flow (veh/h)",
                        &format!("q = {} veh/h: k1 = {}, k2 = {} veh/km", num(qh), num(k1), num(k2)));
    Some(s.chart("diagram", "Two traffic states carrying the same flow",
                 &format!("Greenshields flow-density curve (vf = {} km/h, kj = {} veh/km), each point solved by the \
                           model, cut by the flow q = {} veh/h: the two states it returned, k1 = {} veh/km at {} km/h \
                           (uncongested) and k2 = {} veh/km at {} km/h (congested).", num(vf * 3.6), num(kj * 1000.0),
                          num(qh), num(k1), num(v1), num(k2), num(v2)), &body))
}

fn gs_shockwave(s: &Solved) -> Option<Figure> {
    let (vf, kj, k1, k2) = (s.inp("vf")?, s.inp("kj")?, s.inp("k1")?, s.inp("k2")?);
    let u = s.out("u")?;
    let (curve, _) = gs_curve(s, vf, kj)?;
    let a = gs_state(s, vf, kj, k1)?;
    let b = gs_state(s, vf, kj, k2)?;
    let xs: Vec<f64> = curve.iter().map(|p| p.0).chain([0.0]).collect();
    let ys: Vec<f64> = curve.iter().map(|p| p.1).chain([0.0]).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.legend_bottom = true;
    p.legend_center = true;
    p.line(&curve, "axp-c", "flow-density curve");
    p.line(&[a, b], "axp-c2", &format!("shockwave: slope u = {} km/h", num(u)));
    p.dots(&[a, b], "axp-d2", "");
    p.label(a.0, a.1 + 0.0, "end", "upstream ");
    p.mark(b.0, b.1, "downstream", true);
    let body = p.finish("k, density (veh/km)", "q, flow (veh/h)", &format!("u = {} km/h", num(u)));
    Some(s.chart("diagram", "Shockwave as a chord of the fundamental diagram",
                 &format!("Greenshields flow-density curve (vf = {} km/h, kj = {} veh/km), each point solved by the \
                           model, with the upstream state k1 = {} and the downstream state k2 = {} veh/km (their flows: \
                           density times the speed the model returns for each state): the chord joining them has the \
                           slope of the returned shockwave speed u = {} km/h.", num(vf * 3.6), num(kj * 1000.0),
                          num(k1 * 1000.0), num(k2 * 1000.0), num(u)), &body))
}

fn obs(s: &Solved) -> Option<Vec<(f64, f64)>> {
    let (v, kk) = (s.vec("speeds")?, s.vec("densities")?);
    (v.len() == kk.len() && !v.is_empty()).then(|| kk.into_iter().zip(v).collect())
}

fn gs_fit(s: &Solved) -> Option<Figure> {
    let data = obs(s)?;
    let (vf, kj, qm, r2) = (s.out("vf")?, s.out("kj")?, s.out("q_max")?, s.out("r2")?);
    if !(vf > 0.0 && kj > 0.0 && qm > 0.0) {
        return None;
    }
    let mut up = vec![];
    let mut down = vec![];
    for i in 1..=30 {
        let t = i as f64 / 30.0;
        let qh = qm * (1.0 - (1.0 - t).powi(2)) * (1.0 - 1e-9);
        if let Some(a) = s.solve_other("transport_greenshields_speed_at_flow",
                                       &[("vf", f(vf / 3.6)), ("kj", f(kj / 1000.0)), ("q", f(qh / 3600.0))]) {
            if let (Some(k1), Some(v1), Some(k2), Some(v2)) = (k::num_of(&a, "k_1"), k::num_of(&a, "v_1"),
                                                               k::num_of(&a, "k_2"), k::num_of(&a, "v_2")) {
                up.push((k1, v1));
                down.push((k2, v2));
            }
        }
    }
    if up.len() < 4 {
        return None;
    }
    down.reverse();
    up.extend(down);
    let xs: Vec<f64> = up.iter().chain(data.iter()).map(|p| p.0).collect();
    let ys: Vec<f64> = up.iter().chain(data.iter()).map(|p| p.1).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.line(&up, "axp-c", &format!("fitted model, R² = {}", num(r2)));
    p.dots(&data, "axp-d2", "observed (input)");
    let mid = up[up.len() / 2];
    p.mark(mid.0, mid.1, &format!("capacity {} veh/h", num(qm)), true);
    let body = p.finish("k, density (veh/km)", "v, speed (km/h)", &format!("vf = {} km/h, kj = {} veh/km", num(vf), num(kj)));
    Some(s.chart("fit", "Speed-density data and the fitted traffic-flow model",
                 &format!("{} observed speed-density pairs and the linear speed-density model fitted to them: free-flow \
                           speed vf = {} km/h, jam density kj = {} veh/km, R² = {}; the line is traced by the model's \
                           two speeds at 30 flows up to the capacity {} veh/h (marked near the capacity state).",
                          data.len(), num(vf), num(kj), num(r2), num(qm)), &body))
}

fn data_xy(s: &Solved) -> Option<Figure> {
    let data = obs(s)?;
    let xs: Vec<f64> = data.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = data.iter().map(|p| p.1).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (false, false));
    p.dots(&data, "axp-d", "data (input)");
    let mut parts = vec![];
    for o in ["vf", "kj", "q_max", "r2"] {
        if let Some(v) = s.out(o) {
            parts.push(format!("{o} = {}", num(v)));
        }
    }
    let body = p.finish("densities (veh/km)", "speeds (km/h)", &parts.join(", "));
    Some(s.chart("xy", "Speed-density observations and the values returned",
                 &format!("The speeds against the densities as given ({} points). Returned: {}.", data.len(),
                          parts.join(", ")), &body))
}

/// Greenberg or Underwood: the capacity state (k_m, q_max) the model returns as the density parameter and as the speed
/// parameter vary around the given ones; the given one marked.
fn capacity_locus(s: &Solved, vsym: &str, ksym: &str, name: &str) -> Option<Figure> {
    let (v0, k0) = (s.inp(vsym)?, s.inp(ksym)?);
    let (qm, km) = (s.out("q_max")?, s.out("k_m")?);
    let mut by_k = vec![];
    for kk in k::span(k0 * 0.5, k0 * 2.0, 16, true) {
        if let Some(a) = s.with(&[(ksym, kk)]) {
            if let (Some(x), Some(y)) = (k::num_of(&a, "k_m"), k::num_of(&a, "q_max")) {
                by_k.push((x, y));
            }
        }
    }
    let mut by_v = vec![];
    for vv in k::span(v0 * 0.5, v0 * 2.0, 16, true) {
        if let Some(a) = s.with(&[(vsym, vv)]) {
            if let (Some(x), Some(y)) = (k::num_of(&a, "k_m"), k::num_of(&a, "q_max")) {
                by_v.push((x, y));
            }
        }
    }
    if by_k.len() < 2 {
        return None;
    }
    let xs: Vec<f64> = by_k.iter().chain(by_v.iter()).map(|p| p.0).collect();
    let ys: Vec<f64> = by_k.iter().chain(by_v.iter()).map(|p| p.1).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.legend_left = true;
    p.line(&by_k, "axp-c", &format!("{ksym} from half to twice the given"));
    p.dots(&by_k, "axp-d", "");
    if by_v.len() >= 2 {
        p.line(&by_v, "axp-c3", &format!("{vsym} from half to twice the given"));
    }
    p.mark(km, qm, &format!("capacity {} veh/h", num(qm)), true);
    let body = p.finish("k, density at capacity (veh/km)", "q, capacity (veh/h)",
                        &format!("{vsym} = {} km/h, {ksym} = {} veh/km", num(v0 * 3.6), num(k0 * 1000.0)));
    Some(s.chart("diagram", &format!("{name} model: capacity states in the flow-density plane"),
                 &format!("Capacity of the {name} model in the flow-density plane: the capacity and the density at \
                           capacity the model returns for {ksym} from half to twice the given value (one curve) and for \
                           {vsym} from half to twice the given value (the other), 32 solves. Marked: the given \
                           parameters, capacity {} veh/h at {} veh/km.", num(qm), num(km)), &body))
}

/// Greenberg or Underwood fit: the observations and the fitted model's characteristic states as the model returns
/// them (capacity state from the capacity capability, free-flow or jam end from the fitted parameters).
fn states_fit(s: &Solved, name: &str) -> Option<Figure> {
    let data = obs(s)?;
    let (qm, r2) = (s.out("q_max")?, s.out("r2")?);
    let mut pts = data.clone();
    let mut p_marks: Vec<((f64, f64), String)> = vec![];
    let header;
    if name == "Greenberg" {
        let (v0, kj) = (s.out("v0")?, s.out("kj")?);
        let cap = s.solve_other("transport_greenberg_capacity", &[("v0", f(v0 / 3.6)), ("kj", f(kj / 1000.0))])?;
        let km = k::num_of(&cap, "k_m")?;
        p_marks.push(((km, v0), format!("v0 = {} km/h", num(v0))));
        p_marks.push(((kj, 0.0), format!("kj = {} veh/km", num(kj))));
        header = format!("v0 = {} km/h, kj = {} veh/km", num(v0), num(kj));
    } else {
        let (vf, k0) = (s.out("vf")?, s.out("k0")?);
        let cap = s.solve_other("transport_underwood_capacity", &[("vf", f(vf / 3.6)), ("k0", f(k0 / 1000.0))])?;
        let (km, vm) = (k::num_of(&cap, "k_m")?, k::num_of(&cap, "v_m")?);
        p_marks.push(((0.0, vf), format!("vf = {} km/h", num(vf))));
        p_marks.push(((km, vm), format!("at capacity {} km/h", num(vm))));
        header = format!("vf = {} km/h, k0 = {} veh/km", num(vf), num(k0));
    }
    pts.extend(p_marks.iter().map(|m| m.0));
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.legend_bottom = true;
    p.legend_left = true;
    p.dots(&data, "axp-d", "observations (input)");
    p.dots(&p_marks.iter().map(|m| m.0).collect::<Vec<_>>(), "axp-d2", &format!("fitted {name} states, r² = {}", num(r2)));
    for (i, (pt, t)) in p_marks.iter().enumerate() {
        if i == 0 {
            p.mark(pt.0, pt.1, t, false);
        } else {
            p.value(pt.0, pt.1 - 0.0, "end", &format!("{t} "));
        }
    }
    let body = p.finish("k, density (veh/km)", "v, speed (km/h)", &format!("{header}, capacity {} veh/h", num(qm)));
    Some(s.chart("fit", &format!("{name} model fitted to speed-density data"),
                 &format!("Speed-density observations ({} pairs) and the states of the fitted {name} model: {header} \
                           (r² = {}), capacity {} veh/h; the capacity state is the one the model's capacity capability \
                           returns for the fitted parameters.", data.len(), num(r2), num(qm)), &body))
}

fn shockwave(s: &Solved) -> Option<Figure> {
    let (q1, k1, q2, k2) = (s.inp("q1")? * 3600.0, s.inp("k1")? * 1000.0, s.inp("q2")? * 3600.0, s.inp("k2")? * 1000.0);
    let u = s.out("u")?;
    let dir = s.out_text("direction").unwrap_or_default();
    let xs = [0.0, k1, k2];
    let ys = [0.0, q1, q2];
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.legend_bottom = true;
    p.dashed(&[(0.0, 0.0), (k1, q1)], "state speeds (rays from the origin)");
    p.dashed(&[(0.0, 0.0), (k2, q2)], "");
    p.line(&[(k1, q1), (k2, q2)], "axp-c", &format!("shockwave: slope u = {} km/h", num(u)));
    p.dots(&[(k1, q1), (k2, q2)], "axp-d", "");
    p.value(k1, q1 - 0.0, "start", "  upstream");
    p.mark(k2, q2, "downstream", false);
    let body = p.finish("k, density (veh/km)", "q, flow (veh/h)", &format!("u = {} km/h, {dir}", num(u)));
    Some(s.chart("diagram", "Shockwave between two traffic states",
                 &format!("Two traffic states on flow-density axes: upstream q1 = {} veh/h at k1 = {} veh/km and \
                           downstream q2 = {} veh/h at k2 = {} veh/km (dashed rays: their speeds). The chord between \
                           them has the slope of the returned shockwave speed u = {} km/h, moving {dir}.", num(q1),
                          num(k1), num(q2), num(k2), num(u)), &body))
}

// ------------------------------------------------------------------------------------------------ queues
fn bottleneck(s: &Solved) -> Option<Figure> {
    let t_r = s.inp("t_r")?;
    let (qmax, tt, dtot) = (s.out("Q_max")?, s.out("T")?, s.out("D_total")?);
    if !(qmax > 0.0 && tt > t_r && t_r > 0.0) {
        return None;
    }
    let mut pts = vec![(0.0, 0.0)];
    for t in k::span(t_r / 16.0, t_r, 16, false) {
        let a = s.with(&[("t_r", t)])?;
        pts.push((t / 60.0, k::num_of(&a, "Q_max")?));
    }
    pts.push((tt / 60.0, 0.0));
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.area(&pts, "axp-a");
    p.line(&pts, "axp-c", "queue (each point solved)");
    p.dashed(&[(t_r / 60.0, 0.0), (t_r / 60.0, qmax)], "end of the reduction");
    p.legend_bottom = true;
    p.legend_center = true;
    p.mark(t_r / 60.0, qmax, &format!("Qmax = {} veh", num(qmax)), true);
    p.label(tt / 60.0, 0.0, "end", &format!("queue ends {} min", num(tt / 60.0)));
    let body = p.finish("t, time from the start of the reduction (min)", "vehicles queued",
                        &format!("total delay {} veh·h (shaded)", num(dtot)));
    Some(s.chart("queue", "Bottleneck queue: its growth and dissipation",
                 &format!("Queue at a bottleneck whose capacity is reduced for {} min: the queue at each moment of the \
                           reduction is the longest queue the model returns for a reduction ending then (16 solves); it \
                           then dissipates until the returned end of the queue, {} min. Marked: the longest queue {} veh; \
                           the shaded area is the total delay, {} veh·h.", num(t_r / 60.0), num(tt / 60.0), num(qmax),
                          num(dtot)), &body))
}

fn fifo(s: &Solved) -> Option<Figure> {
    let arr = s.vec("arrivals")?;
    let svc0 = s.vec("service")?;
    let waits = s.out_list("waits")?;
    let n = arr.len();
    if n == 0 || n > 16 || waits.len() != n {
        return None;
    }
    let svc: Vec<f64> = if svc0.len() == 1 { vec![svc0[0]; n] } else if svc0.len() == n { svc0 } else { return None };
    let (maxw, last) = (s.out("max_wait")?, s.out("last_departure")?);
    let xs: Vec<f64> = vec![0.0, last];
    let ys: Vec<f64> = vec![0.5, n as f64 + 0.5];
    let mut p = Plot::new(&xs, &ys, false, false, (true, false));
    let iw = waits.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))?.0;
    for i in 0..n {
        let y = i as f64 + 1.0;
        if waits[i] > 0.0 {
            p.rect(arr[i], y - 0.32, arr[i] + waits[i], y + 0.32, "axp-b5");
        }
        p.rect(arr[i] + waits[i], y - 0.32, arr[i] + waits[i] + svc[i], y + 0.32, "axp-b");
    }
    p.dots(&arr.iter().enumerate().map(|(i, a)| (*a, i as f64 + 1.0)).collect::<Vec<_>>(), "axp-d2", "arrival (input)");
    p.legend_bottom = true;
    p.mark(arr[iw] + waits[iw], iw as f64 + 1.0, &format!("longest wait {}", num(maxw)), iw > 0);
    let body = p.finish("t, time (s)", "customer, in order of arrival",
                        &format!("pale: waiting, dark: in service; mean wait {}", num(s.out("mean_wait").unwrap_or(f64::NAN))));
    Some(s.chart("queue", "FIFO queue: arrivals, waits and service",
                 &format!("A first-in-first-out queue of {n} customers: each row starts at its arrival time (input), \
                           waits for the time the model returned (pale) and is then served for its service time (dark). \
                           Marked: the longest wait {}; also returned: mean wait {}, mean time in the system {}, last \
                           departure {}, longest queue {}.", num(maxw), num(s.out("mean_wait")?),
                          num(s.out("mean_time_in_system")?), num(last), num(s.out("max_queue")?)), &body))
}

fn interval_queue(s: &Solved) -> Option<Figure> {
    let arr = s.vec("arrivals")?;
    let queue = s.out_list("queue")?;
    let served = s.out_list("served")?;
    let qmax = s.out("Q_max")?;
    let n = arr.len();
    if n == 0 || n > 24 || queue.len() != n || served.len() != n {
        return None;
    }
    let labels: Vec<String> = (1..=n).map(|i| i.to_string()).collect();
    let g = vec![("arrivals (input)".to_string(), arr.iter().map(|x| Some(*x)).collect()),
                 ("served".to_string(), served.iter().map(|x| Some(*x)).collect()),
                 ("queue at the end".to_string(), queue.iter().map(|x| Some(*x)).collect())];
    let cap = s.vec("capacity").map(|c| c.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")).unwrap_or_default();
    let body = k::bar_groups(&labels, &g, "interval", "vehicles", &format!("Qmax = {} veh, capacity {}", num(qmax),
                                                                            ax_clip(&cap, 20)));
    Some(s.chart("queue", "Queue from interval arrivals and capacity",
                 &format!("Arrivals per interval (input) against a capacity of [{cap}] per interval: the vehicles served \
                           and the queue left at the end of each interval as the model returned them. Longest queue {} \
                           vehicles.", num(qmax)), &body))
}

fn ax_clip(t: &str, n: usize) -> String {
    crate::figures_plot::clip(t, n)
}

fn signal_queue(s: &Solved) -> Option<Figure> {
    let (r, g) = (s.inp("r")?, s.inp("g")?);
    let (qmax, tc, dtot) = (s.out("Q_max")?, s.out("t_clear")?, s.out("D_total")?);
    if !(qmax > 0.0 && tc >= 0.0 && tc <= g * (1.0 + 1e-9)) {
        return None;
    }
    let pts = vec![(0.0, 0.0), (r, qmax), (r + tc, 0.0), (r + g, 0.0)];
    let mut p = Plot::new(&[0.0, r + g], &[0.0, qmax * 1.15], false, false, (true, true));
    p.rect(0.0, qmax * 1.08, r, qmax * 1.13, "axp-b");
    p.rect(r, qmax * 1.08, r + g, qmax * 1.13, "axp-b6");
    p.area(&pts[..3], "axp-a");
    p.line(&pts, "axp-c", "");
    p.label(r / 2.0, qmax * 1.03, "middle", "red");
    p.label(r + g / 2.0, qmax * 1.03, "middle", "green");
    p.mark(r, qmax, &format!("{} veh", num(qmax)), false);
    p.label(r + tc, 0.0, "start", &format!(" cleared {} s into green", num(tc)));
    let body = p.finish("t, time in the cycle (s)", "queue (vehicles)", &format!("total delay {} veh·s (shaded)", num(dtot)));
    Some(s.chart("queue", "Queue at a traffic signal over one cycle",
                 &format!("Queue at a signal over one cycle (red {} s, green {} s): it grows during red to the largest \
                           queue the model returned, {} vehicles, and clears {} s into green; the shaded area is the \
                           total delay {} veh·s ({} s per vehicle, {} % stopped).", num(r), num(g), num(qmax), num(tc),
                          num(dtot), num(s.out("d_uniform")?), num(s.out("P_stopped")? * 100.0)), &body))
}

fn timespace(s: &Solved) -> Option<Figure> {
    let r = s.inp("r")?;
    let (lmax, tmax, wq, wd) = (s.out("L_max")?, s.out("t_max")?, s.out("w_queue")?, s.out("w_discharge")?);
    if !(lmax > 0.0 && tmax > r) {
        return None;
    }
    let tend = tmax * 1.25;
    let mut p = Plot::new(&[0.0, tend], &[-lmax * 1.25, lmax * 0.08], false, false, (true, false));
    p.legend_bottom = true;
    p.legend_left = true;
    p.area(&[(0.0, 0.0), (r, 0.0), (tmax, -lmax)], "axp-a");
    p.rect(0.0, -lmax * 0.015, r, lmax * 0.035, "axp-b");
    p.rect(r, -lmax * 0.015, tend, lmax * 0.035, "axp-b6");
    p.line(&[(0.0, 0.0), (tmax, -lmax)], "axp-c", &format!("queue tail, {} km/h upstream", num(wq.abs())));
    p.line(&[(r, 0.0), (tmax, -lmax)], "axp-c2", &format!("discharge wave, {} km/h upstream", num(wd.abs())));
    p.label(r / 2.0, lmax * 0.05, "middle", "red");
    p.label((r + tend) / 2.0, lmax * 0.05, "middle", "green");
    p.mark(tmax, -lmax, &format!("{} m at {} s", num(lmax), num(tmax)), true);
    let body = p.finish("t, time since the start of red (s)", "position from the stop line (m)",
                        "shaded: the stopped queue");
    Some(s.chart("timespace", "Time-space diagram of a signal queue",
                 &format!("Time-space diagram of the queue at a signal (red {} s): the queue tail moves upstream at {} \
                           km/h and the discharge wave from the start of green at {} km/h, both returned by the model; \
                           where they meet the queue is longest. Marked: t = {} s, longest queue {} m.", num(r),
                          num(wq.abs()), num(wd.abs()), num(tmax), num(lmax)), &body))
}

/// A cycle drawn as one strip: (label, seconds, value class) segments laid end to end, to scale.
fn strip(c: &mut Canvas, segs: &[(String, f64, &str)], total: f64, y: f64, marks: &[(f64, String)]) {
    let (x0, x1) = (40.0, 480.0);
    let sx = |t: f64| x0 + (x1 - x0) * t / total;
    let mut t = 0.0;
    for (i, (lab, d, class)) in segs.iter().enumerate() {
        let (a, b) = (sx(t), sx(t + d));
        c.raw(&format!("<rect class=\"{class}\" x=\"{a:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"40\"/>\
                        <rect class=\"axp-f\" x=\"{a:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"40\"/>", (b - a).max(0.5),
                       (b - a).max(0.5)));
        let up = i % 2 == 0;
        put(c, (a + b) / 2.0, if up { y - 8.0 } else { y + 58.0 }, "middle", "axp-lt", lab);
        put(c, (a + b) / 2.0, y + 25.0, "middle", "axp-vt", &q(*d, "s"));
        t += d;
    }
    // the time axis
    let step = [1.0, 2.0, 5.0, 10.0, 20.0, 25.0, 50.0, 100.0, 200.0, 500.0, 1000.0].iter().copied()
        .find(|s| total / s <= 8.0).unwrap_or(total / 5.0);
    let mut v = 0.0;
    while v <= total + 1e-9 {
        let x = sx(v);
        c.raw(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>", y + 66.0, y + 72.0));
        put(c, x, y + 88.0, "middle", "axp-t", &num(v));
        v += step;
    }
    c.raw(&format!("<line class=\"axp-g\" x1=\"{x0}\" y1=\"{:.1}\" x2=\"{x1}\" y2=\"{:.1}\"/>", y + 66.0, y + 66.0));
    for (tm, lab) in marks {
        let x = sx(*tm);
        c.dashed((x, y - 26.0), (x, y + 66.0));
        put(c, x, y - 32.0, "middle", "axp-mt", lab);
    }
}

fn green_splits(s: &Solved) -> Option<Figure> {
    let (cc, l) = (s.inp("C")?, s.inp("L")?);
    let y = s.vec("y")?;
    let g = s.out_list("g")?;
    let xc = s.out("X_c")?;
    if g.len() != y.len() || g.is_empty() || g.len() > 8 {
        return None;
    }
    let mut segs: Vec<(String, f64, &str)> = g.iter().zip(&y).enumerate()
        .map(|(i, (gi, yi))| (format!("phase {} (y = {})", i + 1, num(*yi)), *gi, "axp-b6")).collect();
    if l > 0.0 {
        segs.push(("lost time".into(), l, "axp-ka"));
    }
    let total: f64 = segs.iter().map(|x| x.1).sum::<f64>().max(cc);
    let mut c = Canvas::new(230.0);
    put(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("C = {}, Xc = {}", q(cc, "s"), num(xc)));
    strip(&mut c, &segs, total, 80.0, &[(cc, format!("C = {}", q(cc, "s")))]);
    put(&mut c, 260.0, 220.0, "middle", "axp-lt", "time in the cycle (s)");
    Some(s.figure("timing", "Signal timing: green splits over one cycle",
                  &format!("Green splits of a {}-phase signal with cycle C = {} s and total lost time L = {} s: the \
                            effective greens the model returned ({} s) laid end to end in green, then the lost time. \
                            Critical degree of saturation Xc = {}.", g.len(), num(cc), num(l),
                           g.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(xc)), c.body(), c.h))
}

fn webster(s: &Solved) -> Option<Figure> {
    let l = s.inp("L")?;
    let y = s.vec("y")?;
    let (c0, yy, cmin) = (s.out("C0")?, s.out("Y")?, s.out("C_min")?);
    let a = s.solve_other("transport_signal_green_splits", &[("C", f(c0)), ("L", f(l)),
                                                             ("y", Input::Json(serde_json::json!(y)))])?;
    let g = k::list_of(&a, "g")?;
    if g.len() != y.len() || g.len() > 8 {
        return None;
    }
    let mut segs: Vec<(String, f64, &str)> = g.iter().enumerate()
        .map(|(i, gi)| (format!("phase {}", i + 1), *gi, "axp-b6")).collect();
    if l > 0.0 {
        segs.push(("lost time".into(), l, "axp-ka"));
    }
    let total: f64 = segs.iter().map(|x| x.1).sum::<f64>().max(c0);
    let mut c = Canvas::new(230.0);
    put(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("C0 = {}, Y = {}", q(c0, "s"), num(yy)));
    strip(&mut c, &segs, total, 80.0, &[(cmin, format!("minimum {}", q(cmin, "s")))]);
    put(&mut c, 260.0, 220.0, "middle", "axp-lt", "time in the cycle (s)");
    Some(s.figure("timing", "Webster's optimum cycle and its greens",
                  &format!("Webster's optimum cycle for lost time L = {} s: the returned C0 = {} s (Y = {}), its effective \
                            greens ({} s) as the model's green-split capability returns them for that cycle, then the lost \
                            time; marked: the returned minimum cycle {} s.", num(l), num(c0), num(yy),
                           g.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(cmin)), c.body(), c.h))
}

fn erlang(s: &Solved) -> Option<Figure> {
    let lam = s.inp("lam")?;
    let c0 = s.val("c")?.int();
    let pb = s.out("P_block")?;
    if c0 < 1 {
        return None;
    }
    let xs = k::span(lam * 0.5, lam * 1.5, 31, false);
    let mut series = vec![];
    let mut cs = vec![(c0, format!("c = {c0} (given)"))];
    for c in [(c0 / 2).max(1), c0 * 2] {
        if cs.iter().all(|x| x.0 != c) {
            cs.push((c, format!("c = {c}")));
        }
    }
    for (c, nm) in cs {
        let mut pts = vec![];
        for x in &xs {
            if let Some(a) = s.with_input(&[("lam", f(*x)), ("c", Input::Int(c))]) {
                if let Some(y) = k::num_of(&a, "P_block") {
                    if y > 0.0 {
                        pts.push((x * 3600.0, y));
                    }
                }
            }
        }
        if pts.len() >= 2 {
            series.push((nm, pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = k::lines(&series, "arrival rate (veh/h)", "P_block, probability turned away",
                        &format!("P_block = {}", num(pb)), Some((lam * 3600.0, pb)), &num(pb), false);
    Some(s.chart("curve-2", "Blocking probability with no waiting room (Erlang B)",
                 &format!("Probability an arrival is turned away against the arrival rate from {} to {} veh/h, one curve \
                           for each number of spaces (the one given, half and twice it), 31 points per curve computed by \
                           the model; marked: the inputs given, P_block = {}.", num(lam * 1800.0), num(lam * 5400.0),
                          num(pb)), &body))
}

// ------------------------------------------------------------------------------------------------ choice and data
fn logit_bars(s: &Solved, attrs: bool) -> Option<Figure> {
    let pr = s.out_list("P")?;
    let v = if attrs { s.out_list("V")? } else { s.vec("V")? };
    if pr.len() != v.len() || pr.is_empty() || pr.len() > 10 {
        return None;
    }
    let who = if attrs { "alt" } else { "mode" };
    let labels: Vec<String> = (1..=v.len()).map(|i| format!("{who} {i}")).collect();
    let vals: Vec<f64> = pr.iter().map(|x| x * 100.0).collect();
    let best = vals.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))?.0;
    let vs = v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ");
    let header = if attrs { format!("utilities V = {vs}") } else {
        format!("V = {vs}; logsum {}", num(s.out("logsum")?))
    };
    let body = k::bars(&labels, &vals, &[best], "", "choice probability (%)", &header);
    let shares = pr.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ");
    Some(s.chart("bars", if attrs { "Logit choice from attributes: utilities and shares" } else {
        "Logit mode shares from utilities" },
                 &format!("Multinomial logit choice among {} alternatives: the utilities{} (above the bars) and the \
                           probabilities the model returned ({shares}).", pr.len(),
                          if attrs { " the model returned" } else { " given" }), &body))
}

fn spot_speeds(s: &Solved) -> Option<Figure> {
    let mut v = s.vec("speeds")?;
    let (p15, p50, p85) = (s.out("p15")?, s.out("p50")?, s.out("p85")?);
    if v.len() < 2 {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = v.len() as f64;
    let mut step = vec![];
    for (i, x) in v.iter().enumerate() {
        step.push((*x, 100.0 * i as f64 / n));
        step.push((*x, 100.0 * (i + 1) as f64 / n));
    }
    let xs: Vec<f64> = v.iter().copied().chain([p15, p85]).collect();
    let mut p = Plot::new(&xs, &[0.0, 100.0], false, false, (false, true));
    p.legend_bottom = true;
    p.line(&step, "axp-c", "observed speeds (input), cumulative");
    for (pv, pc, nm) in [(p15, 15.0, "15th"), (p50, 50.0, "50th"), (p85, 85.0, "85th")] {
        p.dashed(&[(pv, 0.0), (pv, pc)], "");
        p.mark(pv, pc, &format!("{nm}: {} km/h", num(pv)), pc > 50.0);
    }
    let body = p.finish("spot speed (km/h)", "cumulative percent of vehicles", "percentiles returned");
    Some(s.chart("distribution", "Spot speeds: 15th, 50th and 85th percentiles",
                 &format!("Cumulative distribution of {} spot speeds (input). Marked: the 15th, 50th and 85th percentile \
                           speeds the model returned, {}, {} and {} km/h.", v.len(), num(p15), num(p50), num(p85)), &body))
}

fn mean_speeds(s: &Solved) -> Option<Figure> {
    let v = s.vec("speeds")?;
    let (tms, sms, sd) = (s.out("TMS")?, s.out("SMS")?, s.out("s")?);
    if v.is_empty() || v.len() > 60 {
        return None;
    }
    let pts: Vec<(f64, f64)> = v.iter().enumerate().map(|(i, x)| (*x, i as f64 + 1.0)).collect();
    let n = v.len() as f64;
    let xs: Vec<f64> = v.iter().copied().chain([tms, sms]).collect();
    let mut p = Plot::new(&xs, &[0.5, n + 1.6], false, false, (false, false));
    p.dots(&pts, "axp-d", "");
    p.line(&[(tms, 0.5), (tms, n + 0.5)], "axp-c2", "");
    p.line(&[(sms, 0.5), (sms, n + 0.5)], "axp-c3", "");
    p.value(tms, n + 1.1, "start", &format!(" time-mean {}", num(tms)));
    p.value(sms, n + 1.1, "end", &format!("space-mean {} ", num(sms)));
    let body = p.finish("speed (km/h)", "vehicle (spot speeds as given)",&format!("standard deviation {} km/h", num(sd)));
    Some(s.chart("distribution", "Time-mean and space-mean speed",
                 &format!("Spot speeds of {} vehicles (input). Marked: the time-mean speed {} km/h (arithmetic mean) and \
                           the space-mean speed {} km/h (harmonic mean) the model returned, standard deviation {} km/h.",
                          v.len(), num(tms), num(sms), num(sd)), &body))
}

fn reliability(s: &Solved) -> Option<Figure> {
    let t = s.vec("times")?;
    let (mean, t95) = (s.out("mean")?, s.out("t95")?);
    if t.is_empty() || t.len() > 40 {
        return None;
    }
    let pts: Vec<(f64, f64)> = t.iter().enumerate().map(|(i, x)| (i as f64 + 1.0, *x)).collect();
    let n = t.len() as f64;
    let mut p = Plot::new(&[0.5, n + 0.5], &t.iter().copied().chain([0.0, t95]).collect::<Vec<_>>(), false, false,
                          (false, true));
    for (x, y) in &pts {
        p.rect(x - 0.32, 0.0, x + 0.32, *y, "axp-b");
    }
    p.dashed(&[(0.5, t95), (n + 0.5, t95)], "");
    p.line(&[(0.5, mean), (n + 0.5, mean)], "axp-c2", "");
    p.value(n + 0.5, t95, "end", &format!("95th percentile {}", num(t95)));
    p.value(0.6, mean, "start", &format!("mean = {}", num(mean)));
    let body = p.finish("position in the list", "travel time (min)",
                        &format!("buffer index {}, planning time index {}", num(s.out("buffer_index")?),
                                 num(s.out("planning_time_index")?)));
    Some(s.chart("data", "Travel times and the reliability indices returned",
                 &format!("The observed travel times (input), one bar per run ({}), with the mean {} min and the 95th \
                           percentile {} min the model returned; also returned: buffer index {}, planning time index {}, \
                           travel time index {}.", t.len(), num(mean), num(t95), num(s.out("buffer_index")?),
                          num(s.out("planning_time_index")?), num(s.out("travel_time_index")?)), &body))
}

fn trip_fit(s: &Solved) -> Option<Figure> {
    let x = s.mat("X")?;
    let y = s.vec("y")?;
    let r2 = s.out("r2")?;
    if x.len() != y.len() || x.is_empty() || x.len() > 40 {
        return None;
    }
    let mut pts = vec![];
    for (row, yo) in x.iter().zip(&y) {
        let a = s.with_input(&[("x_new", Input::Json(serde_json::json!(row)))])?;
        pts.push((*yo, k::num_of(&a, "y_new")?));
    }
    let ynew = s.out("y_new");
    let all: Vec<f64> = pts.iter().flat_map(|p| [p.0, p.1]).chain(ynew).collect();
    let (lo, hi) = (all.iter().cloned().fold(f64::INFINITY, f64::min), all.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let mut p = Plot::new(&all, &all, false, false, (false, false));
    p.legend_left = true;
    p.dashed(&[(lo, lo), (hi, hi)], "observed = fitted");
    p.dots(&pts, "axp-d", &format!("zones (r² = {})", num(r2)));
    if let Some(yn) = ynew {
        p.mark(yn, yn, &format!("new zone: {}", num(yn)), true);
    }
    let body = p.finish("observed trips", "fitted trips (forecast)", "trip-generation regression");
    Some(s.chart("fit", "Trip-generation regression: observed against fitted",
                 &format!("Trip-generation regression on {} zones: each zone's observed trips against the trips the \
                           model forecasts for that zone's explanatory values ({} solves), with the 1:1 line; r² = {}.{}",
                          x.len(), x.len(), num(r2),
                          ynew.map(|v| format!(" Marked on the line: the forecast {} trips for the new zone.", num(v)))
                              .unwrap_or_default()), &body))
}

// ------------------------------------------------------------------------------------------------ curves and profiles
fn ssd_diagram(s: &Solved) -> Option<Figure> {
    let v = s.inp("v")?;
    let (ssd, dr0, db) = (s.out("SSD")?, s.out("d_r")?, s.out("d_b")?);
    let mut lo = vec![];
    let mut tot = vec![];
    for vv in k::span(v * 0.25, v * 1.3, 25, false) {
        if let Some(a) = s.with(&[("v", vv)]) {
            if let (Some(r), Some(t)) = (k::num_of(&a, "d_r"), k::num_of(&a, "SSD")) {
                lo.push((vv * 3.6, r));
                tot.push((vv * 3.6, t));
            }
        }
    }
    if lo.len() < 2 {
        return None;
    }
    let xs: Vec<f64> = tot.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = tot.iter().map(|p| p.1).chain([0.0]).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (false, true));
    p.legend_left = true;
    let mut band = lo.clone();
    band.extend(tot.iter().rev());
    p.area(&band, "axp-a");
    let mut base: Vec<(f64, f64)> = vec![(lo[0].0, 0.0)];
    base.extend(lo.iter());
    base.push((lo[lo.len() - 1].0, 0.0));
    p.area(&base, "axp-a2");
    p.line(&tot, "axp-c", "stopping sight distance (total)");
    p.line(&lo, "axp-c2", "reaction distance");
    p.mark(v * 3.6, ssd, &format!("{} m", num(ssd)), true);
    let body = p.finish("v, design speed (km/h)", "distance (m)",
                        &format!("reaction {} m + braking {} m", num(dr0), num(db)));
    Some(s.chart("diagram", "Stopping sight distance: reaction plus braking",
                 &format!("Stopping sight distance against design speed, 25 speeds computed by the model: the lower band \
                           is the distance travelled during the reaction time, the upper band the braking distance. \
                           Marked: v = {} km/h, SSD = {} m (reaction {} m + braking {} m).", num(v * 3.6), num(ssd),
                          num(dr0), num(db)), &body))
}

fn train_profile(s: &Solved) -> Option<Figure> {
    let (d, vmax, a, b) = (s.inp("D")?, s.inp("v_max")?, s.inp("a")?, s.inp("b")?);
    let (tt, vp) = (s.out("T")?, s.out("v_peak")?);
    let prof = s.out_text("profile").unwrap_or_default();
    if !(vp > 0.0 && d > 0.0) {
        return None;
    }
    let dist = |v: f64, acc: f64| -> Option<f64> {
        let r = s.solve_other("transport_acceleration_distance", &[("v1", f(0.0)), ("v2", f(v)), ("a", f(acc))])?;
        k::num_of(&r, "d")
    };
    let mut up = vec![(0.0, 0.0)];
    let mut down = vec![];
    for v in k::span(vp / 14.0, vp, 14, false) {
        up.push((dist(v, a)?, v * 3.6));
        down.push((d - dist(v, b)?, v * 3.6));
    }
    down.reverse();
    let mut pts = up.clone();
    pts.extend(down);
    pts.push((d, 0.0));
    let mut p = Plot::new(&[0.0, d], &[0.0, vmax * 3.6 * 1.12], false, false, (true, true));
    p.legend_bottom = true;
    p.legend_center = true;
    p.area(&pts, "axp-a");
    p.dashed(&[(0.0, vmax * 3.6), (d, vmax * 3.6)], &format!("line speed {} km/h", num(vmax * 3.6)));
    p.line(&pts, "axp-c", "speed (each point solved)");
    let top = up[up.len() - 1];
    p.mark(top.0, top.1, &format!("peak {} km/h", num(vp * 3.6)), false);
    let body = p.finish("distance (m)", "speed (km/h)", &format!("running time {} s, {prof}", num(tt)));
    Some(s.chart("profile", "Train run between stations: speed profile",
                 &format!("Speed-distance profile between stations {} m apart: the distance to reach each speed while \
                           accelerating at {} m/s² and while braking at {} m/s² is computed by the model's acceleration \
                           distance capability, up to the returned peak speed {} km/h. Running time returned: {} s \
                           ({prof}).", num(d), num(a), num(b), num(vp * 3.6), num(tt)), &body))
}

/// The road profile of a vertical curve of length l between grades g1 and g2 (PVI at station 0, elevation 0).
fn elev(x: f64, g1: f64, g2: f64, l: f64) -> f64 {
    if x <= -l / 2.0 {
        g1 * x
    } else if x >= l / 2.0 {
        g2 * x
    } else {
        let u = x + l / 2.0;
        g1 * (-l / 2.0) + g1 * u + (g2 - g1) * u * u / (2.0 * l)
    }
}

fn profile_plot(g1: f64, g2: f64, l: f64, span_s: f64) -> (Plot, Vec<(f64, f64)>, f64) {
    let half = (l / 2.0 + 0.45 * span_s.max(l)).max(l * 0.8);
    let xs = k::span(-half, half, 121, false);
    let pts: Vec<(f64, f64)> = xs.iter().map(|x| (*x, elev(*x, g1, g2, l))).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let (ylo, yhi) = (ys.iter().cloned().fold(f64::INFINITY, f64::min), ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let pad = (yhi - ylo).max(1.0) * 0.35;
    let p = Plot::new(&[-half, half], &[ylo - pad * 0.3, yhi + pad], false, false, (false, false));
    (p, pts, yhi + pad * 0.55)
}

fn crest_profile(s: &Solved) -> Option<Figure> {
    let (g1, g2) = (s.inp("g1")?, s.inp("g2")?);
    let (l, ss) = if s.key == "transport_crest_curve_length" { (s.out("L")?, s.inp("S")?) } else { (s.inp("L")?, s.out("S")?) };
    let case = s.out_text("case").unwrap_or_default();
    if !(l > 0.0 && ss > 0.0) {
        return None;
    }
    let (mut p, pts, ytop) = profile_plot(g1, g2, l, ss);
    let half = pts[pts.len() - 1].0;
    p.legend_bottom = true;
    p.legend_center = true;
    p.dashed(&[(-half, g1 * -half), (0.0, 0.0), (half, g2 * half)], "grades through the PVI");
    p.line(&pts, "axp-c", &format!("crest curve L = {} m", num(l)));
    p.dots(&[(-l / 2.0, elev(-l / 2.0, g1, g2, l)), (l / 2.0, elev(l / 2.0, g1, g2, l))], "axp-d", "");
    p.label(-l / 2.0, elev(-l / 2.0, g1, g2, l) - 0.0, "end", "BVC ");
    p.label(l / 2.0, elev(l / 2.0, g1, g2, l), "start", " EVC");
    // the crest of the drawn curve, and the sight distance as a dimension centred on it
    let xc = if (g2 - g1).abs() > 0.0 { (-l / 2.0 - g1 * l / (g2 - g1)).clamp(-l / 2.0, l / 2.0) } else { 0.0 };
    p.line(&[(xc - ss / 2.0, ytop), (xc + ss / 2.0, ytop)], "axp-c2", "");
    p.dots(&[(xc - ss / 2.0, ytop), (xc + ss / 2.0, ytop)], "axp-d2", "");
    p.mark(xc + ss / 2.0, ytop, &format!("S = {} m", num(ss)), false);
    let body = p.finish("station from the PVI (m)", "elevation (m)", &format!("grades {}, {}; {case}", num(g1), num(g2)));
    let what = if s.key == "transport_crest_curve_length" {
        format!("the shortest crest curve the model returned for a sight distance S = {} m, L = {} m", num(ss), num(l))
    } else {
        format!("a crest curve of length L = {} m and the sight distance the model returned, S = {} m", num(l), num(ss))
    };
    Some(s.chart("profile", if s.key == "transport_crest_curve_length" { "Crest curve length for a sight distance" } else {
        "Sight distance over a crest curve" },
                 &format!("Road profile between grades g1 = {} and g2 = {} with {what} (case {case}), drawn from the \
                           grades and that length (PVI at station 0; heights exaggerated by the axes); the sight distance \
                           is shown as a length centred on the crest.", num(g1), num(g2)), &body))
}

fn sag_profile(s: &Solved) -> Option<Figure> {
    let (g1, g2, ss) = (s.inp("g1")?, s.inp("g2")?, s.inp("S")?);
    let (l, aa, kk) = (s.out("L")?, s.out("A")?, s.out("K")?);
    if !(l > 0.0 && ss > 0.0) {
        return None;
    }
    let (mut p, pts, ytop) = profile_plot(g1, g2, l, ss);
    let half = pts[pts.len() - 1].0;
    p.legend_bottom = true;
    p.dashed(&[(-half, g1 * -half), (0.0, 0.0), (half, g2 * half)], "grades through the PVI");
    p.line(&pts, "axp-c", &format!("sag curve L = {} m", num(l)));
    let b = (-l / 2.0, elev(-l / 2.0, g1, g2, l));
    p.dots(&[b, (l / 2.0, elev(l / 2.0, g1, g2, l))], "axp-d", "");
    p.label(b.0, b.1, "end", "BVC ");
    p.label(l / 2.0, elev(l / 2.0, g1, g2, l), "start", " EVC");
    p.line(&[(b.0, ytop), (b.0 + ss, ytop)], "axp-c2", "");
    p.dots(&[(b.0, ytop), (b.0 + ss, ytop)], "axp-d2", "");
    p.mark(b.0 + ss, ytop, &format!("headlight S = {} m", num(ss)), false);
    let body = p.finish("station from the PVI (m)", "elevation (m)", &format!("A = {} %, K = {} m per %", num(aa), num(kk)));
    Some(s.chart("profile", "Sag curve sized for headlight sight distance",
                 &format!("A sag vertical curve between grades g1 = {} and g2 = {} sized for a headlight sight distance \
                           S = {} m: the model returned the length L = {} m (A = {} %, K = {} m per %). The profile is \
                           drawn from the grades and that length (heights exaggerated by the axes); the headlight sight \
                           distance is laid out from the BVC.", num(g1), num(g2), num(ss), num(l), num(aa), num(kk)), &body))
}

fn clothoid(s: &Solved) -> Option<Figure> {
    let (r, ls) = (s.inp("R")?, s.inp("L_s")?);
    let s_given = s.inp("s").unwrap_or(ls);
    let (x, y, th) = (s.out("x")?, s.out("y")?, s.out("theta")?);
    let mut pts = vec![(0.0, 0.0)];
    for sv in k::span(ls / 40.0, ls, 40, false) {
        if let Some(a) = s.with(&[("s", sv)]) {
            if let (Some(px), Some(py)) = (k::num_of(&a, "x"), k::num_of(&a, "y")) {
                pts.push((px, py));
            }
        }
    }
    if pts.len() < 4 {
        return None;
    }
    let xs: Vec<f64> = pts.iter().map(|p| p.0).chain([x]).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([y, 0.0]).collect();
    let mut p = Plot::new(&xs, &ys, false, false, (true, true));
    p.legend_bottom = true;
    p.dashed(&[(0.0, 0.0), (xs.iter().cloned().fold(0.0, f64::max), 0.0)], "initial tangent");
    p.line(&pts, "axp-c", &format!("clothoid, R = {} m, Ls = {} m", num(r), num(ls)));
    p.mark(x, y, &format!("({}, {}) m, θ = {}°", num(x), num(y), num(th)), true);
    let body = p.finish("x, along the initial tangent (m)", "y, offset (m)", &format!("at s = {} m", num(s_given)));
    Some(s.chart("geometry", "Clothoid transition curve",
                 &format!("A clothoid transition of length Ls = {} m into a circular curve of radius R = {} m: 41 points \
                           (x, y) along it, each computed by the model (the offset axis is stretched). Marked: the \
                           returned point x = {} m, y = {} m and tangent deflection θ = {}° at s = {} m.", num(ls), num(r),
                          num(x), num(y), num(th), num(s_given)), &body))
}

fn transmissibility(s: &Solved) -> Option<Figure> {
    let fr = s.inp("f")?;
    let (tb, tw) = (s.out("T_body")?, s.out("T_wheel")?);
    let set: Vec<(&str, Input)> = ["m_s", "m_u", "k_s", "k_t"].iter().filter_map(|n| Some((*n, f(s.inp(n)?)))).collect();
    if set.len() != 4 || !(fr > 0.0) {
        return None;
    }
    let nat = s.solve_other("transport_quarter_car_frequencies", &set)?;
    let (fb, fw) = (k::num_of(&nat, "f_body")?, k::num_of(&nat, "f_wheel")?);
    let lo = fr.min(fb) / 4.0;
    let hi = fr.max(fw) * 4.0;
    let mut body_c = vec![];
    let mut wheel_c = vec![];
    for x in k::span(lo, hi, 61, true) {
        if let Some(a) = s.with(&[("f", x)]) {
            if let (Some(b), Some(w)) = (k::num_of(&a, "T_body"), k::num_of(&a, "T_wheel")) {
                body_c.push((x, b));
                wheel_c.push((x, w));
            }
        }
    }
    if body_c.len() < 4 {
        return None;
    }
    let ys: Vec<f64> = body_c.iter().chain(wheel_c.iter()).map(|p| p.1).filter(|v| *v > 0.0).collect();
    let xs: Vec<f64> = body_c.iter().map(|p| p.0).collect();
    let mut p = Plot::new(&xs, &ys, true, true, (false, false));
    p.legend_bottom = true;
    p.legend_left = true;
    let (ya, yb) = (p.ya.lo, p.ya.hi);
    p.dashed(&[(fb, ya), (fb, yb)], &format!("body mode {} Hz", num(fb)));
    p.dashed(&[(fw, ya), (fw, yb)], "");
    p.line(&body_c, "axp-c", "body");
    p.line(&wheel_c, "axp-c2", "wheel");
    p.label(fw, yb, "start", &format!(" wheel hop {} Hz", num(fw)));
    p.mark(fr, tb, &format!("{} at {} Hz", num(tb), num(fr)), true);
    let body = p.finish("f, road input frequency (Hz)", "transmissibility", &format!("wheel {} at the input", num(tw)));
    Some(s.chart("response", "Quarter-car transmissibility: body and wheel",
                 &format!("Body and wheel transmissibility from road input to motion at 61 frequencies, each computed by \
                           the model (log-log); dashed: the natural frequencies from the model's quarter-car frequency \
                           capability, {} Hz and {} Hz. Marked: at f = {} Hz the returned body transmissibility {} (wheel \
                           {}).", num(fb), num(fw), num(fr), num(tb), num(tw)), &body))
}
