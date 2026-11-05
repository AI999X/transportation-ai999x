//! Transportation's own live figures, part two: vehicles, roads and diagrams drawn to scale from the inputs and labelled
//! with the values the model returned (axle loads, wheel loads, banked curves, a quarter car, sight lines on a curve,
//! a nested choice tree, distances laid end to end). Every number written is the model's.

use crate::figures::f;
use crate::figures_plot::put;
use ax_engine::figkit::{self as k, num, q, Canvas, Solved, P};
use ax_engine::figures::Figure;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "transport_braking_weight_transfer" => vec![side_view(s, false)],
        "transport_traction_limited_acceleration" => vec![side_view(s, true)],
        "transport_lateral_load_transfer" => vec![rear_view(s, false)],
        "transport_rollover_threshold" => vec![rear_view(s, true)],
        "transport_curve_safe_speed" => vec![banked(s, false)],
        "transport_superelevation_required" => vec![banked(s, true)],
        "transport_quarter_car_frequencies" => vec![quarter_car(s)],
        "transport_steady_state_cornering" => vec![cornering(s)],
        "transport_yellow_change_interval" => vec![yellow(s)],
        "transport_horizontal_sight_offset" | "transport_sight_distance_from_offset" => vec![sight_offset(s)],
        _ => vec![],
    }
}

fn kn(x: f64) -> String {
    q(x, "N")
}

fn txt(c: &mut Canvas, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
    put(c, x, y, anchor, class, t);
}

/// An arrow whose length stands for a force.
fn force(c: &mut Canvas, from: P, to: P) {
    if (from.0 - to.0).abs() + (from.1 - to.1).abs() > 8.0 {
        c.arrow(from, to);
    }
}

fn wheel(c: &mut Canvas, x: f64, ground: f64, r: f64) {
    c.raw(&format!("<circle class=\"axp-ka\" cx=\"{x:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>\
                    <circle class=\"axp-k\" cx=\"{x:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>\
                    <circle class=\"axp-kd\" cx=\"{x:.1}\" cy=\"{:.1}\" r=\"3\"/>", ground - r, ground - r, ground - r));
}

fn cg_mark(c: &mut Canvas, p: P) {
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\"/>\
                    <path class=\"axp-kd\" d=\"M{:.1},{:.1} L{:.1},{:.1} A8,8 0 0 1 {:.1},{:.1} Z M{:.1},{:.1} L{:.1},{:.1} A8,8 0 0 1 {:.1},{:.1} Z\"/>",
                   p.0, p.1, p.0, p.1, p.0, p.1 - 8.0, p.0 + 8.0, p.1, p.0, p.1, p.0, p.1 + 8.0, p.0 - 8.0, p.1));
}

