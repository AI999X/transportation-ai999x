//! A small chart frame for Transportation's own figures: points, lines, bands, dashed guides and labels placed in
//! data coordinates on the site's figure classes (axp-*), in the generic 520 x 340 frame. Only drawing lives here;
//! every value drawn comes from the model (figures.rs).

use ax_engine::figkit::{num, Canvas};

pub const X0: f64 = 96.0;
pub const X1: f64 = 504.0;
pub const Y0: f64 = 44.0;
pub const Y1: f64 = 278.0;

#[derive(Clone, Copy)]
pub struct Axis {
    pub lo: f64,
    pub hi: f64,
    pub log: bool,
    p0: f64,
    p1: f64,
}

impl Axis {
    /// An axis over the values given (padded a little; logarithmic when asked and all positive).
    pub fn over(vals: &[f64], log: bool, p0: f64, p1: f64, from_zero: bool) -> Axis {
        let v: Vec<f64> = vals.iter().copied().filter(|x| x.is_finite() && (!log || *x > 0.0)).collect();
        let (mut lo, mut hi) = if v.is_empty() { (0.0, 1.0) } else {
            (v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
        };
        let log = log && lo > 0.0;
        if log {
            lo = 10f64.powf(lo.log10().floor());
            hi = 10f64.powf(hi.log10().ceil());
            if hi <= lo {
                hi = lo * 10.0;
            }
        } else {
            if from_zero {
                lo = lo.min(0.0);
                hi = hi.max(0.0);
            }
            if hi - lo <= 1e-300 * hi.abs().max(1.0) || hi == lo {
                let d = if hi == 0.0 { 1.0 } else { hi.abs() * 0.1 };
                lo -= d;
                hi += d;
            }
            let pad = (hi - lo) * 0.04;
            if !(from_zero && lo == 0.0) {
                lo -= pad;
            }
            if !(from_zero && hi == 0.0) {
                hi += pad;
            }
        }
        Axis { lo, hi, log, p0, p1 }
    }
    pub fn px(&self, v: f64) -> f64 {
        let t = if self.log { (v.max(1e-300).ln() - self.lo.ln()) / (self.hi.ln() - self.lo.ln()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.p0 + t * (self.p1 - self.p0)
    }
    pub fn ticks(&self) -> Vec<f64> {
        if self.log {
            let mut t = vec![];
            let mut e = self.lo.log10().round() as i32;
            let top = self.hi.log10().round() as i32;
            let step = ((top - e) / 6 + 1).max(1);
            while e <= top {
                t.push(10f64.powi(e));
                e += step;
            }
            return t;
        }
        let span = self.hi - self.lo;
        let raw = span / 5.0;
        let mag = 10f64.powf(raw.log10().floor());
        let step = [1.0, 2.0, 2.5, 5.0, 10.0].iter().map(|m| m * mag).find(|s| span / s <= 6.5).unwrap_or(10.0 * mag);
        let mut v = (self.lo / step).ceil() * step;
        let mut out = vec![];
        while v <= self.hi + step * 1e-9 && out.len() < 12 {
            out.push(if v.abs() < step * 1e-9 { 0.0 } else { v });
            v += step;
        }
        out
    }
}

/// A chart in data coordinates.
pub struct Plot {
    pub xa: Axis,
    pub ya: Axis,
    pub c: Canvas,
    under: String,
    legend: Vec<(String, String)>,
    pub legend_left: bool,
    pub legend_bottom: bool,
    pub legend_center: bool,
}

impl Plot {
    /// Axes over these x and y values (`ylog`: logarithmic vertical axis; `zero`: include 0 on each axis).
    pub fn new(xs: &[f64], ys: &[f64], xlog: bool, ylog: bool, zero: (bool, bool)) -> Plot {
        let mut ya = Axis::over(ys, ylog, Y1, Y0, zero.1);
        if !ya.log {
            ya.hi += (ya.hi - ya.lo) * 0.12;                 // room at the top for the legend and the labels
        }
        Plot { xa: Axis::over(xs, xlog, X0, X1, zero.0), ya, c: Canvas::new(340.0), under: String::new(), legend: vec![],
               legend_left: false, legend_bottom: false, legend_center: false }
    }
    pub fn p(&self, x: f64, y: f64) -> (f64, f64) {
        (self.xa.px(x).clamp(X0 - 40.0, X1 + 40.0), self.ya.px(y).clamp(Y0 - 30.0, Y1 + 30.0))
    }
    fn path(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for &(x, y) in pts {
            if !x.is_finite() || !y.is_finite() || (self.ya.log && y <= 0.0) || (self.xa.log && x <= 0.0) {
                pen = false;
                continue;
            }
            let (px, py) = self.p(x, y);
            d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        d.trim_end().to_string()
    }
    /// A curve in a line class (axp-c, axp-c2, axp-c3, axp-c4); a non-finite point breaks it.
    pub fn line(&mut self, pts: &[(f64, f64)], class: &str, legend: &str) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.c.raw(&format!("<path class=\"{class}\" d=\"{d}\"/>"));
        }
        if !legend.is_empty() {
            self.legend.push((legend.to_string(), class.to_string()));
        }
    }
    /// A dashed guide line.
    pub fn dashed(&mut self, pts: &[(f64, f64)], legend: &str) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.c.raw(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"{d}\"/>"));
        }
        if !legend.is_empty() {
            self.legend.push((legend.to_string(), "axp-c4".into()));
        }
    }
    /// A filled polygon (an area class: axp-a, axp-a2, ...) drawn under the curves.
    pub fn area(&mut self, pts: &[(f64, f64)], class: &str) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.under.push_str(&format!("<path class=\"{class}\" d=\"{d} Z\"/>"));
        }
    }
    /// A filled rectangle between two data corners, drawn under the curves.
    pub fn rect(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, class: &str) {
        let (a, b) = (self.p(x0, y0), self.p(x1, y1));
        self.under.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                     a.0.min(b.0), a.1.min(b.1), (a.0 - b.0).abs(), (a.1 - b.1).abs()));
    }
    /// Points (a dot class: axp-d, axp-d2, ...).
    pub fn dots(&mut self, pts: &[(f64, f64)], class: &str, legend: &str) {
        for &(x, y) in pts {
            if x.is_finite() && y.is_finite() && (!self.ya.log || y > 0.0) {
                let (px, py) = self.p(x, y);
                self.c.raw(&format!("<circle class=\"{class}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"4\"/>"));
            }
        }
        if !legend.is_empty() {
            self.legend.push((legend.to_string(), class.to_string()));
        }
    }
    /// A marked point with its label (the visitor's value), the label kept inside the frame.
    pub fn mark(&mut self, x: f64, y: f64, label: &str, above: bool) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        let (px, py) = self.p(x, y);
        self.c.raw(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>"));
        let w = label.chars().count() as f64 * 9.0;
        let tx = if px + 10.0 + w <= 516.0 { px + 10.0 } else { (px - 10.0 - w).max(6.0) };
        let ty = if above { py - 10.0 } else { py + 20.0 };
        put(&mut self.c, tx, ty.clamp(Y0 + 12.0, Y1 + 12.0), "start", "axp-mt", label);
    }
    /// A grey label at a data point.
    pub fn label(&mut self, x: f64, y: f64, anchor: &str, t: &str) {
        let (px, py) = self.p(x, y);
        put(&mut self.c, px, py, anchor, "axp-lt", t);
    }
    /// A green value at a data point.
    pub fn value(&mut self, x: f64, y: f64, anchor: &str, t: &str) {
        let (px, py) = self.p(x, y);
        put(&mut self.c, px, py, anchor, "axp-vt", t);
    }
    /// The finished chart body: grid, ticks, frame, the drawing, legend, axis names and the header.
    pub fn finish(mut self, xlab: &str, ylab: &str, header: &str) -> String {
        let mut out = String::new();
        for t in self.xa.ticks() {
            let x = self.xa.px(t);
            if !(X0 - 0.5..=X1 + 0.5).contains(&x) {
                continue;
            }
            out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{Y0}\" x2=\"{x:.1}\" y2=\"{Y1}\"/>\
                                   <text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                                  Y1 + 20.0, num(t)));
        }
        for t in self.ya.ticks() {
            let y = self.ya.px(t);
            if !(Y0 - 0.5..=Y1 + 0.5).contains(&y) {
                continue;
            }
            out.push_str(&format!("<line class=\"axp-g\" x1=\"{X0}\" y1=\"{y:.1}\" x2=\"{X1}\" y2=\"{y:.1}\"/>\
                                   <text class=\"axp-t\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                  X0 - 6.0, y + 5.0, num(t)));
        }
        out.push_str(&self.under);
        out.push_str(&format!("<rect class=\"axp-f\" x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/>", X1 - X0, Y1 - Y0));
        // the legend, one line each, inside the frame at the top
        let n = self.legend.len();
        let lw = self.legend.iter().map(|l| l.0.chars().count()).max().unwrap_or(0) as f64 * 7.6 + 30.0;
        for (i, (t, class)) in self.legend.iter().enumerate() {
            let y = if self.legend_bottom { Y1 - 8.0 - 17.0 * (n - 1 - i) as f64 } else { Y0 + 16.0 + 17.0 * i as f64 };
            let x = if self.legend_center { ((X0 + X1) / 2.0 - lw / 2.0).max(X0 + 4.0) } else if self.legend_left {
                X0 + 10.0
            } else {
                (X1 - 10.0 - lw).max(X0 + 4.0)
            };
            let sw = if class.starts_with("axp-d") {
                format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\"/>", x + 9.0, y - 4.0)
            } else if class.starts_with("axp-c4") {
                format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{x:.1},{:.1} L{:.1},{:.1}\"/>", y - 4.0, x + 20.0,
                        y - 4.0)
            } else if class.starts_with("axp-a") || class.starts_with("axp-b") || class.starts_with("axp-s") {
                format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"20\" height=\"10\"/>", y - 9.0)
            } else {
                format!("<path class=\"{class}\" d=\"M{x:.1},{:.1} L{:.1},{:.1}\"/>", y - 4.0, x + 20.0, y - 4.0)
            };
            out.push_str(&sw);
            put(&mut self.c, x + 26.0, y, "start", "axp-lt", t);
        }
        let _ = n;
        out.push_str(&self.c.s);
        if !xlab.is_empty() {
            out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>",
                                  (X0 + X1) / 2.0, esc(&clip(xlab, 60))));
        }
        if !ylab.is_empty() {
            out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(18,{:.1}) rotate(-90)\" \
                                   text-anchor=\"middle\">{}</text>", (Y0 + Y1) / 2.0, esc(&clip(ylab, 34))));
        }
        if !header.is_empty() {
            out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">{}</text>", esc(&clip(header, 55))));
        }
        out
    }
}

/// Text kept inside the 520-wide canvas by a conservative width estimate (9 px a character for marked values, 7.8 px
/// otherwise), so the figure check (which estimates 9 and 7.6) always passes.
pub fn put(c: &mut Canvas, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
    let w = t.chars().count() as f64 * if class == "axp-mt" || class == "axp-bt" { 9.0 } else { 7.8 };
    let left = match anchor {
        "middle" => x - w / 2.0,
        "end" => x - w,
        _ => x,
    };
    let shift = if left < 4.0 { 4.0 - left } else if left + w > 516.0 { 516.0 - (left + w) } else { 0.0 };
    c.text(x + shift, y, anchor, class, t);
}

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub fn clip(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("{}...", c[..n.saturating_sub(3)].iter().collect::<String>().trim_end()) }
}