/// Side view, heading left: braking (weight transfer) or accelerating at the traction limit.
fn side_view(s: &Solved, traction: bool) -> Option<Figure> {
    let (l, lf, h) = (s.inp("L")?, s.inp("l_f")?, s.inp("h")?);
    let (wf, wr) = (s.out("W_f")?, s.out("W_r")?);
    if !(l > 0.0 && lf >= 0.0 && lf <= l && h > 0.0 && wf >= 0.0 && wr >= 0.0) {
        return None;
    }
    let mut c = Canvas::new(330.0);
    let sc = (280.0 / l).min(120.0 / h);
    let ground = 200.0;
    let xf = 150.0;
    let xr = xf + l * sc;
    let cg = (xf + lf * sc, ground - h * sc);
    let r = 24.0;
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"14\"/>\
                    <rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"14\"/>",
                   xf - 50.0, (cg.1 - 30.0).min(ground - 70.0), xr - xf + 95.0, (ground - r - (cg.1 - 30.0).min(ground - 70.0)).max(20.0),
                   xf - 50.0, (cg.1 - 30.0).min(ground - 70.0), xr - xf + 95.0, (ground - r - (cg.1 - 30.0).min(ground - 70.0)).max(20.0)));
    c.line((20.0, ground), (500.0, ground));
    wheel(&mut c, xf, ground, r);
    wheel(&mut c, xr, ground, r);
    cg_mark(&mut c, cg);
    txt(&mut c, cg.0 + 12.0, cg.1 - 10.0, "start", "axp-lt", "CG");
    // axle loads, one scale: the larger is 80 px
    let big = wf.max(wr).max(1e-12);
    force(&mut c, (xf, ground + 80.0 * wf / big + 6.0), (xf, ground + 2.0));
    force(&mut c, (xr, ground + 80.0 * wr / big + 6.0), (xr, ground + 2.0));
    txt(&mut c, xf - 8.0, ground + 50.0, "end", "axp-vt", &format!("front {}", kn(wf)));
    txt(&mut c, xr + 8.0, ground + 50.0, "start", "axp-vt", &format!("rear {}", kn(wr)));
    // weight at the CG (no value: the drawing states the axle loads)
    force(&mut c, (cg.0, cg.1 + 10.0), (cg.0, cg.1 + 46.0));
    txt(&mut c, cg.0 + 6.0, cg.1 + 44.0, "start", "axp-lt", "W");
    let (title, caption, fig);
    if traction {
        let (fm, am) = (s.out("F_max")?, s.out("a_max")?);
        let drive = s.txt("drive").unwrap_or_else(|| "rear".into());
        let at: Vec<f64> = match drive.as_str() {
            "front" => vec![xf],
            "all" => vec![xf, xr],
            _ => vec![xr],
        };
        for x in &at {
            force(&mut c, (*x, ground - 4.0), (*x - 60.0, ground - 4.0));
        }
        txt(&mut c, at[at.len() - 1] - 30.0, ground + 18.0, "end", "axp-vt", &format!("F = {}", kn(fm)));
        force(&mut c, (cg.0, cg.1), (cg.0 - 70.0, cg.1));
        txt(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("{drive} drive: a_max = {} m/s²", num(am)));
        txt(&mut c, 16.0, 318.0, "start", "axp-lt", "heading left; arrows to one scale for the axle loads");
        title = "Traction-limited acceleration";
        caption = format!("Side view of a {drive}-drive vehicle (wheelbase {} m, CG {} m behind the front axle and {} m \
                           high, drawn to scale) accelerating to the left at its traction limit. Marked: the largest \
                           tractive force the model returned, {}, acceleration {} m/s² and axle loads front {}, rear {}.",
                          num(l), num(lf), num(h), kn(fm), num(am), kn(wf), kn(wr));
        fig = "fbd";
    } else {
        let (dw, share) = (s.out("dW")?, s.out("front_share")?);
        let a = s.inp("a")?;
        force(&mut c, (cg.0, cg.1), (cg.0 - 70.0, cg.1));
        txt(&mut c, cg.0 - 74.0, cg.1 - 8.0, "end", "axp-lt", "inertia");
        txt(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("braking at {} m/s²", num(a)));
        txt(&mut c, 16.0, 300.0, "start", "axp-vt", &format!("load transferred to the front {}", kn(dw)));
        txt(&mut c, 16.0, 320.0, "start", "axp-vt", &format!("ideal front brake share {} %", num(share * 100.0)));
        title = "Braking: weight transfer to the front axle";
        caption = format!("Side view of a vehicle (wheelbase {} m, CG {} m behind the front axle and {} m high, drawn to \
                           scale) braking at {} m/s² towards the left. Marked: the dynamic axle loads the model returned, \
                           front {} and rear {} (arrows to one scale), the load transferred {} and the ideal front brake \
                           share {}.", num(l), num(lf), num(h), num(a), kn(wf), kn(wr), kn(dw), num(share));
        fig = "fbd";
    }
    Some(s.figure(fig, title, &caption, c.body(), c.h))
}

/// Rear view: lateral load transfer in a left turn, or the rollover geometry.
fn rear_view(s: &Solved, rollover: bool) -> Option<Figure> {
    let (tw, h) = (s.inp("t_w")?, s.inp("h")?);
    if !(tw > 0.0 && h > 0.0) {
        return None;
    }
    let mut c = Canvas::new(320.0);
    let sc = (220.0 / tw).min(150.0 / h);
    let ground = 220.0;
    let cx = 260.0;
    let (xl, xr) = (cx - tw * sc / 2.0, cx + tw * sc / 2.0);
    let cg = (cx, ground - h * sc);
    let top = (cg.1 - 40.0).min(ground - 80.0);
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"12\"/>\
                    <rect class=\"axp-k\" x=\"{:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"12\"/>",
                   xl - 20.0, xr - xl + 40.0, ground - 30.0 - top, xl - 20.0, xr - xl + 40.0, ground - 30.0 - top));
    for x in [xl, xr] {
        c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"22\" height=\"50\" rx=\"5\"/>\
                        <rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"22\" height=\"50\" rx=\"5\"/>",
                       x - 11.0, ground - 50.0, x - 11.0, ground - 50.0));
    }
    c.line((30.0, ground), (490.0, ground));
    cg_mark(&mut c, cg);
    txt(&mut c, cg.0 + 12.0, cg.1 - 10.0, "start", "axp-lt", "CG");
    // track dimension
    c.dashed((xl, ground + 14.0), (xr, ground + 14.0));
    txt(&mut c, cx, ground + 30.0, "middle", "axp-lt", &format!("track {} m", num(tw)));
    let (title, caption);
    if rollover {
        let (ssf, vr) = (s.out("SSF")?, s.out("v_roll_kmh")?);
        let r = s.inp("R");
        c.raw(&format!("<path class=\"axp-c2\" d=\"M{:.1},{:.1} L{xr:.1},{ground:.1}\"/>", cg.0, cg.1));
        c.dashed((cg.0, cg.1), (cg.0, ground));
        txt(&mut c, cg.0 - 8.0, (cg.1 + ground) / 2.0, "end", "axp-lt", &format!("h = {} m", num(h)));
        txt(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("static stability factor t/2h = {}", num(ssf)));
        txt(&mut c, 16.0, 300.0, "start", "axp-vt", &format!("rollover speed{}: {} km/h",
                                                             r.map(|x| format!(" on R = {} m", num(x))).unwrap_or_default(), num(vr)));
        title = "Rollover threshold: static stability factor";
        caption = format!("Rear view of a vehicle with track {} m and CG height {} m (to scale); the green line from the \
                           CG to the outer wheel's contact shows the tipping geometry. Marked: the static stability factor \
                           the model returned, {}, and the rollover speed {} km/h{}.", num(tw), num(h), num(ssf), num(vr),
                          r.map(|x| format!(" on a curve of radius {} m", num(x))).unwrap_or_default());
    } else {
        let (wo, wi, dw) = (s.out("W_out")?, s.out("W_in")?, s.out("dW")?);
        let ay = s.inp("a_y")?;
        if !(wo >= 0.0 && wi >= 0.0) {
            return None;
        }
        let big = wo.max(wi).max(1e-12);
        // turning left: the outer wheels on the right, the inertia force outward (right)
        force(&mut c, (xr, ground + 6.0 + 70.0 * wo / big), (xr, ground + 2.0));
        force(&mut c, (xl, ground + 6.0 + 70.0 * wi / big), (xl, ground + 2.0));
        txt(&mut c, xr + 16.0, ground + 56.0, "start", "axp-vt", &format!("outer {}", kn(wo)));
        txt(&mut c, xl - 16.0, ground + 56.0, "end", "axp-vt", &format!("inner {}", kn(wi)));
        force(&mut c, cg, (cg.0 + 80.0, cg.1));
        txt(&mut c, cg.0 + 84.0, cg.1 + 5.0, "start", "axp-lt", "inertia");
        txt(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("turning left at a_y = {} m/s²", num(ay)));
        txt(&mut c, 16.0, 312.0, "start", "axp-vt", &format!("transferred to the outer wheels: {}", kn(dw)));
        title = "Cornering: lateral load transfer";
        caption = format!("Rear view of a vehicle (track {} m, CG {} m high, drawn to scale) turning left at a lateral \
                           acceleration of {} m/s²: the outward inertia force at the CG and the wheel loads the model \
                           returned, outer {} and inner {} (arrows to one scale); load transferred {}.", num(tw), num(h),
                          num(ay), kn(wo), kn(wi), kn(dw));
    }
    Some(s.figure("fbd", title, &caption, c.body(), c.h))
}

/// A banked carriageway: the cross-slope to scale, the vehicle, weight, normal reaction and the centripetal direction.
fn banked(s: &Solved, required: bool) -> Option<Figure> {
    let r = s.inp("R")?;
    let fr = s.inp("f")?;
    let (e, vkmh) = if required { (s.out("e")?, s.inp("v")? * 3.6) } else { (s.inp("e")?, s.out("v_kmh")?) };
    if !(e.abs() < 1.0) {
        return None;
    }
    let mut c = Canvas::new(300.0);
    let th = e.atan();
    let (cx, cy) = (260.0, 190.0);
    let (ux, uy) = (th.cos(), -th.sin());   // along the road, rising to the right (the outside)
    let a = (cx - 200.0 * ux, cy - 200.0 * uy);
    let b = (cx + 200.0 * ux, cy + 200.0 * uy);
    c.raw(&format!("<path class=\"axp-ka\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},250 L{:.1},250 Z\"/>", a.0, a.1, b.0, b.1,
                   b.0, a.0));
    c.line(a, b);
    c.dashed(a, (b.0, a.1));
    // the vehicle: a box standing square on the road; n points up from the road surface
    let n = (uy, -ux);
    let corner = |along: f64, up: f64| (cx + along * ux + up * n.0, cy + along * uy + up * n.1);
    let pts = [corner(-60.0, 0.0), corner(60.0, 0.0), corner(60.0, 60.0), corner(-60.0, 60.0)];
    c.raw(&format!("<path class=\"axp-ka\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>\
                    <path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>",
                   pts[0].0, pts[0].1, pts[1].0, pts[1].1, pts[2].0, pts[2].1, pts[3].0, pts[3].1,
                   pts[0].0, pts[0].1, pts[1].0, pts[1].1, pts[2].0, pts[2].1, pts[3].0, pts[3].1));
    let g = corner(0.0, 30.0);
    cg_mark(&mut c, g);
    force(&mut c, g, (g.0, g.1 + 90.0));
    txt(&mut c, g.0 + 8.0, g.1 + 86.0, "start", "axp-lt", "W");
    let base = corner(0.0, 0.0);
    force(&mut c, (base.0 - 30.0 * ux, base.1 - 30.0 * uy), (base.0 - 30.0 * ux + 100.0 * n.0, base.1 - 30.0 * uy + 100.0 * n.1));
    let ntip = (base.0 - 30.0 * ux + 100.0 * n.0, base.1 - 30.0 * uy + 100.0 * n.1);
    txt(&mut c, ntip.0 - 8.0, ntip.1, "end", "axp-lt", "N");
    force(&mut c, (g.0 - 40.0, 60.0), (g.0 - 190.0, 60.0));
    txt(&mut c, g.0 - 115.0, 52.0, "middle", "axp-lt", "towards the centre");
    let head = if required { format!("e = {} %", num(e * 100.0)) } else { format!("v = {} km/h", num(vkmh)) };
    txt(&mut c, 16.0, 22.0, "start", "axp-mt", &head);
    txt(&mut c, 16.0, 278.0, "start", "axp-vt", &if required { format!("v = {} km/h", num(vkmh)) } else {
        format!("e = {} %", num(e * 100.0)) });
    txt(&mut c, 200.0, 278.0, "start", "axp-vt", &format!("R = {}", q(r, "m")));
    txt(&mut c, 360.0, 278.0, "start", "axp-vt", &format!("f = {}", num(fr)));
    let (title, cap) = if required {
        ("Superelevation needed on a curve",
         format!("A banked carriageway (cross-slope to scale) for a speed of {} km/h on a radius R = {} m with side friction \
                  f = {}: weight, normal reaction and the centripetal direction. Marked: the superelevation the model \
                  returned, e = {} %.", num(vkmh), num(r), num(fr), num(e * 100.0)))
    } else {
        ("Safe speed on a banked curve",
         format!("A banked carriageway with superelevation e = {} (cross-slope to scale) on a radius R = {} m with side \
                  friction f = {}: weight, normal reaction and the centripetal direction. Marked: the safe speed the model \
                  returned, {} km/h.", num(e), num(r), num(fr), num(vkmh)))
    };
    Some(s.figure("fbd", title, &cap, c.body(), c.h))
}

/// A spring drawn as a zig-zag between a and b.
fn spring(c: &mut Canvas, a: P, b: P) {
    c.resistor(a, b);
}

fn quarter_car(s: &Solved) -> Option<Figure> {
    let (ms, mu, ks, kt) = (s.inp("m_s")?, s.inp("m_u")?, s.inp("k_s")?, s.inp("k_t")?);
    let (fb, fw) = (s.out("f_body")?, s.out("f_wheel")?);
    let mut c = Canvas::new(330.0);
    let x = 200.0;
    // road, tyre spring, wheel mass, suspension spring, body mass
    c.raw("<path class=\"axp-k\" d=\"M110,300 L290,300\"/>");
    for i in 0..9 {
        let xx = 115.0 + 20.0 * i as f64;
        c.line((xx, 300.0), (xx - 10.0, 312.0));
    }
    spring(&mut c, (x, 300.0), (x, 235.0));
    c.block(x - 60.0, 195.0, 120.0, 40.0, &format!("{} kg", num(mu)));
    spring(&mut c, (x, 195.0), (x, 115.0));
    c.block(x - 80.0, 55.0, 160.0, 60.0, &format!("{} kg", num(ms)));
    txt(&mut c, x + 24.0, 270.0, "start", "axp-lt", &format!("tyre kt = {}", q(kt, "N/m")));
    txt(&mut c, x + 24.0, 160.0, "start", "axp-lt", &format!("suspension ks = {}", q(ks, "N/m")));
    txt(&mut c, x - 66.0, 220.0, "end", "axp-lt", "wheel");
    txt(&mut c, x - 86.0, 90.0, "end", "axp-lt", "body");
    txt(&mut c, 330.0, 82.0, "start", "axp-vt", &format!("body mode {}", q(fb, "Hz")));
    txt(&mut c, 330.0, 218.0, "start", "axp-vt", &format!("wheel hop {}", q(fw, "Hz")));
    txt(&mut c, 16.0, 22.0, "start", "axp-mt", "quarter-car model");
    Some(s.figure("schematic", "Quarter-car model: body and wheel-hop frequencies",
                  &format!("A quarter-car model: sprung mass {} kg on a suspension spring ks = {} N/m, unsprung mass {} kg \
                            on a tyre spring kt = {} N/m. Marked: the natural frequencies the model returned, body mode {} \
                            Hz and wheel hop {} Hz.", num(ms), num(ks), num(mu), num(kt), num(fb), num(fw)), c.body(), c.h))
}

fn cornering(s: &Solved) -> Option<Figure> {
    let (l, lf, u, delta) = (s.inp("L")?, s.inp("l_f")?, s.inp("u")?, s.inp("delta")?);
    let (r, ay, rr, beta) = (s.out("r")?, s.out("a_y")?, s.out("R")?, s.out("beta")?);
    if !(l > 0.0 && lf >= 0.0 && lf <= l) {
        return None;
    }
    let mut c = Canvas::new(320.0);
    let sc = 300.0 / l;
    let y = 200.0;
    let xr = 110.0;
    let xf = xr + l * sc;
    let xcg = xf - lf * sc;
    // body outline, axle line, wheels (the front one steered, its angle exaggerated six times)
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"70\" rx=\"16\"/>\
                    <rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"70\" rx=\"16\"/>",
                   xr - 40.0, y - 35.0, xf - xr + 80.0, xr - 40.0, y - 35.0, xf - xr + 80.0));
    c.line((xr, y), (xf, y));
    let wh = |c: &mut Canvas, cx: f64, ang: f64| {
        let (dx, dy) = (22.0 * ang.cos(), -22.0 * ang.sin());
        c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:7\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", cx - dx, y - dy,
                       cx + dx, y + dy));
    };
    wh(&mut c, xr, 0.0);
    let shown = (delta * 6.0).clamp(-1.2, 1.2);
    wh(&mut c, xf, shown);
    cg_mark(&mut c, (xcg, y));
    txt(&mut c, xcg, y + 26.0, "middle", "axp-lt", "CG");
    txt(&mut c, xf + 10.0, y - 40.0, "start", "axp-lt", &format!("δ = {}°", num(delta.to_degrees())));
    force(&mut c, (xcg, y - 40.0), (xcg, y - 120.0));
    txt(&mut c, xcg + 8.0, y - 100.0, "start", "axp-vt", &format!("towards the centre, R = {}", q(rr, "m")));
    force(&mut c, (xf + 50.0, y), (xf + 100.0, y));
    txt(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("yaw rate {} rad/s, a_y = {} m/s²", num(r), num(ay)));
    txt(&mut c, 16.0, 300.0, "start", "axp-vt", &format!("sideslip at the CG {}°, speed {} km/h", num(beta), num(u * 3.6)));
    Some(s.figure("schematic", "Steady-state cornering of the bicycle model",
                  &format!("Plan view of the bicycle model of a vehicle (wheelbase {} m, CG {} m behind the front axle) at \
                            {} km/h with a front steer angle of {}° (the wheel angle exaggerated six times). Marked: the \
                            values the model returned, turn radius {} m, yaw rate {} rad/s, lateral acceleration {} m/s² and \
                            sideslip {}°.", num(l), num(lf), num(u * 3.6), num(delta.to_degrees()), num(rr), num(r), num(ay),
                           num(beta)), c.body(), c.h))
}

fn yellow(s: &Solved) -> Option<Figure> {
    let v = s.inp("v")?;
    let (y, xc) = (s.out("Y")?, s.out("x_c")?);
    if !(xc > 0.0) {
        return None;
    }
    let mut c = Canvas::new(240.0);
    let (x1, x0) = (440.0, 90.0);
    // the road and the stop line
    c.raw("<rect class=\"axp-ka\" x=\"20\" y=\"110\" width=\"480\" height=\"60\"/>");
    c.line((20.0, 110.0), (500.0, 110.0));
    c.line((20.0, 170.0), (500.0, 170.0));
    c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:5\" d=\"M{x1},110 L{x1},170\"/>"));
    txt(&mut c, x1, 100.0, "middle", "axp-lt", "stop line");
    // the signal head
    c.raw(&format!("<rect class=\"axp-k\" x=\"{}\" y=\"30\" width=\"18\" height=\"48\" rx=\"4\"/>\
                    <circle class=\"axp-kd\" cx=\"{}\" cy=\"54\" r=\"6\"/>", x1 + 22.0, x1 + 31.0));
    // the vehicle at the critical distance
    c.raw(&format!("<rect class=\"axp-k\" x=\"{}\" y=\"124\" width=\"56\" height=\"32\" rx=\"6\"/>", x0 - 56.0));
    force(&mut c, (x0 + 4.0, 140.0), (x0 + 50.0, 140.0));
    c.dashed((x0, 190.0), (x1, 190.0));
    c.line((x0, 182.0), (x0, 198.0));
    c.line((x1, 182.0), (x1, 198.0));
    txt(&mut c, (x0 + x1) / 2.0, 212.0, "middle", "axp-vt", &format!("x_c = {} m", num(xc)));
    txt(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("yellow interval Y = {} s", num(y)));
    txt(&mut c, 16.0, 232.0, "start", "axp-lt", &format!("approach speed {} km/h", num(v * 3.6)));
    Some(s.figure("schematic", "Yellow interval and the critical stopping distance",
                  &format!("A vehicle approaching a signal at {} km/h: from the critical distance the model returned, x_c = \
                            {} m, it can just stop at the line; the yellow change interval returned is Y = {} s.",
                           num(v * 3.6), num(xc), num(y)), c.body(), c.h))
}

/// Plan of the inside lane of a curve: the sight line as the chord over the arc S, the clearance M to the obstruction.
fn sight_offset(s: &Solved) -> Option<Figure> {
    let r = s.inp("R")?;
    let (ss, m) = if s.key == "transport_horizontal_sight_offset" { (s.inp("S")?, s.out("M")?) } else { (s.out("S")?, s.inp("M")?) };
    if !(r > 0.0 && ss > 0.0 && m > 0.0 && m < r && ss / r < std::f64::consts::PI) {
        return None;
    }
    let th = ss / r;                                   // the angle the sight distance subtends
    let chord = 2.0 * r * (th / 2.0).sin();
    let sc = 290.0 / chord.max(1e-9);
    let sc = sc.min(90.0 / (r * (1.0 - (th / 2.0).cos())).max(m).max(1e-9));
    // centre of the curve below the drawing; the arc's middle at (260, top)
    let top = 70.0;
    let cen = (260.0, top + r * sc);
    let at = |ang: f64, rad: f64| (cen.0 + rad * sc * ang.sin(), cen.1 - rad * sc * ang.cos());
    let mut d = String::new();
    let ext = th * 0.75;
    for i in 0..=60 {
        let a = -ext + 2.0 * ext * i as f64 / 60.0;
        let p = at(a, r);
        d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
    }
    c_path(&d)?;
    let mut c = Canvas::new(220.0);
    c.raw(&format!("<path class=\"axp-k\" d=\"{}\"/>", d.trim_end()));
    let mut d2 = String::new();
    for i in 0..=60 {
        let a = -ext + 2.0 * ext * i as f64 / 60.0;
        let p = at(a, r - m);
        d2.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
    }
    c.raw(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"{}\"/>", d2.trim_end()));
    let (p1, p2) = (at(-th / 2.0, r), at(th / 2.0, r));
    c.raw(&format!("<path class=\"axp-c2\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p1.0, p1.1, p2.0, p2.1));
    c.dot(p1);
    c.dot(p2);
    txt(&mut c, p1.0, p1.1 + 20.0, "middle", "axp-lt", "driver");
    txt(&mut c, p2.0, p2.1 + 20.0, "middle", "axp-lt", "object");
    let mid = at(0.0, r);
    let mid2 = at(0.0, r - m);
    c.arrow(mid, mid2);
    txt(&mut c, mid.0 + 8.0, (mid.1 + mid2.1) / 2.0 + 4.0, "start", "axp-vt", &format!("M = {} m", num(m)));
    txt(&mut c, mid.0, mid.1 - 10.0, "middle", "axp-lt", "centre of the inside lane");
    txt(&mut c, mid2.0, mid2.1 + 22.0, "middle", "axp-lt", "obstruction line (dashed)");
    txt(&mut c, 16.0, 22.0, "start", "axp-mt", &format!("S = {} m along the lane, R = {} m", num(ss), num(r)));
    let (title, cap) = if s.key == "transport_horizontal_sight_offset" {
        ("Sight offset on the inside of a curve",
         format!("Plan of the inside lane of a curve of radius R = {} m with a sight distance S = {} m along the lane: the \
                  sight line is the chord between driver and object, and the clearance it needs to the obstruction is the \
                  middle ordinate the model returned, M = {} m (geometry to scale).", num(r), num(ss), num(m)))
    } else {
        ("Sight distance allowed by a lateral clearance",
         format!("Plan of the inside lane of a curve of radius R = {} m with an obstruction M = {} m from the lane centre: \
                  the sight distance the model returned, S = {} m, is the length along the lane whose chord just clears \
                  the obstruction (geometry to scale).", num(r), num(m), num(ss)))
    };
    Some(s.figure("geometry", title, &cap, c.body(), c.h))
}

fn c_path(d: &str) -> Option<()> {
    (!d.contains("NaN") && !d.contains("inf")).then_some(())
}

// ------------------------------------------------------------------------------------------------ trees and parts
pub fn nested_tree(s: &Solved) -> Option<Figure> {
    let v = s.vec("V")?;
    let nest = s.vec("nest")?;
    let lam = s.vec("lam")?;
    let pr = s.out_list("P")?;
    let pn = s.out_list("P_nest")?;
    let n = v.len();
    if n == 0 || n > 8 || nest.len() != n || pr.len() != n || pn.is_empty() || pn.len() > 4 {
        return None;
    }
    let mut c = Canvas::new(330.0);
    let root = (260.0, 50.0);
    c.raw(&format!("<circle class=\"axp-kd\" cx=\"{}\" cy=\"{}\" r=\"6\"/>", root.0, root.1));
    txt(&mut c, root.0 + 12.0, root.1 + 4.0, "start", "axp-lt", "choice");
    let leaf_w = 480.0 / n as f64;
    let leaf_x = |i: usize| 20.0 + leaf_w * (i as f64 + 0.5);
    // alternatives sorted by nest so each nest's leaves sit together
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| nest[*a].partial_cmp(&nest[*b]).unwrap_or(std::cmp::Ordering::Equal).then(a.cmp(b)));
    for (j, pnj) in pn.iter().enumerate() {
        let members: Vec<usize> = order.iter().enumerate().filter(|(_, i)| nest[**i] as usize == j + 1).map(|(pos, _)| pos)
            .collect();
        if members.is_empty() {
            continue;
        }
        let nx = members.iter().map(|p| leaf_x(*p)).sum::<f64>() / members.len() as f64;
        let ny = 150.0;
        c.line(root, (nx, ny - 8.0));
        c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"16\"/>", nx - 8.0, ny - 8.0));
        txt(&mut c, nx, ny - 16.0, "middle", "axp-vt", &format!("nest {}: {} %", j + 1, num(pnj * 100.0)));
        if let Some(l) = lam.get(j) {
            txt(&mut c, nx, ny + 24.0, "middle", "axp-lt", &format!("λ = {}", num(*l)));
        }
        for pos in members {
            let i = order[pos];
            let lx = leaf_x(pos);
            c.line((nx, ny + 30.0), (lx, 240.0));
            c.dot((lx, 244.0));
            txt(&mut c, lx, 266.0, "middle", "axp-lt", &format!("alt {}", i + 1));
            txt(&mut c, lx, 284.0, "middle", "axp-lt", &format!("V = {}", num(v[i])));
            txt(&mut c, lx, 304.0, "middle", "axp-vt", &format!("{} %", num(pr[i] * 100.0)));
        }
    }
    txt(&mut c, 16.0, 22.0, "start", "axp-mt", "nested logit");
    Some(s.figure("tree", "Nested logit: nest and alternative probabilities",
                  &format!("A nested logit tree: {n} alternatives with utilities [{}] grouped into nests with dissimilarity \
                            parameters [{}]. Marked: the nest probabilities the model returned ({}) and the alternative \
                            probabilities ({}).", v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                           lam.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                           pn.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                           pr.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")), c.body(), c.h))
}

/// Parts laid end to end to scale on one bar: (name, value) and the unit.
fn parts_bar(c: &mut Canvas, parts: &[(String, f64)], unit: &str, total_label: &str) -> Option<()> {
    let total: f64 = parts.iter().map(|p| p.1).sum();
    if !(total > 0.0) || parts.iter().any(|p| p.1 < 0.0) {
        return None;
    }
    let (x0, x1, y) = (40.0, 480.0, 90.0);
    let sx = |t: f64| x0 + (x1 - x0) * t / total;
    let classes = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];
    let mut t = 0.0;
    for (i, (nm, d)) in parts.iter().enumerate() {
        let (a, b) = (sx(t), sx(t + d));
        c.raw(&format!("<rect class=\"{}\" x=\"{a:.1}\" y=\"{y}\" width=\"{:.1}\" height=\"44\"/>", classes[i % 4],
                       (b - a).max(0.0)));
        let up = i % 2 == 0;
        put(c, (a + b) / 2.0, if up { y - 26.0 } else { y + 64.0 }, "middle", "axp-lt", nm);
        put(c, (a + b) / 2.0, if up { y - 9.0 } else { y + 81.0 }, "middle", "axp-vt", &q(*d, unit));
        t += d;
    }
    let step = {
        let raw = total / 5.0;
        let mag = 10f64.powf(raw.log10().floor());
        [1.0, 2.0, 2.5, 5.0, 10.0].iter().map(|m| m * mag).find(|s| total / s <= 6.5).unwrap_or(10.0 * mag)
    };
    let mut v = 0.0;
    while v <= total * (1.0 + 1e-9) {
        let x = sx(v);
        c.raw(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{}\" x2=\"{x:.1}\" y2=\"{}\"/>", y + 92.0, y + 98.0));
        put(c, x, y + 114.0, "middle", "axp-t", &num(v));
        v += step;
    }
    c.raw(&format!("<line class=\"axp-g\" x1=\"{x0}\" y1=\"{}\" x2=\"{x1}\" y2=\"{}\"/>", y + 92.0, y + 92.0));
    put(c, 16.0, 22.0, "start", "axp-mt", &format!("{total_label} {}", q(total, unit)));
    Some(())
}

pub fn breakdown_ssd(s: &Solved) -> Option<Figure> {
    let (ssd, dr0, db) = (s.out("SSD")?, s.out("d_r")?, s.out("d_b")?);
    let mut c = Canvas::new(240.0);
    parts_bar(&mut c, &[("reaction".into(), dr0), ("braking".into(), db)], "m", "stopping sight distance")?;
    put(&mut c, 260.0, 232.0, "middle", "axp-lt", "distance (m)");
    Some(s.figure("breakdown", "Stopping sight distance: reaction and braking",
                  &format!("Stopping sight distance at {} km/h: the reaction and braking distances the model returned, to \
                            scale. Marked: {} m + {} m = SSD {} m.", num(s.inp("v")? * 3.6), num(dr0), num(db), num(ssd)),
                  c.body(), c.h))
}

pub fn breakdown_psd(s: &Solved) -> Option<Figure> {
    let (psd, d1, d2, d3, d4) = (s.out("PSD")?, s.out("d1")?, s.out("d2")?, s.out("d3")?, s.out("d4")?);
    let mut c = Canvas::new(240.0);
    parts_bar(&mut c, &[("d1 initial".into(), d1), ("d2 occupying".into(), d2), ("d3 clearance".into(), d3),
                        ("d4 opposing".into(), d4)], "m", "passing sight distance")?;
    put(&mut c, 260.0, 232.0, "middle", "axp-lt", "distance (m)");
    Some(s.figure("breakdown", "Passing sight distance and its four parts",
                  &format!("Passing sight distance at {} km/h: its four parts as the model returned them, laid end to end \
                            to scale. Marked: d1 = {} m, d2 = {} m, d3 = {} m, d4 = {} m, PSD = {} m.",
                           num(s.inp("v")? * 3.6), num(d1), num(d2), num(d3), num(d4), num(psd)), c.body(), c.h))
}

pub fn breakdown_resistance(s: &Solved) -> Option<Figure> {
    let (fr, fa, ft, pkw) = (s.out("F_roll")?, s.out("F_aero")?, s.out("F_total")?, s.out("P_kW")?);
    let fg = s.out("F_grade").unwrap_or(0.0);
    let mut parts = vec![("rolling".to_string(), fr), ("aerodynamic".to_string(), fa)];
    if fg > 0.0 {
        parts.push(("grade".into(), fg));
    } else if fg < 0.0 {
        return None;                                   // a downhill grade does not add end to end
    }
    let mut c = Canvas::new(240.0);
    parts_bar(&mut c, &parts, "N", "total resistance")?;
    put(&mut c, 260.0, 232.0, "middle", "axp-lt", &format!("force (N); power needed {} kW", num(pkw)));
    let _ = k::num;
    let _ = f;
    Some(s.figure("breakdown", "Road load: rolling, aerodynamic and grade resistance",
                  &format!("Road-load forces at {} km/h as the model returned them, laid end to end: rolling {} N, \
                            aerodynamic {} N{}; total {} N, power needed {} kW.", num(s.inp("v")? * 3.6), num(fr), num(fa),
                           if fg > 0.0 { format!(", grade {} N", num(fg)) } else { String::new() }, num(ft), num(pkw)),
                  c.body(), c.h))
}
