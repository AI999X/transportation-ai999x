"""Transportation procedures: braking and stopping, vehicle performance and dynamics, highway geometry, traffic flow
and queues, railway operations, route planning and travel demand (domain code, loaded by the pack; enabled only once
tools/certify_procedures.py has certified them).

Every capability states its inputs, outputs, conditions, method and LaTeX, the documented question form it reads,
worked examples (their outputs were computed by the reference, not by the procedure), an INDEPENDENT reference
computation used only by certification (the checking libraries are imported inside the reference functions: SciPy
ODE integration and quadrature, NumPy linear algebra, NetworkX, python-control, SymPy) and a deliberately wrong
variant that the checks must reject. The shipped code (run, verify, parse) is plain Python (math, re, heapq, mpmath).

Conventions (stated once, used everywhere):
- SI inside: m, s, m/s, m/s^2, kg, N, W. solve() takes SI numbers or any unit of the shared reader; the question
  reader also converts the usual transport units: km/h, mph, ft/s; km, ft, mi; min, h; t, lb; kN, lbf; kW, hp;
  veh/h (vph, pc/h, pcu/h, veh/h/ln) and veh/km (veh/mi, pc/km). A traffic flow is vehicles per second inside and is
  shown in veh/h; a traffic density is vehicles per metre inside and is shown in veh/km.
- Gravity g = 9.81 m/s^2 (as AASHTO and most transport texts); the unit "g" of an acceleration is 9.81 m/s^2.
- Grades and superelevation rates are decimal fractions, positive uphill (G = 0.03 is a 3 % upgrade); the reader
  takes "3%", "0.03" or "30 per mille" (rail); a bare number of 1 or more is refused as ambiguous. On a grade the
  normal force is W cos(theta) and the grade force W sin(theta) with tan(theta) = G, unless a procedure states the
  textbook small-grade form g (f + G) (stopping and skid distances).
- Friction coefficients are plain numbers (longitudinal f for braking, side friction f for curves).
- Perception-reaction times, signal times and headways are in seconds.
- Angles (steering, bank) are read in degrees unless written in rad; angle outputs are shown in degrees.
- Curves: the point-mass model on a superelevated curve, v^2/(g R) = (e + f)/(1 - e f), exact (not the simplified
  e + f form). Crest vertical curves use AASHTO eye height 1.08 m and object height 0.60 m unless stated.
- Lists carry no unit; each list input states its unit in its meaning (edge weights in km, speeds in km/h, travel
  times in min, ...).
- Outputs built from exp/log/trig/powers are rounded to 12 significant digits so that every platform prints the same
  digits; values that are zero by balance (below 1e-12 of their scale) are written as exactly 0.

Question form (one per procedure): the capability phrase, "with" (or a colon), then `name = value` pairs separated by
commas or "and" (`Braking distance with speed = 100 km/h and deceleration = 3.4 m/s^2`); physical values carry a unit;
lists are written in brackets (`speeds = [52, 61, 48]`), graphs as bracketed edge lists (`roads = [A-B 4, B-C 2.5]`).
Any other number or an unknown name is refused, never guessed.
"""
from __future__ import annotations

import heapq
import math
import re

# =============================================================================================== constants
G = 9.81                      # gravity, m/s^2 (transport engineering convention)
G_ = G                        # the same, for functions whose grade argument is named G
KMH = 1 / 3.6                 # km/h -> m/s
HP = 745.6998715822702        # mechanical horsepower, W
LB = 0.45359237               # pound mass, kg
LBF = 4.4482216152605         # pound force, N
FT = 0.3048
MI = 1609.344


def _r(x):
    """12 significant digits, never a negative zero (lists element by element)."""
    if isinstance(x, (bool, str)) or x is None:
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    x = float(x)
    if x == 0.0 or not math.isfinite(x):
        return 0.0 if x == 0.0 else x
    return float(f"{x:.12g}") + 0.0


def _snap(x, scale):
    return 0.0 if abs(x) <= 1e-12 * abs(scale) else x


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return 1.0
        return max([_rel(x, y) for x, y in zip(a, b)] or [0.0])
    if isinstance(a, str) or isinstance(b, str):
        return 0.0 if a == b else 1.0
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return 1.0
    if not (math.isfinite(a) and math.isfinite(b)):
        return 1.0
    d = abs(a - b)
    if d <= 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _need(cond, message):
    if not cond:
        raise ValueError(message)


def _finite(**kw):
    for k, v in kw.items():
        if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
            raise ValueError(f"{k} must be a finite number")


def _pos(**kw):
    _finite(**kw)
    for k, v in kw.items():
        if v <= 0:
            raise ValueError(f"{k} must be positive")


def _nonneg(**kw):
    _finite(**kw)
    for k, v in kw.items():
        if v < 0:
            raise ValueError(f"{k} must not be negative")


def _vec(name, v, n=None, positive=False, nonneg=False):
    _need(isinstance(v, (list, tuple)) and len(v) > 0, f"{name} must be a list of numbers")
    for x in v:
        _need(isinstance(x, (int, float)) and not isinstance(x, bool) and math.isfinite(x), f"{name} must hold finite numbers")
        if positive:
            _need(x > 0, f"every entry of {name} must be positive")
        if nonneg:
            _need(x >= 0, f"every entry of {name} must be zero or positive")
    if n is not None:
        _need(len(v) == n, f"{name} must have {n} entries")
    return [float(x) for x in v]


def _mat(name, m, rows=None, cols=None, nonneg=False, positive=False):
    _need(isinstance(m, (list, tuple)) and len(m) > 0, f"{name} must be a matrix")
    out = [_vec(name, r, positive=positive, nonneg=nonneg) for r in m]
    _need(len({len(r) for r in out}) == 1, f"{name} must be rectangular")
    if rows is not None:
        _need(len(out) == rows, f"{name} must have {rows} rows")
    if cols is not None:
        _need(len(out[0]) == cols, f"{name} must have {cols} columns")
    return out


def _bisect_root(f, a, b, iters=400):
    """Root of a continuous f with f(a), f(b) of opposite signs, to the last representable bit."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need(fa * fb < 0, "no root in the admissible range")
    for _ in range(iters):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm < 0) == (fa < 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def _theta(Gr):
    return math.atan(Gr)


# =============================================================================================== units of the reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_UNITS = {
    "length": {"m": 1, "mm": 1e-3, "cm": 1e-2, "km": 1e3, "ft": FT, "feet": FT, "foot": FT, "mi": MI, "mile": MI,
               "miles": MI, "yd": 0.9144, "in": 0.0254, "metres": 1, "meters": 1, "metre": 1, "meter": 1},
    "speed": {"m/s": 1, "km/h": KMH, "kmh": KMH, "kph": KMH, "km/hr": KMH, "kmph": KMH, "mph": MI / 3600,
              "mi/h": MI / 3600, "ft/s": FT, "fps": FT},
    "time": {"s": 1, "sec": 1, "secs": 1, "second": 1, "seconds": 1, "min": 60, "mins": 60, "minute": 60, "minutes": 60,
             "h": 3600, "hr": 3600, "hrs": 3600, "hour": 3600, "hours": 3600},
    "accel": {"m/s^2": 1, "m/s2": 1, "m/s/s": 1, "ft/s^2": FT, "ft/s2": FT, "g": G},
    "mass": {"kg": 1, "t": 1e3, "tonne": 1e3, "tonnes": 1e3, "lb": LB, "lbs": LB},
    "force": {"N": 1, "kN": 1e3, "MN": 1e6, "lbf": LBF, "kip": 1e3 * LBF},
    "power": {"W": 1, "kW": 1e3, "MW": 1e6, "hp": HP},
    "area": {"m^2": 1, "m2": 1, "ft^2": FT ** 2, "ft2": FT ** 2},
    "air_density": {"kg/m^3": 1, "kg/m3": 1},
    "flow": {"veh/h": 1 / 3600, "vph": 1 / 3600, "veh/hr": 1 / 3600, "pc/h": 1 / 3600, "pcu/h": 1 / 3600,
             "pcph": 1 / 3600, "veh/h/ln": 1 / 3600, "pc/h/ln": 1 / 3600, "vphpl": 1 / 3600, "pcphpl": 1 / 3600,
             "veh/min": 1 / 60, "veh/s": 1, "1/h": 1 / 3600, "/h": 1 / 3600, "1/s": 1, "trains/h": 1 / 3600,
             "customers/h": 1 / 3600, "arrivals/h": 1 / 3600, "per hour": 1 / 3600, "1/min": 1 / 60, "/min": 1 / 60},
    "density": {"veh/km": 1e-3, "vpk": 1e-3, "pc/km": 1e-3, "pcu/km": 1e-3, "veh/km/ln": 1e-3, "pc/km/ln": 1e-3,
                "veh/mi": 1 / MI, "vpm": 1 / MI, "pc/mi": 1 / MI, "veh/mi/ln": 1 / MI, "pc/mi/ln": 1 / MI,
                "veh/m": 1, "1/km": 1e-3},
    "number": {"": 1},
    "fraction": {"": 1, "%": 0.01, "percent": 0.01},
    "grade": {"": 1, "%": 0.01, "percent": 0.01, "‰": 1e-3, "permille": 1e-3, "permil": 1e-3},
    "angle": {"": math.pi / 180, "°": math.pi / 180, "deg": math.pi / 180, "degree": math.pi / 180,
              "degrees": math.pi / 180, "rad": 1, "radian": 1, "radians": 1},
    "cornering": {"N/rad": 1, "kN/rad": 1e3, "N/deg": 180 / math.pi, "kN/deg": 180e3 / math.pi},
    "stiffness": {"N/m": 1, "kN/m": 1e3, "N/mm": 1e3, "MN/m": 1e6},
    "damping": {"N*s/m": 1, "Ns/m": 1, "N.s/m": 1, "kN*s/m": 1e3, "kNs/m": 1e3, "kg/s": 1},
    "inertia": {"kg*m^2": 1, "kgm^2": 1, "kg m^2": 1, "kg*m2": 1, "t*m^2": 1e3},
    "frequency": {"Hz": 1, "hz": 1},
    "davis_b": {"N*s/m": 1, "N/(m/s)": 1, "kN*s/m": 1e3, "kN/(m/s)": 1e3, "N/(km/h)": 3.6, "kN/(km/h)": 3.6e3},
    "davis_c": {"N*s^2/m^2": 1, "N/(m/s)^2": 1, "kN*s^2/m^2": 1e3, "kN/(m/s)^2": 1e3, "N/(km/h)^2": 12.96,
                "kN/(km/h)^2": 12960.0},
    "minutes": {"": 1, "min": 1, "mins": 1, "minutes": 1},
}
_STRICT_FRACTION = {"grade"}          # a bare grade of 1 or more is refused (3 could mean 3 % or 300 %)


def _norm_unit(u: str) -> str:
    u = u.strip().replace("²", "^2").replace("³", "^3").replace("·", "*").replace("−", "-").replace("º", "°")
    u = re.sub(r"\bper\s+mille\b|\bper\s+mil\b", "permille", u)
    u = re.sub(r"\bper\s+cent\b", "percent", u)
    u = re.sub(r"\bvehicles?\b", "veh", u)
    u = re.sub(r"\s+per\s+", "/", u)
    if u.lower() in ("per hour",):
        return "per hour"
    return re.sub(r"\s+", "", u)


def _convert(value: float, unit: str, table: str):
    t = _UNITS[table]
    u = _norm_unit(unit)
    if u in t:
        si = value * t[u]
    else:
        low = {k.lower(): v for k, v in t.items() if len(k) >= 3 and k not in ("MN", "MW")}
        if len(u) >= 3 and u.lower() in low:
            si = value * low[u.lower()]
        else:
            return None
    if table in _STRICT_FRACTION and u == "" and abs(value) >= 1:
        return None
    return si


# =============================================================================================== the procedure registry
PROCEDURES: dict = {}
_ROUTES: list = []          # (phrase, key): a question is routed to the procedure whose phrase is the longest found
_SPEC: dict = {}            # key -> internal reading information


def _in(sym, kind, table, meaning, names=(), show="", required=True, typ=None, **extra):
    d = {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}
    if typ:
        d["type"] = typ
    d.update(extra)
    return d, table, [sym] + list(names), show


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _route(head: str):
    h = " " + re.sub(r"\s+", " ", head.lower().replace("-", " ")) + " "
    for phrase, key in _ROUTES:
        if re.search(r"(?<![a-z])" + re.escape(phrase.replace("-", " ")) + r"(?![a-z])", h):
            return key
    return None


def _show_value(v):
    if isinstance(v, (list, tuple)):
        if v and isinstance(v[0], (list, tuple)):
            return "[" + "; ".join(" ".join(f"{x:g}" for x in row) for row in v) + "]"
        return "[" + ", ".join(f"{x:g}" if isinstance(x, (int, float)) else str(x) for x in v) + "]"
    if isinstance(v, str):
        return v
    if isinstance(v, bool):
        return "yes" if v else "no"
    return f"{v:g}"


def _read(key, body):
    """'name = value unit, name = value unit and name = [a, b, c]' -> SI values; None for anything else."""
    info = _SPEC[key]
    body = body.strip().rstrip(".?! ")
    lists = []

    def keep(m):
        lists.append(m.group(1))
        return f"\x00{len(lists) - 1}\x00"
    body = re.sub(r"\[([^\[\]]*)\]", keep, body)
    items = [s for s in re.split(r"\s*(?:,|;|\band\b)\s*", body) if s.strip()]
    if not items:
        return None
    out = {}
    for item in items:
        m = re.fullmatch(r"([A-Za-z][\w ()'/\-]*?)\s*=\s*(.+)", item.strip())
        if not m:
            return None
        name, val = re.sub(r"\s+", " ", m.group(1).strip()), m.group(2).strip()
        sym = info["names"].get(name) or info["names"].get(name.lower())
        if sym is None or sym in out:
            return None
        spec, table = info["inputs"][sym]
        typ = spec.get("type", "number")
        lm = re.fullmatch("\x00(\\d+)\x00", val)
        if typ == "vector":
            if not lm:
                return None
            nums = [x for x in re.split(r"[\s,;]+", lists[int(lm.group(1))].strip()) if x]
            if not nums or not all(re.fullmatch(_NUM, x) for x in nums):
                return None
            out[sym] = ", ".join(nums)
            continue
        if typ == "matrix":
            if not lm:
                return None
            rows = [r for r in lists[int(lm.group(1))].split(";") if r.strip()]
            cells = [[x for x in re.split(r"[\s,]+", r.strip()) if x] for r in rows]
            if not cells or not all(c and all(re.fullmatch(_NUM, x) for x in c) for c in cells):
                return None
            out[sym] = "; ".join(" ".join(c) for c in cells)
            continue
        if typ == "edges":
            if not lm:
                return None
            out[sym] = lists[int(lm.group(1))].strip()
            continue
        if lm:
            return None
        if typ in ("choice", "boolean", "text"):
            if not re.fullmatch(r"[\w\-]+", val):
                return None
            out[sym] = val
            continue
        nm = re.match("(" + _NUM + r")\s*(.*)$", val)
        if not nm:
            return None
        number, unit = float(nm.group(1)), nm.group(2)
        if typ == "integer":
            if unit.strip() or not re.fullmatch(r"[-+]?\d+", nm.group(1)):
                return None
            out[sym] = int(nm.group(1))
            continue
        si = _convert(number, unit, table)                 # an unknown unit (or a second number) is refused
        if si is None:
            return None
        # flows, densities and angles go to the runtime as numbers with a unit it reads (veh/h -> '600 1/h'), so the
        # answer echoes them in the unit the reader understands; everything else is passed in SI
        out[sym] = {"flow": lambda x: f"{x * 3600!r} 1/h", "density": lambda x: f"{x * 1000!r} 1/km",
                    "angle": lambda x: f"{math.degrees(x)!r} deg"}.get(table, lambda x: x)(si)
    return out


def _parser(key):
    def parse(question):
        if not isinstance(question, str):
            return None
        m = re.search(r"\s+with\s+|\s*:\s+", question)
        if not m:
            return None
        head, body = question[:m.start()], question[m.end():]
        if re.search(r"\d", head) or _route(head) != key:
            return None
        return _read(key, body)
    return parse


def _verifier(run, extra=None):
    def verify(inputs, outputs):
        try:
            again = run(**inputs)
        except Exception:
            return 1.0
        worst = 0.0
        for k, v in again.items():
            if k not in outputs:
                return 1.0
            worst = max(worst, _rel(outputs[k], v))
        if extra is not None:
            try:
                worst = max(worst, float(extra(inputs, outputs)))
            except Exception:
                return 1.0
        return worst
    return verify


def _edges_from_text(v):
    """'A-B 4, B-C 2.5' -> [('A', 'B', 4.0), ...] (examples only; the runtime reads user input itself)."""
    if not isinstance(v, str):
        return [tuple(e) for e in v]
    out = []
    for part in re.split(r"[;,]\s*", v.strip()):
        if not part.strip():
            continue
        m = re.fullmatch(r"\s*([^\s\-]+)\s*-\s*([^\s\-]+)(?:\s+(" + _NUM + r"))?\s*", part)
        out.append((m.group(1), m.group(2), float(m.group(3))) if m.group(3) else (m.group(1), m.group(2)))
    return out


def proc(key, name, topic, purpose, inputs, outputs, run, reference, wrong, rand, examples, phrases, *,
         conditions=(), method="", latex="", invalid=(), extra_verify=None, primary=None):
    """Register one procedure. `examples` are dicts of input values written in each input's display unit (lists for
    vectors); their outputs come from EXAMPLE_OUTPUTS (computed by the reference) or, until then, from `run`."""
    specs, names, shows = {}, {}, {}
    for d, table, aliases, show in inputs:
        specs[d["symbol"]] = (d, table)
        for a in aliases:
            names[a] = d["symbol"]
            names.setdefault(a.lower(), d["symbol"])
        shows[d["symbol"]] = (aliases[1] if len(aliases) > 1 else aliases[0], show, table)
    _SPEC[key] = {"inputs": specs, "names": names}
    for ph in phrases:
        _ROUTES.append((ph.lower(), key))
    _ROUTES.sort(key=lambda r: -len(r[0]))

    def to_si(ex):
        out = {}
        for s, v in ex.items():
            d, table = specs[s]
            t = d.get("type")
            if t in ("vector", "matrix", "choice", "boolean", "text", "edges"):
                out[s] = [list(r) if isinstance(r, (list, tuple)) else r for r in v] if isinstance(v, (list, tuple)) else v
            elif t == "integer":
                out[s] = int(v)
            else:
                out[s] = _convert(float(v), shows[s][1], table)
        return out

    def as_text(ex, drop=None):
        parts = []
        for s, v in ex.items():
            if s == drop:
                continue
            label, show, _t = shows[s]
            d = specs[s][0]
            if d.get("type") == "edges":
                parts.append(f"{label} = [{v}]")
            else:
                parts.append(f"{label} = {_show_value(v)}" + (f" {show}" if show and d.get("type") not in
                                                               ("vector", "matrix") else ""))
        text = ", ".join(parts[:-1]) + (" and " if len(parts) > 1 else "") + parts[-1]
        return f"{phrases[0][0].upper()}{phrases[0][1:]} with {text}."

    def coerce_ex(si):
        out = {}
        for s, v in si.items():
            d = specs[s][0]
            if d.get("type") == "vector":
                out[s] = [float(x) for x in v]
            elif d.get("type") == "matrix":
                out[s] = [[float(x) for x in r] for r in v]
            elif d.get("type") == "edges":
                out[s] = _edges_from_text(v)
            elif d.get("type") == "boolean" and isinstance(v, str):
                out[s] = v.lower() in ("true", "yes", "1")
            else:
                out[s] = v
        return out

    stored = EXAMPLE_OUTPUTS.get(key)
    exs = []
    for i, ex in enumerate(examples):
        si = to_si(ex)
        outs = stored[i] if stored and i < len(stored) else {k: v for k, v in run(**coerce_ex(si)).items()}
        exs.append({"inputs": si, "outputs": outs})
    questions = [{"q": as_text(ex), "outputs": exs[i]["outputs"]} for i, ex in enumerate(examples[:2])]
    req = [d["symbol"] for d, *_ in inputs if d.get("required")]
    if len(req) >= 2:
        questions.append({"q": as_text(examples[0], drop=req[-1]), "outcome": "NEEDS_INPUT"})
    questions.append({"q": as_text(examples[0])[:-1] + " and colour = 3.", "outcome": "UNRECOGNIZED_FORMAT"})
    for bad in invalid:
        questions.append({"q": as_text(bad), "outcome": "INVALID_INPUT"})
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": [d for d, *_ in inputs], "outputs": outputs,
        "primary": primary or outputs[0]["symbol"], "conditions": list(conditions), "method": method, "latex": latex,
        "run": run, "verify": _verifier(run, extra_verify), "parse": _parser(key), "reference": reference,
        "wrong": wrong, "random_inputs": rand, "templates": [q["q"] for q in questions[:2]], "examples": exs,
        "questions": questions, "keywords": [p.lower() for p in phrases]}


# =============================================================================================== reference helpers
def _brentq(f, a, b):
    from scipy.optimize import brentq
    return brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500)


def _quad(f, a, b, **kw):
    from scipy.integrate import quad
    v, _err = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=500, **kw)
    return v


def _ivp(fun, t_end, y0, events=None, atol=1e-12):
    """DOP853 at rtol 1e-13 with dense output (the references' ODE integrator)."""
    from scipy.integrate import solve_ivp
    sol = solve_ivp(fun, (0.0, t_end), [float(v) for v in y0], method="DOP853", rtol=1e-13, atol=atol,
                    dense_output=True, events=events)
    if sol.status < 0:
        raise RuntimeError(sol.message)
    return sol


def _first_crossing(sol, idx, target, t_hi=None):
    """First time component `idx` of a dense ODE solution reaches `target` (scan the steps, then brentq)."""
    ts = sol.t if t_hi is None else [t for t in sol.t if t <= t_hi] + [t_hi]
    prev_t, prev_f = ts[0], sol.sol(ts[0])[idx] - target
    if prev_f == 0:
        return prev_t
    for t in ts[1:]:
        f = sol.sol(t)[idx] - target
        if f == 0:
            return t
        if (f < 0) != (prev_f < 0):
            return _brentq(lambda s: sol.sol(s)[idx] - target, prev_t, t)
        prev_t, prev_f = t, f
    raise RuntimeError("the target is not reached in the integration window")


# Outputs of the documented examples, computed by each capability's INDEPENDENT reference on the build machine and
# rounded to 12 significant digits; certification checks the procedure against them.
EXAMPLE_OUTPUTS: dict = {
    "transport_braking_distance": [{"d": 113.471314452, "t": 8.16993464052}, {"d": 13.778659612, "t": 1.98412698413}],
    "transport_stopping_sight_distance": [{"SSD": 182.915758896, "d_r": 69.4444444444, "d_b": 113.471314452}, {"SSD": 134.210493198, "d_r": 55.5555555556, "d_b": 78.6549376424}],
    "transport_skid_speed": [{"v": 18.792551716, "v_kmh": 67.6531861777}, {"v": 20.5862089759, "v_kmh": 74.1103523133}],
    "transport_speed_reduction_distance": [{"d": 59.4484993809, "t": 3.2925322734}, {"d": 163.360255103, "t": 6.53441020413}],
    "transport_impact_speed": [{"v_impact": 14.5938343145, "v_impact_kmh": 52.5378035323, "d_stop": 78.0920829086}, {"v_impact": 14.4605593868, "v_impact_kmh": 52.0580137923, "d_stop": 45.2255553938}],
    "transport_acceleration_distance": [{"d": 270.061728395, "t": 13.8888888889}, {"d": 38.5802469136, "t": 5.55555555556}],
    "transport_yellow_change_interval": [{"Y": 3.27686703097, "x_c": 45.5120420967}, {"Y": 3.92008316397, "x_c": 65.3347193995}],
    "transport_overtaking_manoeuvre": [{"t": 8.94427191, "d": 233.91639825, "v_f": 32.8608523094}, {"t": 6.32455532034, "d": 127.841046116, "v_f": 26.5379995296}],
    "transport_braking_with_air_resistance": [{"S": 84.829494117, "t": 5.13282351187}, {"S": 75.4845928514, "t": 4.19776049664}],
    "transport_coast_down": [{"t": 42.2501297979, "d": 917.248498508}, {"t": 54.1059950857, "d": 717.140524841}],
    "transport_acceleration_under_power": [{"t": 8.32025289502, "d": 162.053710134}, {"t": 20.2750958989, "d": 419.709002307}],
    "transport_traction_limited_acceleration": [{"F_max": 5695.35610619, "a_max": 3.6791840708, "W_f": 7595.80486726, "W_r": 7119.19513274}, {"F_max": 3783.85714286, "a_max": 3.15321428571, "W_f": 6306.42857143, "W_r": 5465.57142857}],
    "transport_braking_weight_transfer": [{"W_f": 10313.8888889, "W_r": 4401.11111111, "dW": 2138.88888889, "front_share": 0.700909880319}, {"W_f": 12330.6666667, "W_r": 7289.33333333, "dW": 1866.66666667, "front_share": 0.628474345906}],
    "transport_lateral_load_transfer": [{"W_out": 7676.85483871, "W_in": 7038.14516129, "dW": 319.35483871}, {"W_out": 15191.0, "W_in": 6391.0, "dW": 4400.0}],
    "transport_rollover_threshold": [{"SSF": 1.29166666667, "v_roll": 35.5966992852, "v_roll_kmh": 128.148117427}, {"SSF": 0.473684210526, "v_roll": 32.8286202856, "v_roll_kmh": 118.183033028}],
    "transport_curve_safe_speed": [{"v": 23.0993893616, "v_kmh": 83.1578017018}, {"v": 18.508259454, "v_kmh": 66.6297340344}],
    "transport_minimum_curve_radius": [{"R_min": 389.499251205}, {"R_min": 121.856333131}],
    "transport_side_friction_demand": [{"f": 0.191907630522}, {"f": 0.203793014279}],
    "transport_superelevation_required": [{"e": 0.0536631838159, "e_percent": 5.36631838159}, {"e": 0.0513200518352, "e_percent": 5.13200518352}],
    "transport_crest_curve_length": [{"L": 260.070844016, "case": "S < L"}, {"L": 136.802484496, "case": "S > L"}],
    "transport_crest_curve_sight_distance": [{"S": 198.694809508, "case": "S < L"}, {"S": 139.66563146, "case": "S > L"}],
    "transport_horizontal_sight_offset": [{"M": 10.6036064308}, {"M": 9.97086771668}],
    "transport_sight_distance_from_offset": [{"S": 147.339310509}, {"S": 102.258626606}],
    "transport_clothoid_coordinates": [{"x": 79.8578947851, "y": 3.55104311752, "theta": 7.63943726841}, {"x": 59.994600225, "y": 0.599961429676, "theta": 1.71887338539}],
    "transport_greenshields_capacity": [{"q_max": 3000.0, "k_m": 60.0, "v_m": 50.0}, {"q_max": 2250.0, "k_m": 75.0, "v_m": 30.0}],
    "transport_greenshields_speed_at_flow": [{"k_1": 25.3589838486, "v_1": 78.8675134595, "k_2": 94.6410161514, "v_2": 21.1324865405}, {"k_1": 17.0849737787, "v_1": 70.2371578407, "k_2": 122.915026221, "v_2": 9.76284215926}],
    "transport_greenshields_fit": [{"vf": 95.3537121426, "kj": 120.140541639, "q_max": 2863.96165603, "r2": 0.998689915789}, {"vf": 73.2005659424, "kj": 130.337774979, "q_max": 2385.19972303, "r2": 0.989428812144}],
    "transport_greenberg_fit": [{"v0": 36.43837575, "kj": 197.836221849, "q_max": 2651.98056847, "r2": 0.957329344359}, {"v0": 31.5044481359, "kj": 186.575457954, "q_max": 2162.37947699, "r2": 0.987735633397}],
    "transport_underwood_fit": [{"vf": 122.301529724, "k0": 57.3960457026, "q_max": 2582.37542409, "r2": 0.974955848924}, {"vf": 97.9236480774, "k0": 59.7023022881, "q_max": 2150.7215247, "r2": 0.988029625238}],
    "transport_greenberg_capacity": [{"q_max": 1545.09365292, "k_m": 51.503121764}, {"q_max": 1655.45748527, "k_m": 66.2182994109}],
    "transport_underwood_capacity": [{"q_max": 1839.39720586, "v_m": 36.7879441171, "k_m": 50.0}, {"q_max": 1030.06243528, "v_m": 29.4303552937, "k_m": 35.0}],
    "transport_shockwave_speed": [{"u": -10.9090909091, "u_ms": -3.0303030303, "direction": "upstream"}, {"u": -17.2727272727, "u_ms": -4.79797979798, "direction": "upstream"}],
    "transport_greenshields_shockwave": [{"u": 6.92307692308, "u_ms": 1.92307692308}, {"u": 45.8333333333, "u_ms": 12.7314814815}],
    "transport_signal_queue_shockwave": [{"L_max": 158.73015873, "t_max": 68.5714285714, "w_queue": -8.33333333333, "w_discharge": -20.0}, {"L_max": 65.4970760234, "t_max": 45.4736842105, "w_queue": -5.18518518519, "w_discharge": -15.2380952381}],
    "transport_signal_queue_uniform_delay": [{"Q_max": 6.66666666667, "t_clear": 20.0, "D_total": 200.0, "d_uniform": 17.1428571429, "P_stopped": 0.857142857143}, {"Q_max": 7.77777777778, "t_clear": 25.4545454545, "D_total": 235.101010101, "d_uniform": 13.2244318182, "P_stopped": 0.755681818182}],
    "transport_bottleneck_queue": [{"Q_max": 700.0, "T": 4950.0, "T_min": 82.5, "D_total": 481.25}, {"Q_max": 166.666666667, "T": 2400.0, "T_min": 40.0, "D_total": 55.5555555556}],
    "transport_interval_queue": [{"queue": [0.0, 20.0, 60.0, 70.0, 50.0, 10.0], "Q_max": 70.0, "served": [90.0, 100.0, 100.0, 100.0, 100.0, 100.0]}, {"queue": [0.0, 10.0, 50.0, 45.0, 20.0], "Q_max": 50.0, "served": [30.0, 45.0, 30.0, 45.0, 45.0]}],
    "transport_signal_green_splits": [{"g": [34.2857142857, 28.5714285714, 17.1428571429], "X_c": 0.7875}, {"g": [29.7142857143, 22.2857142857], "X_c": 0.807692307692}],
    "transport_poisson_arrivals": [{"P_n": 0.224041807655, "P_le": 0.423190081127, "P_ge": 0.800851726529}, {"P_n": 0.0486107508296, "P_le": 0.118464411529, "P_ge": 0.930146339301}],
    "transport_headway_probability": [{"P_ge": 0.263597138116, "P_lt": 0.736402861884}, {"P_ge": 0.223130160148, "P_lt": 0.776869839852}],
    "transport_gap_acceptance_capacity": [{"c": 363.101002924}, {"c": 616.100542839}],
    "transport_time_and_space_mean_speed": [{"TMS": 58.1666666667, "SMS": 57.2529499157, "s": 8.03533861556}, {"TMS": 93.0, "SMS": 92.0116996561, "s": 10.4880884817}],
    "transport_spot_speed_percentiles": [{"p85": 68.6, "p50": 59.5, "p15": 50.05}, {"p85": 102.8, "p50": 92.0, "p15": 84.4}],
    "transport_safe_following_gap": [{"gap": 43.8528806584}, {"gap": 12.6}],
    "transport_mm1_queue": [{"rho": 0.8, "L": 3.99999999996, "Lq": 3.19999999996, "W": 35.9999999997, "Wq": 28.7999999997, "P0": 0.2}, {"rho": 0.666666666667, "L": 1.99999999999, "Lq": 1.33333333332, "W": 23.9999999999, "Wq": 15.9999999999, "P0": 0.333333333333}],
    "transport_mmc_queue": [{"P_wait": 0.461788959025, "Lq": 1.38536687706, "Wq": 3.32488050495, "W": 12.3248805049, "L": 5.13536687706}, {"P_wait": 0.742551566081, "Lq": 4.45530939646, "Wq": 17.8212375859, "W": 28.1069518716, "L": 7.02673796789}],
    "transport_md1_queue": [{"Lq": 1.60000000003, "Wq": 14.4000000002, "W": 21.6000000002, "L": 2.40000000003}, {"Lq": 0.666666666672, "Wq": 2.00000000002, "W": 4.00000000002, "L": 1.33333333334}],
    "transport_erlang_b_blocking": [{"P_block": 0.0694187690644, "carried": 111.669747712, "lost_per_h": 4.16512614387}, {"P_block": 0.119739188445, "carried": 8.80260811555, "lost_per_h": 4.78956753779}],
    "transport_fifo_queue_simulation": [{"waits": [0.0, 3.0, 7.0, 11.0, 13.0, 14.0, 18.0], "mean_wait": 9.42857142857, "max_wait": 18.0, "mean_time_in_system": 15.8571428571, "last_departure": 45.0, "max_queue": 4.0}, {"waits": [0.0, 0.0, 9.0, 9.0, 15.0, 16.0, 25.0, 23.0], "mean_wait": 12.125, "max_wait": 25.0, "mean_time_in_system": 24.125, "last_departure": 50.0, "max_queue": 5.0}],
    "transport_train_running_time": [{"T": 87.8703703704, "v_peak": 22.2222222222, "profile": "accelerate-cruise-brake"}, {"T": 51.9615242271, "v_peak": 23.0940107676, "profile": "accelerate-brake"}],
    "transport_rail_station_headway": [{"h": 70.202020202, "trains_per_h": 51.2805755396}, {"h": 47.8470065542, "trains_per_h": 75.2398166419}],
    "transport_train_braking_distance": [{"S": 701.698286147, "t": 42.4728683905}, {"S": 664.541799356, "t": 39.9643183673}],
    "transport_train_acceleration": [{"t": 86.5875384958, "d": 1222.69194342}, {"t": 105.135595834, "d": 1555.69198322}],
    "transport_fastest_route": [{"route": "A-C-E", "time": 9.66666666667, "distance": 12.5}, {"route": "P-R-S", "time": 7.8, "distance": 11.0}],
    "transport_travel_time_skim": [{"places": "A, B, C, D, E", "times": [[0.0, 4.0, 3.0, 7.75, 9.66666666667], [4.0, 0.0, 1.8, 3.75, 6.15], [3.0, 1.8, 0.0, 4.8, 6.66666666667], [7.75, 3.75, 4.8, 0.0, 2.4], [9.66666666667, 6.15, 6.66666666667, 2.4, 0.0]]}, {"places": "P, Q, R, S", "times": [[0.0, 6.0, 5.4, 7.8], [6.0, 0.0, 4.0, 6.4], [5.4, 4.0, 0.0, 2.4], [7.8, 6.4, 2.4, 0.0]]}],
    "transport_all_or_nothing_assignment": [{"flows": [0.0, 0.0, 800.0, 300.0, 800.0, 300.0, 0.0, 0.0, 300.0, 1000.0], "VHT": 150.0}, {"flows": [1000.0, 1400.0, 0.0, 1400.0, 0.0], "VHT": 276.666666667}],
    "transport_gravity_model": [{"trips": [[585.291609215, 216.598390059, 198.110000726], [494.749778142, 550.037708887, 455.212512971], [119.958612642, 133.363901055, 546.677486303]]}, {"trips": [[452.776813945, 47.223186055], [147.223186055, 552.776813945]]}],
    "transport_furness_balancing": [{"trips": [[128.005362822, 275.799789296, 196.194847882], [361.892088736, 129.955182633, 308.15272863], [60.1025484415, 194.24502807, 245.652423488]]}, {"trips": [[52.0, 78.0], [88.0, 22.0]]}],
    "transport_logit_mode_shares": [{"P": [0.345018612617, 0.514707287448, 0.140274099936], "logsum": -0.135843086208}, {"P": [0.689974481128, 0.310025518872], "logsum": 0.871100665948}],
    "transport_logit_from_attributes": [{"V": [-2.5, -3.35, -4.32], "P": [0.629152135572, 0.268909017211, 0.101938847218]}, {"V": [-1.75, -1.7], "P": [0.487502603516, 0.512497396484]}],
    "transport_logit_elasticities": [{"P": [0.345018612617, 0.514707287448, 0.140274099936], "direct": [-0.78597766486, -0.873526882594, -0.687780720051], "cross": [0.41402233514, 0.926473117406, 0.112219279949]}, {"P": [0.645656306226, 0.354343693774], "direct": [-0.372060878463, -0.387393783735], "cross": [0.677939121537, 0.212606216265]}],
    "transport_nested_logit": [{"P": [0.560574109573, 0.176347032294, 0.263078858132], "P_nest": [0.560574109573, 0.439425890427]}, {"P": [0.518080024916, 0.165219117164, 0.297671364261, 0.0190294936597], "P_nest": [0.68329914208, 0.31670085792]}],
    "transport_logsum_user_benefit": [{"dCS": 0.959447811658, "logsum0": -0.135843086208, "logsum1": 0.104018866707}, {"dCS": 0.410935569503, "logsum0": 0.613261687518, "logsum1": 0.654355244469}],
    "transport_trip_generation_regression": [{"coef": [258.227672282, 5.71819751176, 1.38738416969], "r2": 0.994433953978, "y_new": 2953.28888624}, {"coef": [187.347560976, 2.40904471545], "r2": 0.998189214817}],
    "transport_understeer_gradient": [{"K": 0.00141612200436, "K_deg_per_g": 0.795961956569, "behaviour": "understeer", "v_c": 43.6648071634, "v_c_kmh": 157.193305788}, {"K": -0.00577777777778, "K_deg_per_g": -3.2475247828, "behaviour": "oversteer", "v_c": 20.8012573584, "v_c_kmh": 74.8845264904}],
    "transport_steady_state_cornering": [{"r": 0.213728040196, "a_y": 4.27456080392, "R": 93.5768651677, "beta": -1.00246781832}, {"r": 0.268899272089, "a_y": 4.03348908133, "R": 55.7829698961, "beta": -0.641138841337}],
    "transport_steer_angle_for_radius": [{"delta": 1.24769153557}, {"delta": 2.0918613711}],
    "transport_yaw_natural_frequency": [{"wn": 6.70663850226, "zeta": 0.913299063995, "f_n": 1.06739466917}, {"wn": 8.65401859177, "zeta": 0.930687108001, "f_n": 1.37732983649}],
    "transport_quarter_car_frequencies": [{"f_body": 1.18543873987, "f_wheel": 11.2618541121}, {"f_body": 1.17486538962, "f_wheel": 11.9765024799}],
    "transport_quarter_car_transmissibility": [{"T_body": 2.39633431943, "T_wheel": 1.15903723756}, {"T_body": 0.125631349633, "T_wheel": 1.89424523467}],
}


# =============================================================================================== shared input builders
def _L(sym, meaning, names=(), show="m", **kw):
    return _in(sym, "length", "length", meaning, names, show, **kw)


def _V(sym, meaning, names=(), show="km/h", **kw):
    return _in(sym, "speed", "speed", meaning, names, show, **kw)


def _T(sym, meaning, names=(), show="s", **kw):
    return _in(sym, "time", "time", meaning, names, show, **kw)


def _A(sym, meaning, names=(), show="m/s^2", **kw):
    return _in(sym, "acceleration", "accel", meaning, names, show, **kw)


def _M(sym, meaning, names=(), show="kg", **kw):
    return _in(sym, "mass", "mass", meaning, names, show, **kw)


def _F(sym, meaning, names=(), show="N", **kw):
    return _in(sym, "force", "force", meaning, names, show, **kw)


def _P(sym, meaning, names=(), show="kW", **kw):
    return _in(sym, "power", "power", meaning, names, show, **kw)


def _N(sym, meaning, names=(), show="", kind="number", **kw):
    return _in(sym, kind, "number", meaning, names, show, **kw)


def _MU(sym, meaning, names=(), **kw):
    return _in(sym, "friction_coefficient", "number", meaning, names, "", **kw)


def _GR(sym="G", meaning="grade, decimal (positive uphill)", names=("grade",), show="%", kind="grade", **kw):
    kw.setdefault("required", False)
    return _in(sym, kind, "grade", meaning, names, show, **kw)


def _Q(sym, meaning, names=(), show="veh/h", **kw):
    return _in(sym, "vehicle_flow", "flow", meaning + " (veh/h in questions; veh/s in solve(), or a string such as '600 1/h')",
               names, show, **kw)


def _K(sym, meaning, names=(), show="veh/km", **kw):
    return _in(sym, "vehicle_density", "density", meaning + " (veh/km in questions; veh/m in solve(), or '25 1/km')",
               names, show, **kw)


def _VEC(sym, meaning, names=(), **kw):
    return _in(sym, "number", "number", meaning, names, "", typ="vector", **kw)


def _MAT(sym, meaning, names=(), **kw):
    return _in(sym, "number", "number", meaning, names, "", typ="matrix", **kw)


def _kmh(v):
    return _r(v * 3.6)


# =============================================================================================== braking and stopping
def _stop_sim(v0, decel, t_react=0.0):
    """Reference: integrate the vehicle's motion (constant speed for the reaction time, then constant deceleration)
    and return (reaction distance, total distance at rest, braking time)."""
    s1 = _ivp(lambda t, y: [y[1], 0.0], t_react, [0.0, v0]) if t_react > 0 else None
    x1 = float(s1.sol(t_react)[0]) if s1 is not None else 0.0
    s2 = _ivp(lambda t, y: [y[1], -decel], 3.0 * v0 / decel, [x1, v0])
    tb = _first_crossing(s2, 1, 0.0)
    return x1, float(s2.sol(tb)[0]), tb


def braking_distance(v, a):
    _pos(v=v, a=a)
    return {"d": _r(v * v / (2 * a)), "t": _r(v / a)}


def _ref_braking(v, a):
    _x1, d, t = _stop_sim(v, a)
    return {"d": d, "t": t}


proc("transport_braking_distance", "Braking distance from speed and deceleration", "braking and stopping",
     "Distance and time for a vehicle to stop from speed v at a constant deceleration a (no reaction time).",
     [_V("v", "initial speed", ("speed", "initial speed")), _A("a", "deceleration (magnitude)", ("deceleration",))],
     [_o("d", "length", "braking distance (m)"), _o("t", "time", "braking time (s)")],
     braking_distance, _ref_braking, lambda v, a: {"d": _r(v * v / a), "t": _r(v / a)},
     lambda rng: {"v": rng.uniform(5, 45), "a": rng.uniform(1, 9)},
     [{"v": 100, "a": 3.4}, {"v": 50, "a": 7}], ["braking distance", "braking distance from speed and deceleration"],
     conditions=["v > 0", "a > 0"], invalid=[{"v": 100, "a": -3}],
     method="d = v^2 / (2a), t = v / a", latex=r"d = \frac{v^2}{2a},\quad t = \frac{v}{a}")


def _decel_fg(f, Gr):
    return G * (f + Gr)


def ssd(v, t_r, f=None, a=None, G=0.0):
    _pos(v=v)
    _nonneg(t_r=t_r)
    _finite(G=G)
    _need((f is None) != (a is None), "give either the friction coefficient f or the deceleration a (not both)")
    if f is not None:
        _pos(f=f)
        dec = G_ * (f + G)
    else:
        _pos(a=a)
        dec = a + 9.81 * G
    _need(dec > 0, "the grade is too steep: the vehicle cannot stop (f + G must be positive)")
    dr, db = v * t_r, v * v / (2 * dec)
    return {"SSD": _r(dr + db), "d_r": _r(dr), "d_b": _r(db)}


def _ref_ssd(v, t_r, f=None, a=None, G=0.0):
    dec = 9.81 * (f + G) if f is not None else a + 9.81 * G
    x1, d, _t = _stop_sim(v, dec, t_r)
    return {"SSD": d, "d_r": x1, "d_b": d - x1}


def _wrong_ssd(v, t_r, f=None, a=None, G=0.0):
    dec = 9.81 * (f + G) if f is not None else a + 9.81 * G
    return {"SSD": _r(v * t_r + v * v / dec), "d_r": _r(v * t_r), "d_b": _r(v * v / dec)}


def _rand_ssd(rng):
    x = {"v": rng.uniform(8, 35), "t_r": rng.uniform(0.8, 3.0), "G": rng.uniform(-0.06, 0.08)}
    if rng.random() < 0.5:
        x["f"] = rng.uniform(0.25, 0.7)
    else:
        x["a"] = rng.uniform(2.0, 5.0)
    return x


proc("transport_stopping_sight_distance", "Stopping sight distance (reaction plus braking, with grade)",
     "braking and stopping",
     "Stopping sight distance = perception-reaction distance + braking distance on a grade, with either the "
     "longitudinal friction coefficient f (braking deceleration g (f + G)) or a deceleration rate a (AASHTO: "
     "a + g G, design value 3.4 m/s^2).",
     [_V("v", "design speed", ("speed", "design speed")), _T("t_r", "perception-reaction time", ("reaction time",
                                                                                                    "perception reaction time")),
      _MU("f", "longitudinal friction coefficient", ("friction", "friction coefficient"), required=False),
      _A("a", "deceleration rate", ("deceleration",), required=False), _GR()],
     [_o("SSD", "length", "stopping sight distance (m)"), _o("d_r", "length", "perception-reaction distance (m)"),
      _o("d_b", "length", "braking distance (m)")],
     ssd, _ref_ssd, _wrong_ssd, _rand_ssd,
     [{"v": 100, "t_r": 2.5, "a": 3.4}, {"v": 80, "t_r": 2.5, "f": 0.35, "G": -3}],
     ["stopping sight distance", "stopping distance"],
     conditions=["v > 0", "t_r >= 0", "exactly one of f, a", "f + G > 0 (or a + g G > 0)"],
     invalid=[{"v": 100, "t_r": 2.5, "f": 0.3, "G": -40}],
     method="SSD = v t_r + v^2 / (2 g (f + G)), or with a deceleration rate v t_r + v^2 / (2 (a + g G))",
     latex=r"SSD = v\,t_r + \frac{v^2}{2 g (f + G)}")


def skid_speed(d, f, G=0.0):
    _pos(d=d, f=f)
    _finite(G=G)
    _need(f + G > 0, "f + G must be positive")
    v = math.sqrt(2 * G_ * (f + G) * d)
    return {"v": _r(v), "v_kmh": _kmh(v)}


def _ref_skid(d, f, G=0.0):
    """Integrate the skid backwards in time from rest at the end of the marks until the start of the marks."""
    dec = 9.81 * (f + G)
    sol = _ivp(lambda t, y: [-y[1], dec], 3.0 * math.sqrt(2 * d / dec) + 1.0, [d, 0.0])
    tau = _first_crossing(sol, 0, 0.0)
    v = float(sol.sol(tau)[1])
    return {"v": v, "v_kmh": v * 3.6}


proc("transport_skid_speed", "Speed estimate from skid-mark length", "braking and stopping",
     "Initial speed of a vehicle that skidded to rest over a length d with friction coefficient f on a grade G.",
     [_L("d", "skid-mark length", ("skid length", "skid distance", "length")), _MU("f", "friction coefficient",
                                                                                 ("friction", "friction coefficient")),
      _GR()],
     [_o("v", "speed", "initial speed (m/s)"), _o("v_kmh", "speed_kmh", "initial speed (km/h)")],
     skid_speed, _ref_skid, lambda d, f, G=0.0: (lambda v: {"v": _r(v), "v_kmh": _kmh(v)})(math.sqrt(9.81 * (f + G) * d)),
     lambda rng: {"d": rng.uniform(5, 80), "f": rng.uniform(0.3, 0.8), "G": rng.uniform(-0.05, 0.06)},
     [{"d": 30, "f": 0.6}, {"d": 45, "f": 0.5, "G": -2}], ["skid mark speed", "speed from skid marks",
                                                             "skid speed estimate"],
     conditions=["d > 0", "f > 0", "f + G > 0"], invalid=[{"d": 30, "f": -0.6}],
     method="v = sqrt(2 g (f + G) d)", latex=r"v = \sqrt{2 g (f + G)\, d}")


def slow_distance(v1, v2, f, G=0.0):
    _pos(v1=v1, f=f)
    _nonneg(v2=v2)
    _finite(G=G)
    _need(v2 < v1, "the final speed must be below the initial speed")
    dec = 9.81 * (f + G)
    _need(dec > 0, "f + G must be positive")
    return {"d": _r((v1 * v1 - v2 * v2) / (2 * dec)), "t": _r((v1 - v2) / dec)}


def _ref_slow(v1, v2, f, G=0.0):
    dec = 9.81 * (f + G)
    sol = _ivp(lambda t, y: [y[1], -dec], 2.0 * v1 / dec, [0.0, v1])
    t = _first_crossing(sol, 1, v2)
    return {"d": float(sol.sol(t)[0]), "t": t}


proc("transport_speed_reduction_distance", "Distance to slow from one speed to another (friction and grade)",
     "braking and stopping",
     "Braking distance and time to slow from v1 to v2 with friction coefficient f on a grade G (deceleration "
     "g (f + G)).",
     [_V("v1", "initial speed", ("initial speed",)), _V("v2", "final speed", ("final speed",)),
      _MU("f", "friction coefficient", ("friction", "friction coefficient")), _GR()],
     [_o("d", "length", "braking distance (m)"), _o("t", "time", "braking time (s)")],
     slow_distance, _ref_slow, lambda v1, v2, f, G=0.0: {"d": _r((v1 * v1 - v2 * v2) / (2 * 9.81 * f)),
                                                        "t": _r((v1 - v2) / (9.81 * (f + G)))},
     lambda rng: (lambda v1: {"v1": v1, "v2": rng.uniform(0, 0.9) * v1, "f": rng.uniform(0.25, 0.7),
                              "G": rng.choice([-1, 1]) * rng.uniform(0.01, 0.07)})(rng.uniform(10, 40)),
     [{"v1": 90, "v2": 40, "f": 0.4, "G": 3}, {"v1": 120, "v2": 60, "f": 0.3, "G": -4}],
     ["speed reduction braking distance", "distance to slow down", "braking distance from one speed to another"],
     conditions=["v1 > v2 >= 0", "f + G > 0"], invalid=[{"v1": 40, "v2": 90, "f": 0.4}],
     method="d = (v1^2 - v2^2) / (2 g (f + G))", latex=r"d = \frac{v_1^2 - v_2^2}{2 g (f + G)}")


def impact_speed(v, D, t_r, f, G=0.0):
    _pos(v=v, D=D, f=f)
    _nonneg(t_r=t_r)
    _finite(G=G)
    dec = 9.81 * (f + G)
    _need(dec > 0, "f + G must be positive")
    dr = v * t_r
    stop = dr + v * v / (2 * dec)
    if D <= dr:
        vi = v
    elif D >= stop:
        vi = 0.0
    else:
        vi = math.sqrt(max(0.0, v * v - 2 * dec * (D - dr)))
    return {"v_impact": _r(vi), "v_impact_kmh": _kmh(vi), "d_stop": _r(stop)}


def _ref_impact(v, D, t_r, f, G=0.0):
    dec = 9.81 * (f + G)
    x1, dstop, tb = _stop_sim(v, dec, t_r)
    if D <= x1:
        vi = v
    elif D >= dstop:
        vi = 0.0
    else:
        s2 = _ivp(lambda t, y: [y[1], -dec], tb, [x1, v])
        t = _first_crossing(s2, 0, D)
        vi = float(s2.sol(t)[1])
    return {"v_impact": vi, "v_impact_kmh": vi * 3.6, "d_stop": dstop}


def _wrong_impact(v, D, t_r, f, G=0.0):
    dec = 9.81 * (f + G)
    vi = math.sqrt(max(0.0, v * v - 2 * dec * D))
    return {"v_impact": _r(vi), "v_impact_kmh": _kmh(vi), "d_stop": _r(v * v / (2 * dec))}


proc("transport_impact_speed", "Impact speed with an obstacle after reaction and braking", "braking and stopping",
     "A driver at speed v sees an obstacle at distance D, reacts after t_r and brakes with friction f on grade G: "
     "the speed at the obstacle (0 if the vehicle stops in time) and the full stopping distance.",
     [_V("v", "initial speed", ("speed", "initial speed")), _L("D", "distance to the obstacle", ("distance",
                                                                                               "obstacle distance")),
      _T("t_r", "perception-reaction time", ("reaction time",)), _MU("f", "friction coefficient", ("friction",
                                                                                                "friction coefficient")),
      _GR()],
     [_o("v_impact", "speed", "speed at the obstacle (m/s), 0 if the vehicle stops before it"),
      _o("v_impact_kmh", "speed_kmh", "speed at the obstacle (km/h)"),
      _o("d_stop", "length", "stopping distance from the moment the obstacle is seen (m)")],
     impact_speed, _ref_impact, _wrong_impact,
     lambda rng: (lambda v, tr, f: {"v": v, "t_r": tr, "f": f,
                                    "D": rng.uniform(1.05, 0.95 * (1 + v / (2 * 9.81 * f * tr))) * v * tr})(
         rng.uniform(12, 35), rng.uniform(0.8, 2.0), rng.uniform(0.35, 0.75)),
     [{"v": 90, "D": 60, "t_r": 1.0, "f": 0.6}, {"v": 60, "D": 30, "t_r": 1.5, "f": 0.7}],
     ["impact speed", "collision speed after braking"],
     conditions=["v > 0", "D > 0", "t_r >= 0", "f + G > 0"], invalid=[{"v": 90, "D": -60, "t_r": 1.0, "f": 0.6}],
     method="v_i = sqrt(v^2 - 2 g (f + G) (D - v t_r)) when the obstacle lies within the braking phase",
     latex=r"v_i = \sqrt{v^2 - 2 g (f + G)(D - v t_r)}")


def accel_distance(v1, v2, a):
    _nonneg(v1=v1)
    _pos(v2=v2, a=a)
    _need(v2 > v1, "the final speed must exceed the initial speed")
    return {"d": _r((v2 * v2 - v1 * v1) / (2 * a)), "t": _r((v2 - v1) / a)}


def _ref_accel(v1, v2, a):
    sol = _ivp(lambda t, y: [y[1], a], 2.0 * (v2 - v1) / a, [0.0, v1])
    t = _first_crossing(sol, 1, v2)
    return {"d": float(sol.sol(t)[0]), "t": t}


proc("transport_acceleration_distance", "Distance and time to accelerate (acceleration lane length)",
     "vehicle performance",
     "Distance and time for a vehicle to accelerate from v1 to v2 at a constant acceleration a (for example the "
     "length of an acceleration lane).",
     [_V("v1", "initial speed", ("initial speed",)), _V("v2", "final speed", ("final speed",)),
      _A("a", "acceleration", ("acceleration",))],
     [_o("d", "length", "distance travelled while accelerating (m)"), _o("t", "time", "time taken (s)")],
     accel_distance, _ref_accel, lambda v1, v2, a: {"d": _r((v2 - v1) ** 2 / (2 * a)), "t": _r((v2 - v1) / a)},
     lambda rng: (lambda v1: {"v1": v1, "v2": v1 + rng.uniform(3, 25), "a": rng.uniform(0.5, 3.5)})(rng.uniform(2, 20)),
     [{"v1": 40, "v2": 100, "a": 1.2}, {"v1": 0, "v2": 50, "a": 2.5}],
     ["acceleration distance", "acceleration lane length", "distance to accelerate"],
     conditions=["v2 > v1 >= 0", "a > 0"], invalid=[{"v1": 100, "v2": 40, "a": 1.2}],
     method="d = (v2^2 - v1^2) / (2a), t = (v2 - v1) / a", latex=r"d = \frac{v_2^2 - v_1^2}{2a}")


def yellow_interval(v, t_r, a, G=0.0):
    _pos(v=v, a=a)
    _nonneg(t_r=t_r)
    _finite(G=G)
    dec = a + 9.81 * G
    _need(dec > 0, "a + g G must be positive")
    return {"Y": _r(t_r + v / (2 * dec)), "x_c": _r(v * t_r + v * v / (2 * dec))}


def _ref_yellow(v, t_r, a, G=0.0):
    """The critical stopping distance by simulation; the yellow time is the time a vehicle at that distance needs to
    reach the stop line at constant speed (simulated as well)."""
    _x1, xc, _t = _stop_sim(v, a + 9.81 * G, t_r)
    sol = _ivp(lambda t, y: [v], 2.0 * xc / v, [0.0])
    return {"Y": _first_crossing(sol, 0, xc), "x_c": xc}


proc("transport_yellow_change_interval", "Yellow change interval (kinematic, ITE)", "traffic signals",
     "Minimum yellow change interval Y = t_r + v / (2a + 2gG) so that a driver who is just too close to stop can "
     "clear at the approach speed, and the critical stopping distance from the stop line.",
     [_V("v", "approach speed", ("speed", "approach speed")), _T("t_r", "perception-reaction time", ("reaction time",)),
      _A("a", "comfortable deceleration rate", ("deceleration",)), _GR()],
     [_o("Y", "time", "yellow change interval (s)"), _o("x_c", "length", "critical stopping distance (m)")],
     yellow_interval, _ref_yellow,
     lambda v, t_r, a, G=0.0: {"Y": _r(t_r + v / (a + 2 * 9.81 * G)), "x_c": _r(v * t_r + v * v / (a + 2 * 9.81 * G))},
     lambda rng: {"v": rng.uniform(8, 25), "t_r": rng.uniform(0.8, 1.5), "a": rng.uniform(2.5, 3.5),
                  "G": rng.uniform(-0.04, 0.05)},
     [{"v": 50, "t_r": 1.0, "a": 3.05}, {"v": 60, "t_r": 1.0, "a": 3.05, "G": -2}],
     ["yellow change interval", "yellow interval", "amber time"],
     conditions=["v > 0", "a + g G > 0"], invalid=[{"v": 50, "t_r": 1.0, "a": -3.05}],
     method="Y = t_r + v / (2a + 2 g G); x_c = v t_r + v^2 / (2 (a + g G))", latex=r"Y = t_r + \frac{v}{2a + 2gG}")


def overtaking(v, a, s):
    _nonneg(v=v)
    _pos(a=a, s=s)
    t = math.sqrt(2 * s / a)
    return {"t": _r(t), "d": _r(v * t + a * t * t / 2), "v_f": _r(v + a * t)}


def _ref_overtake(v, a, s):
    sol = _ivp(lambda t, y: [y[1], a, v], 4.0 * math.sqrt(s / a) + 1.0, [0.0, v, 0.0])
    from scipy.optimize import brentq
    f = lambda t: sol.sol(t)[0] - sol.sol(t)[2] - s
    t = brentq(f, 0.0, sol.t[-1], xtol=1e-300, rtol=8.9e-16, maxiter=500)
    y = sol.sol(t)
    return {"t": t, "d": float(y[0]), "v_f": float(y[1])}


proc("transport_overtaking_manoeuvre", "Overtaking manoeuvre: time and distance with constant acceleration",
     "vehicle performance",
     "A vehicle that starts at the speed v of the vehicle it passes accelerates at a until it has gained the relative "
     "distance s (initial gap + both lengths + final gap): the time, its distance travelled and its final speed.",
     [_V("v", "speed of the overtaken vehicle (and initial speed of the passing vehicle)", ("speed",)),
      _A("a", "acceleration of the passing vehicle", ("acceleration",)),
      _L("s", "relative distance to gain", ("relative distance", "gap to gain"))],
     [_o("t", "time", "time to complete the overtaking (s)"), _o("d", "length", "distance travelled by the passing vehicle (m)"),
      _o("v_f", "speed", "final speed of the passing vehicle (m/s)")],
     overtaking, _ref_overtake, lambda v, a, s: (lambda t: {"t": _r(t), "d": _r(v * t + a * t * t), "v_f": _r(v + a * t)})(
         math.sqrt(s / a)),
     lambda rng: {"v": rng.uniform(10, 25), "a": rng.uniform(0.5, 2.5), "s": rng.uniform(20, 120)},
     [{"v": 70, "a": 1.5, "s": 60}, {"v": 50, "a": 2.0, "s": 40}],
     ["overtaking manoeuvre", "overtaking time", "passing manoeuvre"],
     conditions=["v >= 0", "a > 0", "s > 0"], invalid=[{"v": 70, "a": -1.5, "s": 60}],
     method="t = sqrt(2 s / a), d = v t + a t^2 / 2", latex=r"t = \sqrt{\frac{2s}{a}},\quad d = v t + \tfrac{1}{2} a t^2")


# ------------------------------------------------------------------- braking and coasting with aerodynamic resistance
def _resist_const(m, f_r, Gr, mu=0.0, eta=1.0):
    th = math.atan(Gr)
    return m * 9.81 * ((eta * mu + f_r) * math.cos(th) + math.sin(th))


def _decel_drag(m, F0, K, gamma, v1, v2):
    """Distance and time for dv/dt = -(F0 + K v^2) / (gamma m) from v1 down to v2 (F0 > 0, K > 0)."""
    gm = gamma * m
    S = gm / (2 * K) * math.log1p(K * (v1 * v1 - v2 * v2) / (F0 + K * v2 * v2))
    c = math.sqrt(K / F0)
    t = gm / math.sqrt(K * F0) * (math.atan(v1 * c) - math.atan(v2 * c))
    return S, t


def brake_resist(m, v1, mu, C_D, A, rho=1.2, f_r=0.01, G=0.0, eta=1.0, gamma=1.04, v2=0.0):
    _pos(m=m, v1=v1, mu=mu, C_D=C_D, A=A, rho=rho, eta=eta, gamma=gamma)
    _nonneg(f_r=f_r, v2=v2)
    _finite(G=G)
    _need(v2 < v1, "the final speed must be below the initial speed")
    _need(eta <= 1, "the braking efficiency must not exceed 1")
    F0 = _resist_const(m, f_r, G, mu, eta)
    _need(F0 > 0, "the constant retarding force must be positive (the grade is too steep)")
    S, t = _decel_drag(m, F0, rho * C_D * A / 2, gamma, v1, v2)
    return {"S": _r(S), "t": _r(t)}


def _ref_decel_drag(m, F0, K, gamma, v1, v2):
    ev = lambda t, y: y[1] - v2
    ev.terminal = True
    sol = _ivp(lambda t, y: [y[1], -(F0 + K * y[1] * abs(y[1])) / (gamma * m)], 3.0 * gamma * m * v1 / F0, [0.0, v1],
               events=ev)
    return float(sol.y_events[0][0][0]), float(sol.t_events[0][0])


def _ref_brake_resist(m, v1, mu, C_D, A, rho=1.2, f_r=0.01, G=0.0, eta=1.0, gamma=1.04, v2=0.0):
    S, t = _ref_decel_drag(m, _resist_const(m, f_r, G, mu, eta), rho * C_D * A / 2, gamma, v1, v2)
    return {"S": S, "t": t}


proc("transport_braking_with_air_resistance", "Theoretical stopping distance with aerodynamic and rolling resistance",
     "braking and stopping",
     "Stopping (or slowing) distance and time of a road vehicle braking with tyre-road friction mu and braking "
     "efficiency eta, helped by aerodynamic drag (rho C_D A v^2 / 2), rolling resistance f_r and the grade; gamma is "
     "the mass factor for rotating parts (1.04 for braking).",
     [_M("m", "vehicle mass", ("mass",)), _V("v1", "initial speed", ("initial speed", "speed")),
      _MU("mu", "tyre-road friction coefficient", ("friction", "friction coefficient")),
      _N("C_D", "drag coefficient", ("drag coefficient",), kind="drag_coefficient"),
      _in("A", "area", "area", "frontal area", ("frontal area",), "m^2"),
      _in("rho", "density", "air_density", "air density (default 1.2 kg/m^3)", ("air density",), "kg/m^3", required=False),
      _N("f_r", "rolling resistance coefficient (default 0.01)", ("rolling resistance",), required=False),
      _GR(), _N("eta", "braking efficiency (default 1)", ("braking efficiency",), kind="efficiency", required=False),
      _N("gamma", "mass factor for braking (default 1.04)", ("mass factor",), required=False),
      _V("v2", "final speed (default 0)", ("final speed",), required=False)],
     [_o("S", "length", "braking distance (m)"), _o("t", "time", "braking time (s)")],
     brake_resist, _ref_brake_resist,
     lambda m, v1, mu, C_D, A, rho=1.2, f_r=0.01, G=0.0, eta=1.0, gamma=1.04, v2=0.0: (
         lambda F0, K: {"S": _r(gamma * m / K * math.log1p(K * (v1 * v1 - v2 * v2) / (F0 + K * v2 * v2))),
                        "t": _r(_decel_drag(m, F0, K, gamma, v1, v2)[1])})(_resist_const(m, f_r, G, mu, eta),
                                                                             rho * C_D * A / 2),
     lambda rng: {"m": rng.uniform(900, 2500), "v1": rng.uniform(15, 45), "mu": rng.uniform(0.3, 0.9),
                  "C_D": rng.uniform(0.25, 0.45), "A": rng.uniform(1.8, 3.0), "f_r": rng.uniform(0.008, 0.02),
                  "G": rng.uniform(-0.05, 0.06), "eta": rng.uniform(0.8, 1.0), "gamma": rng.uniform(1.0, 1.08),
                  "v2": rng.choice([0.0, rng.uniform(0, 10)])},
     [{"m": 1500, "v1": 120, "mu": 0.7, "C_D": 0.35, "A": 2.2, "f_r": 0.012, "eta": 0.95},
      {"m": 2000, "v1": 100, "mu": 0.5, "C_D": 0.4, "A": 2.6, "G": -3, "v2": 30}],
     ["theoretical stopping distance", "braking distance including air resistance",
      "braking distance including aerodynamic resistance"],
     conditions=["m, v1, mu, C_D, A > 0", "0 < eta <= 1", "v2 < v1", "a positive constant retarding force"],
     invalid=[{"m": 1500, "v1": 120, "mu": 0.7, "C_D": 0.35, "A": 2.2, "eta": 1.5}],
     method="S = gamma m / (2K) ln((F0 + K v1^2) / (F0 + K v2^2)), K = rho C_D A / 2, F0 = W ((eta mu + f_r) cos(theta) + sin(theta))",
     latex=r"S = \frac{\gamma m}{2K}\ln\frac{F_0 + K v_1^2}{F_0 + K v_2^2}")


def coast_down(m, v1, v2, C_D, A, rho=1.2, f_r=0.012, G=0.0, gamma=1.0):
    _pos(m=m, v1=v1, C_D=C_D, A=A, rho=rho, gamma=gamma)
    _nonneg(f_r=f_r, v2=v2)
    _finite(G=G)
    _need(v2 < v1, "the final speed must be below the initial speed")
    F0 = _resist_const(m, f_r, G)
    _need(F0 > 0, "rolling resistance plus grade must retard the vehicle (F0 > 0)")
    S, t = _decel_drag(m, F0, rho * C_D * A / 2, gamma, v1, v2)
    return {"t": _r(t), "d": _r(S)}


def _ref_coast(m, v1, v2, C_D, A, rho=1.2, f_r=0.012, G=0.0, gamma=1.0):
    S, t = _ref_decel_drag(m, _resist_const(m, f_r, G), rho * C_D * A / 2, gamma, v1, v2)
    return {"t": t, "d": S}


proc("transport_coast_down", "Coast-down time and distance (rolling and aerodynamic resistance)", "vehicle performance",
     "Time and distance for a vehicle coasting in neutral to slow from v1 to v2 under rolling resistance f_r, the "
     "grade and aerodynamic drag rho C_D A v^2 / 2.",
     [_M("m", "vehicle mass", ("mass",)), _V("v1", "initial speed", ("initial speed",)),
      _V("v2", "final speed", ("final speed",)),
      _N("C_D", "drag coefficient", ("drag coefficient",), kind="drag_coefficient"),
      _in("A", "area", "area", "frontal area", ("frontal area",), "m^2"),
      _in("rho", "density", "air_density", "air density (default 1.2 kg/m^3)", ("air density",), "kg/m^3", required=False),
      _N("f_r", "rolling resistance coefficient (default 0.012)", ("rolling resistance",), required=False), _GR(),
      _N("gamma", "mass factor (default 1)", ("mass factor",), required=False)],
     [_o("t", "time", "coast-down time (s)"), _o("d", "length", "coast-down distance (m)")],
     coast_down, _ref_coast,
     lambda m, v1, v2, C_D, A, rho=1.2, f_r=0.012, G=0.0, gamma=1.0: (
         lambda F0, K: {"t": _r(gamma * m * (v1 - v2) / (F0 + K * v1 * v2)),
                        "d": _r(_decel_drag(m, F0, K, gamma, v1, v2)[0])})(_resist_const(m, f_r, G), rho * C_D * A / 2),
     lambda rng: (lambda v1: {"m": rng.uniform(900, 30000), "v1": v1, "v2": rng.uniform(0.2, 0.8) * v1,
                              "C_D": rng.uniform(0.25, 0.7), "A": rng.uniform(1.8, 9.0), "f_r": rng.uniform(0.008, 0.02),
                              "G": rng.uniform(-0.005, 0.04), "gamma": rng.uniform(1.0, 1.1)})(rng.uniform(15, 35)),
     [{"m": 1400, "v1": 100, "v2": 60, "C_D": 0.32, "A": 2.2}, {"m": 1800, "v1": 80, "v2": 20, "C_D": 0.38, "A": 2.5,
                                                               "f_r": 0.015, "G": 1}],
     ["coast down time", "coast down distance", "coasting time"],
     conditions=["v2 < v1", "F0 = W (f_r cos(theta) + sin(theta)) > 0"],
     invalid=[{"m": 1400, "v1": 60, "v2": 100, "C_D": 0.32, "A": 2.2}],
     method="t = gamma m / sqrt(K F0) (atan(v1 sqrt(K/F0)) - atan(v2 sqrt(K/F0))); d = gamma m / (2K) ln((F0 + K v1^2)/(F0 + K v2^2))",
     latex=r"t = \frac{\gamma m}{\sqrt{K F_0}}\left[\arctan\!\left(v_1\sqrt{K/F_0}\right) - \arctan\!\left(v_2\sqrt{K/F_0}\right)\right]")


# ------------------------------------------------------------------- acceleration under constant power
def _power_balance(m, P, C_D, A, rho, f_r, Gr, eta):
    K = rho * C_D * A / 2
    F0 = _resist_const(m, f_r, Gr)
    h = lambda v: eta * P - F0 * v - K * v ** 3                 # v times the net force
    hi = 1.0
    while h(hi) > 0:
        hi *= 2
    return K, F0, _bisect_root(h, 0.0, hi)


def power_accel(m, P, v1, v2, C_D, A, rho=1.2, f_r=0.015, G=0.0, eta=1.0, gamma=1.0):
    _pos(m=m, P=P, v1=v1, v2=v2, C_D=C_D, A=A, rho=rho, eta=eta, gamma=gamma)
    _nonneg(f_r=f_r)
    _finite(G=G)
    _need(eta <= 1, "the drivetrain efficiency must not exceed 1")
    _need(v1 != v2, "the two speeds must differ")
    K, F0, veq = _power_balance(m, P, C_D, A, rho, f_r, G, eta)
    lo, hi = min(v1, v2), max(v1, v2)
    _need((v1 < v2 and hi < veq * (1 - 1e-9)) or (v1 > v2 and lo > veq * (1 + 1e-9)),
          f"the vehicle cannot go from v1 to v2: its equilibrium speed under this power is {veq:.6g} m/s")
    import mpmath
    net = lambda v: eta * P / v - F0 - K * v * v
    with mpmath.workdps(25):
        t = mpmath.quad(lambda v: gamma * m / net(v), [v1, v2])
        d = mpmath.quad(lambda v: gamma * m * v / net(v), [v1, v2])
    return {"t": _r(float(t)), "d": _r(float(d))}


def _ref_power_accel(m, P, v1, v2, C_D, A, rho=1.2, f_r=0.015, G=0.0, eta=1.0, gamma=1.0):
    K = rho * C_D * A / 2
    F0 = _resist_const(m, f_r, G)
    rhs = lambda t, y: [y[1], (eta * P / y[1] - F0 - K * y[1] ** 2) / (gamma * m)]
    ev = lambda t, y: y[1] - v2
    ev.terminal = True
    sol = _ivp(rhs, 1e6, [0.0, v1], events=ev)
    return {"t": float(sol.t_events[0][0]), "d": float(sol.y_events[0][0][0])}


def _wrong_power_accel(m, P, v1, v2, C_D, A, rho=1.2, f_r=0.015, G=0.0, eta=1.0, gamma=1.0):
    out = power_accel(m, P, v1, v2, C_D, A, rho, f_r, G, eta, gamma)
    F0 = _resist_const(m, f_r, G)
    K = rho * C_D * A / 2
    vm = (v1 + v2) / 2
    tt = gamma * m * (v2 - v1) / (eta * P / vm - F0 - K * vm * vm)        # one-point estimate at the mean speed
    return dict(out, t=_r(tt))


proc("transport_acceleration_under_power", "Acceleration time and distance under constant engine power",
     "vehicle performance",
     "Time and distance for a vehicle of mass m to change speed from v1 to v2 with constant power P at the wheels "
     "(tractive force eta P / v) against rolling, aerodynamic and grade resistance. A truck slowing on an upgrade "
     "towards its crawl speed is covered too (v2 < v1); both speeds must lie on the same side of the speed at which "
     "the power balances the resistance.",
     [_M("m", "vehicle mass", ("mass",)), _P("P", "engine power", ("power",)), _V("v1", "initial speed", ("initial speed",)),
      _V("v2", "final speed", ("final speed",)), _N("C_D", "drag coefficient", ("drag coefficient",), kind="drag_coefficient"),
      _in("A", "area", "area", "frontal area", ("frontal area",), "m^2"),
      _in("rho", "density", "air_density", "air density (default 1.2 kg/m^3)", ("air density",), "kg/m^3", required=False),
      _N("f_r", "rolling resistance coefficient (default 0.015)", ("rolling resistance",), required=False), _GR(),
      _N("eta", "drivetrain efficiency (default 1)", ("efficiency", "drivetrain efficiency"), kind="efficiency",
         required=False),
      _N("gamma", "mass factor for rotating parts (default 1)", ("mass factor",), required=False)],
     [_o("t", "time", "time to change speed (s)"), _o("d", "length", "distance travelled (m)")],
     power_accel, _ref_power_accel, _wrong_power_accel,
     lambda rng: (lambda m, P, CD, A, fr, Gr: (lambda veq: (lambda v1, v2: {"m": m, "P": P, "v1": v1, "v2": v2, "C_D": CD, "A": A,
                                                                              "f_r": fr, "G": Gr})(
         *((rng.uniform(0.05, 0.4) * veq, rng.uniform(0.5, 0.9) * veq) if rng.random() < 0.7 else
           (rng.uniform(1.3, 2.0) * veq, rng.uniform(1.1, 1.25) * veq))))(
         _power_balance(m, P, CD, A, 1.2, fr, Gr, 1.0)[2]))(
         rng.uniform(1000, 30000), rng.uniform(40e3, 300e3), rng.uniform(0.25, 0.7), rng.uniform(1.8, 9.0),
         rng.uniform(0.008, 0.02), rng.uniform(-0.01, 0.06)),
     [{"m": 1500, "P": 75, "v1": 20, "v2": 100, "C_D": 0.32, "A": 2.2},
      {"m": 25000, "P": 250, "v1": 80, "v2": 70, "C_D": 0.6, "A": 8.5, "f_r": 0.008, "G": 5}],
     ["acceleration under constant power", "acceleration time under power", "time to accelerate with engine power"],
     conditions=["v1, v2 > 0 on the same side of the equilibrium speed", "eta <= 1"],
     invalid=[{"m": 1500, "P": 75, "v1": 20, "v2": 1000, "C_D": 0.32, "A": 2.2}],
     method="t = integral of gamma m dv / (eta P / v - F0 - K v^2), d = integral of gamma m v dv / (...) (numerical quadrature)",
     latex=r"t = \int_{v_1}^{v_2} \frac{\gamma m\, dv}{\eta P / v - F_0 - K v^2}")


# =============================================================================================== traction and load transfer
def _check_axles(L, l_f):
    _pos(L=L, l_f=l_f)
    _need(l_f < L, "the distance from the front axle to the centre of gravity must be less than the wheelbase")


def traction_limit(m, L, l_f, h, mu, f_r=0.0, drive="rear"):
    _pos(m=m, h=h, mu=mu)
    _nonneg(f_r=f_r)
    _check_axles(L, l_f)
    W, l_r = m * G_, L - l_f
    if drive == "rear":
        _need(mu * h < L, "mu h must be less than the wheelbase")
        F = mu * W * (l_f - f_r * h) / L / (1 - mu * h / L)
        Wr = F / mu
        Wf = W - Wr
    elif drive == "front":
        F = mu * W * (l_r + f_r * h) / L / (1 + mu * h / L)
        Wf = F / mu
        Wr = W - Wf
    else:
        F = mu * W
        Wr = (W * l_f + (F - f_r * W) * h) / L
        Wf = W - Wr
    a = (F - f_r * W) / m
    _need(a > 0, "the friction-limited tractive force does not exceed the rolling resistance")
    _need(Wf > 0 and Wr > 0, "an axle would lift off")
    return {"F_max": _r(F), "a_max": _r(a), "W_f": _r(Wf), "W_r": _r(Wr)}


def _ref_traction(m, L, l_f, h, mu, f_r=0.0, drive="rear"):
    """The equilibrium equations (vertical forces, moments about the front contact, the friction limit on the driven
    axle(s), Newton's second law) solved together as one linear system by NumPy."""
    import numpy as np
    W = m * 9.81
    lim = {"rear": [0, -mu, 1, 0], "front": [-mu, 0, 1, 0], "all": [-mu, -mu, 1, 0]}[drive]
    A = np.array([[1, 1, 0, 0], [0, L, 0, -m * h], lim, [0, 0, -1, m]], float)
    b = np.array([W, W * l_f, 0.0, -f_r * W])
    Wf, Wr, F, a = np.linalg.solve(A, b)
    return {"F_max": F, "a_max": a, "W_f": Wf, "W_r": Wr}


proc("transport_traction_limited_acceleration", "Maximum tractive force and acceleration limited by tyre friction",
     "vehicle performance",
     "The largest tractive force the road can transmit (rear-, front- or all-wheel drive), the resulting maximum "
     "acceleration and the dynamic axle loads, with weight transfer from a centre of gravity at height h and rolling "
     "resistance f_r (aerodynamic drag neglected, level road).",
     [_M("m", "vehicle mass", ("mass",)), _L("L", "wheelbase", ("wheelbase",)),
      _L("l_f", "distance from the front axle to the centre of gravity", ("cg to front axle",)),
      _L("h", "height of the centre of gravity", ("cg height", "height")),
      _MU("mu", "tyre-road friction coefficient", ("friction", "friction coefficient")),
      _N("f_r", "rolling resistance coefficient (default 0)", ("rolling resistance",), required=False),
      _in("drive", "number", "number", "driven axles: rear, front or all", ("drive",), typ="choice",
          choices=["rear", "front", "all"], required=False)],
     [_o("F_max", "force", "maximum tractive force (N)"), _o("a_max", "acceleration", "maximum acceleration (m/s^2)"),
      _o("W_f", "force", "front axle load (N)"), _o("W_r", "force", "rear axle load (N)")],
     traction_limit, _ref_traction,
     lambda m, L, l_f, h, mu, f_r=0.0, drive="rear": traction_limit(m, L, L - l_f, h, mu, f_r, drive),
     lambda rng: (lambda L: {"m": rng.uniform(800, 2500), "L": L, "l_f": rng.uniform(0.3, 0.45) * L,
                             "h": rng.uniform(0.4, 0.7), "mu": rng.uniform(0.3, 0.9), "f_r": rng.uniform(0, 0.02),
                             "drive": rng.choice(["rear", "front", "all"])})(rng.uniform(2.3, 3.2)),
     [{"m": 1500, "L": 2.7, "l_f": 1.1, "h": 0.55, "mu": 0.8, "f_r": 0.012, "drive": "rear"},
      {"m": 1200, "L": 2.5, "l_f": 1.0, "h": 0.5, "mu": 0.6, "drive": "front"}],
     ["traction limited acceleration", "maximum tractive force", "maximum tractive effort"],
     conditions=["0 < l_f < L", "mu h < L for rear-wheel drive", "both axles keep contact"],
     invalid=[{"m": 1500, "L": 2.7, "l_f": 3.1, "h": 0.55, "mu": 0.8, "drive": "rear"}],
     method="RWD: F = mu W (l_f - f_r h)/L / (1 - mu h / L); FWD: F = mu W (l_r + f_r h)/L / (1 + mu h / L); AWD: F = mu W",
     latex=r"F_{max} = \frac{\mu W (l_f - f_{rl} h)/L}{1 - \mu h / L}")


def brake_transfer(m, a, h, L, l_f):
    _pos(m=m, a=a, h=h)
    _check_axles(L, l_f)
    W = m * G_
    Wf = (W * (L - l_f) + m * a * h) / L
    Wr = W - Wf
    _need(Wr > 0, "the rear axle would lift off at this deceleration")
    return {"W_f": _r(Wf), "W_r": _r(Wr), "dW": _r(m * a * h / L), "front_share": _r(Wf / W)}


def _ref_brake_transfer(m, a, h, L, l_f):
    """Equilibrium of the decelerating vehicle (d'Alembert force m a forward at the centre of gravity) solved by NumPy,
    once with and once without the deceleration; the ideal brake split from the brake forces that lock both axles at
    once (B_f / W_f = B_r / W_r, B_f + B_r = m a)."""
    import numpy as np
    W = m * 9.81

    def axles(acc):
        return np.linalg.solve(np.array([[1.0, 1.0], [L, 0.0]]), np.array([W, W * (L - l_f) + m * acc * h]))
    Wf, Wr = axles(a)
    Wf0, _ = axles(0.0)
    Bf, Br = np.linalg.solve(np.array([[Wr, -Wf], [1.0, 1.0]]), np.array([0.0, m * a]))
    return {"W_f": Wf, "W_r": Wr, "dW": Wf - Wf0, "front_share": Bf / (m * a)}


proc("transport_braking_weight_transfer", "Longitudinal weight transfer under braking and ideal brake proportioning",
     "vehicle dynamics",
     "Dynamic front and rear axle loads of a vehicle decelerating at a, the load transferred to the front axle, and "
     "the ideal front share of the braking force (both axles at the same friction use).",
     [_M("m", "vehicle mass", ("mass",)), _A("a", "deceleration", ("deceleration",)),
      _L("h", "height of the centre of gravity", ("cg height", "height")), _L("L", "wheelbase", ("wheelbase",)),
      _L("l_f", "distance from the front axle to the centre of gravity", ("cg to front axle",))],
     [_o("W_f", "force", "front axle load (N)"), _o("W_r", "force", "rear axle load (N)"),
      _o("dW", "force", "load transferred to the front axle (N)"), _o("front_share", "number", "ideal front brake share")],
     brake_transfer, _ref_brake_transfer,
     lambda m, a, h, L, l_f: (lambda o: dict(o, W_f=_r(m * G_ * l_f / L + m * a * h / L),
                                             W_r=_r(m * G_ - m * G_ * l_f / L - m * a * h / L)))(brake_transfer(m, a, h, L, l_f)),
     lambda rng: (lambda L: {"m": rng.uniform(800, 3000), "a": rng.uniform(1, 8), "h": rng.uniform(0.4, 0.8), "L": L,
                             "l_f": rng.uniform(0.3, 0.6) * L})(rng.uniform(2.3, 3.4)),
     [{"m": 1500, "a": 7, "h": 0.55, "L": 2.7, "l_f": 1.2}, {"m": 2000, "a": 4, "h": 0.7, "L": 3.0, "l_f": 1.4}],
     ["braking weight transfer", "weight transfer under braking", "longitudinal load transfer"],
     conditions=["a > 0 (deceleration)", "0 < l_f < L", "the rear axle keeps contact"],
     invalid=[{"m": 1500, "a": 7, "h": 0.55, "L": 2.7, "l_f": 2.9}],
     method="W_f = (W l_r + m a h) / L, dW = m a h / L, ideal front share = W_f / W",
     latex=r"W_f = \frac{W l_r + m a h}{L},\quad \Delta W = \frac{m a h}{L}")


def lateral_transfer(m, a_y, h, t_w):
    _pos(m=m, a_y=a_y, h=h, t_w=t_w)
    W = m * G_
    dW = m * a_y * h / t_w
    _need(W / 2 - dW >= 0, "the inner wheels would lift off (rollover) at this lateral acceleration")
    return {"W_out": _r(W / 2 + dW), "W_in": _r(_snap(W / 2 - dW, W)), "dW": _r(dW)}


def _ref_lateral_transfer(m, a_y, h, t_w):
    import numpy as np
    W = m * 9.81
    Nin, Nout = np.linalg.solve(np.array([[1.0, 1.0], [t_w, 0.0]]), np.array([W, W * t_w / 2 - m * a_y * h]))
    return {"W_out": Nout, "W_in": Nin, "dW": Nout - W / 2}


proc("transport_lateral_load_transfer", "Lateral load transfer in a steady turn", "vehicle dynamics",
     "Outer and inner wheel loads of a rigid vehicle (no suspension roll) cornering at lateral acceleration a_y, and the "
     "load transferred to the outer wheels.",
     [_M("m", "vehicle mass", ("mass",)), _A("a_y", "lateral acceleration", ("lateral acceleration",)),
      _L("h", "height of the centre of gravity", ("cg height", "height")), _L("t_w", "track width", ("track width", "track"))],
     [_o("W_out", "force", "load on the outer wheels (N)"), _o("W_in", "force", "load on the inner wheels (N)"),
      _o("dW", "force", "load transferred to the outer wheels (N)")],
     lateral_transfer, _ref_lateral_transfer,
     lambda m, a_y, h, t_w: (lambda W, d: {"W_out": _r(W / 2 + d), "W_in": _r(W / 2 - d), "dW": _r(d)})(
         m * G_, 2 * m * a_y * h / t_w),
     lambda rng: (lambda h, t: {"m": rng.uniform(800, 3000), "h": h, "t_w": t,
                                "a_y": rng.uniform(0.5, 0.95) * 9.81 * t / (2 * h)})(rng.uniform(0.45, 1.0), rng.uniform(1.3, 1.8)),
     [{"m": 1500, "a_y": 0.6, "h": 0.55, "t_w": 1.55}, {"m": 2200, "a_y": 4, "h": 0.8, "t_w": 1.6}],
     ["lateral load transfer", "lateral weight transfer", "cornering load transfer"],
     conditions=["all inputs positive", "the inner wheels keep contact"], invalid=[{"m": 1500, "a_y": 0.6, "h": -0.55, "t_w": 1.55}],
     method="dW = m a_y h / t, W_out = W/2 + dW, W_in = W/2 - dW", latex=r"\Delta W = \frac{m a_y h}{t}")


def rollover(t_w, h, R, e=0.0):
    _pos(t_w=t_w, h=h, R=R)
    _finite(e=e)
    _need(-1 < e < 1, "the superelevation rate must lie between -1 and 1")
    ssf = t_w / (2 * h)
    _need(ssf * e < 1, "this vehicle cannot roll over on this bank (SSF e >= 1)")
    _need(ssf + e > 0, "the bank is so steep the vehicle tips inward at rest")
    v = math.sqrt(G_ * R * (ssf + e) / (1 - ssf * e))
    return {"SSF": _r(ssf), "v_roll": _r(v), "v_roll_kmh": _kmh(v)}


def _ref_rollover(t_w, h, R, e=0.0):
    """Wheel loads of the rigid vehicle on the banked curve from the force and moment equilibrium (NumPy), the speed
    at which the inner wheel load vanishes found by brentq; SSF as the lateral acceleration (in g) that unloads the
    inner wheels on a level road."""
    import numpy as np
    th = math.atan(e)

    def n_inner(v, theta, R_=R):
        A = np.array([[math.sin(theta), math.sin(theta), math.cos(theta)],
                      [math.cos(theta), math.cos(theta), -math.sin(theta)],
                      [t_w / 2, -t_w / 2, h]])
        return float(np.linalg.solve(A, np.array([v * v / R_, 9.81, 0.0]))[0])
    hi = 1.0
    while n_inner(hi, th) > 0:
        hi *= 2
    v = _brentq(lambda s: n_inner(s, th), 0.0, hi)
    ay = _brentq(lambda a: n_inner(math.sqrt(a), 0.0, 1.0), 0.0, 1e4)
    return {"SSF": ay / 9.81, "v_roll": v, "v_roll_kmh": v * 3.6}


proc("transport_rollover_threshold", "Rollover threshold: static stability factor and rollover speed on a curve",
     "vehicle dynamics",
     "Static stability factor SSF = t / (2h) of a rigid vehicle and the speed at which it would tip over on a curve of "
     "radius R with superelevation e (enough friction assumed for the tyres not to slide).",
     [_L("t_w", "track width", ("track width", "track")), _L("h", "height of the centre of gravity", ("cg height", "height")),
      _L("R", "curve radius", ("radius",)), _GR("e", "superelevation rate, decimal", ("superelevation",),
                                               kind="superelevation_rate")],
     [_o("SSF", "number", "static stability factor t / (2h) (rollover threshold in g)"),
      _o("v_roll", "speed", "rollover speed on the curve (m/s)"), _o("v_roll_kmh", "speed_kmh", "rollover speed (km/h)")],
     rollover, _ref_rollover,
     lambda t_w, h, R, e=0.0: (lambda s: {"SSF": _r(s), "v_roll": _r(math.sqrt(G_ * R * (s + e))),
                                          "v_roll_kmh": _kmh(math.sqrt(G_ * R * (s + e)))})(t_w / h),
     lambda rng: {"t_w": rng.uniform(1.3, 2.0), "h": rng.uniform(0.5, 2.0), "R": rng.uniform(30, 500),
                  "e": rng.uniform(0.0, 0.1)},
     [{"t_w": 1.55, "h": 0.6, "R": 100}, {"t_w": 1.8, "h": 1.9, "R": 200, "e": 6}],
     ["rollover threshold", "rollover speed", "static stability factor"],
     conditions=["SSF e < 1", "tyre friction sufficient (sliding is not checked)"], invalid=[{"t_w": -1.55, "h": 0.6, "R": 100}],
     method="SSF = t / (2h); v^2 / (g R) = (SSF + e) / (1 - SSF e)",
     latex=r"SSF = \frac{t}{2h},\quad \frac{v^2}{gR} = \frac{SSF + e}{1 - SSF\,e}")


# =============================================================================================== highway geometry: curves
def _e_check(e):
    _finite(e=e)
    _need(-1 < e < 1, "the superelevation rate must lie between -1 and 1 (a decimal or a percentage)")


def curve_speed(R, e, f):
    _pos(R=R, f=f)
    _e_check(e)
    _need(e * f < 1, "e f >= 1: friction and superelevation never limit the speed")
    _need(e + f > 0, "e + f must be positive")
    v = math.sqrt(G_ * R * (e + f) / (1 - e * f))
    return {"v": _r(v), "v_kmh": _kmh(v)}


def _curve_forces(v, R, e):
    """Reference: normal force N and side friction F (per unit mass) on the banked curve from the horizontal and
    vertical equilibrium, solved by NumPy."""
    import numpy as np
    th = math.atan(e)
    A = np.array([[math.sin(th), math.cos(th)], [math.cos(th), -math.sin(th)]])
    N, F = np.linalg.solve(A, np.array([v * v / R, 9.81]))
    return float(N), float(F)


def _friction_used(v, R, e):
    N, F = _curve_forces(v, R, e)
    return F / N


def _ref_curve_speed(R, e, f):
    hi = 1.0
    while _friction_used(hi, R, e) < f:
        hi *= 2
    v = _brentq(lambda s: _friction_used(s, R, e) - f, 0.0, hi)
    return {"v": v, "v_kmh": v * 3.6}


_CURVE_IN = {"R": lambda: _L("R", "curve radius", ("radius", "curve radius")),
             "e": lambda: _GR("e", "superelevation rate, decimal", ("superelevation", "superelevation rate"),
                              kind="superelevation_rate", required=True),
             "f": lambda: _MU("f", "side friction factor", ("side friction", "side friction factor", "friction")),
             "v": lambda: _V("v", "design speed", ("speed", "design speed"))}

proc("transport_curve_safe_speed", "Maximum safe speed on a superelevated horizontal curve", "highway geometry",
     "The highest speed at which a vehicle (point mass) can hold a curve of radius R with superelevation e and side "
     "friction factor f.",
     [_CURVE_IN["R"](), _CURVE_IN["e"](), _CURVE_IN["f"]()],
     [_o("v", "speed", "maximum safe speed (m/s)"), _o("v_kmh", "speed_kmh", "maximum safe speed (km/h)")],
     curve_speed, _ref_curve_speed,
     lambda R, e, f: (lambda v: {"v": _r(v), "v_kmh": _kmh(v)})(math.sqrt(G_ * R * (e + f))),
     lambda rng: {"R": rng.uniform(30, 1500), "e": rng.uniform(0.0, 0.12), "f": rng.uniform(0.08, 0.4)},
     [{"R": 300, "e": 6, "f": 0.12}, {"R": 150, "e": 8, "f": 0.15}],
     ["maximum safe speed on a curve", "curve safe speed", "safe speed on a horizontal curve"],
     conditions=["R > 0", "f > 0", "e f < 1"], invalid=[{"R": -300, "e": 6, "f": 0.12}],
     method="v = sqrt(g R (e + f) / (1 - e f)) (point-mass equilibrium)", latex=r"v = \sqrt{\frac{g R (e + f)}{1 - e f}}")


def curve_radius(v, e, f):
    _pos(v=v, f=f)
    _e_check(e)
    _need(e * f < 1, "e f >= 1: friction and superelevation never limit the speed")
    _need(e + f > 0, "e + f must be positive")
    return {"R_min": _r(v * v * (1 - e * f) / (G_ * (e + f)))}


def _ref_curve_radius(v, e, f):
    lo = 1e-3
    while _friction_used(v, lo, e) < f:
        lo /= 2
    hi = 1.0
    while _friction_used(v, hi, e) > f:
        hi *= 2
    return {"R_min": _brentq(lambda R: _friction_used(v, R, e) - f, lo, hi)}


proc("transport_minimum_curve_radius", "Minimum radius of a horizontal curve", "highway geometry",
     "The smallest curve radius for a design speed v, maximum superelevation e and side friction factor f.",
     [_CURVE_IN["v"](), _CURVE_IN["e"](), _CURVE_IN["f"]()],
     [_o("R_min", "length", "minimum curve radius (m)")],
     curve_radius, _ref_curve_radius, lambda v, e, f: {"R_min": _r(v * v / (G_ * (e + f)))},
     lambda rng: {"v": rng.uniform(8, 35), "e": rng.uniform(0.0, 0.12), "f": rng.uniform(0.08, 0.4)},
     [{"v": 100, "e": 8, "f": 0.12}, {"v": 60, "e": 6, "f": 0.17}],
     ["minimum curve radius", "minimum radius of a horizontal curve", "minimum radius"],
     conditions=["v > 0", "f > 0", "e f < 1"], invalid=[{"v": 100, "e": 8, "f": -0.12}],
     method="R = v^2 (1 - e f) / (g (e + f))", latex=r"R_{min} = \frac{v^2 (1 - e f)}{g (e + f)}")


def side_friction(v, R, e):
    _pos(v=v, R=R)
    _e_check(e)
    a = v * v / (G_ * R)
    return {"f": _r(_snap((a - e) / (1 + a * e), 1.0))}


proc("transport_side_friction_demand", "Side friction demanded on a superelevated curve", "highway geometry",
     "The side friction factor a vehicle needs at speed v on a curve of radius R with superelevation e (negative: the "
     "superelevation alone exceeds the demand and friction acts up the slope).",
     [_CURVE_IN["v"](), _CURVE_IN["R"](), _CURVE_IN["e"]()],
     [_o("f", "friction_coefficient", "side friction factor demanded")],
     side_friction, lambda v, R, e: {"f": _friction_used(v, R, e)},
     lambda v, R, e: {"f": _r(v * v / (G_ * R) - e)},
     lambda rng: {"v": rng.uniform(8, 35), "R": rng.uniform(50, 1500), "e": rng.uniform(0.0, 0.12)},
     [{"v": 90, "R": 250, "e": 6}, {"v": 50, "R": 80, "e": 4}],
     ["side friction demand", "side friction factor required", "required side friction"],
     conditions=["v > 0", "R > 0"], invalid=[{"v": 90, "R": 0, "e": 6}],
     method="f = (v^2/(gR) - e) / (1 + e v^2/(gR))", latex=r"f = \frac{v^2/(gR) - e}{1 + e\,v^2/(gR)}")


def superelevation(v, R, f):
    _pos(v=v, R=R)
    _finite(f=f)
    _need(f >= 0, "the side friction factor must not be negative")
    a = v * v / (G_ * R)
    e = (a - f) / (1 + a * f)
    _need(-1 < e < 1, "the required superelevation is outside -1 .. 1")
    return {"e": _r(_snap(e, 1.0)), "e_percent": _r(_snap(100 * e, 100.0))}


def _ref_superelevation(v, R, f):
    e = _brentq(lambda x: _friction_used(v, R, x) - f, -0.999, 0.999)
    return {"e": e, "e_percent": 100 * e}


proc("transport_superelevation_required", "Superelevation required on a horizontal curve", "highway geometry",
     "The superelevation rate needed at speed v on a curve of radius R when the side friction factor f is used.",
     [_CURVE_IN["v"](), _CURVE_IN["R"](), _MU("f", "side friction factor", ("side friction", "side friction factor", "friction"))],
     [_o("e", "superelevation_rate", "superelevation rate (decimal)"), _o("e_percent", "number", "superelevation rate (%)")],
     superelevation, _ref_superelevation,
     lambda v, R, f: (lambda e: {"e": _r(e), "e_percent": _r(100 * e)})(v * v / (G_ * R) - f),
     lambda rng: (lambda v, R: {"v": v, "R": R, "f": rng.uniform(0.0, 0.9) * v * v / (9.81 * R)})(rng.uniform(10, 35),
                                                                                                 rng.uniform(100, 1200)),
     [{"v": 100, "R": 450, "f": 0.12}, {"v": 70, "R": 200, "f": 0.14}],
     ["superelevation required", "required superelevation", "superelevation rate needed"],
     conditions=["v > 0", "R > 0", "f >= 0"], invalid=[{"v": 100, "R": -450, "f": 0.12}],
     method="e = (v^2/(gR) - f) / (1 + f v^2/(gR))", latex=r"e = \frac{v^2/(gR) - f}{1 + f\,v^2/(gR)}")


# =============================================================================================== crest vertical curves
def _crest_k(h1, h2):
    return (math.sqrt(h1) + math.sqrt(h2)) ** 2


def crest_length(g1, g2, S, h1=1.08, h2=0.60):
    _finite(g1=g1, g2=g2)
    _pos(S=S, h1=h1, h2=h2)
    A = g1 - g2
    _need(A > 0, "a crest curve needs g1 > g2")
    k = _crest_k(h1, h2)
    L = A * S * S / (2 * k)
    if L >= S:
        return {"L": _r(L), "case": "S < L"}
    L = 2 * S - 2 * k / A
    if L <= 0:
        return {"L": 0.0, "case": "no curve needed"}
    return {"L": _r(L), "case": "S > L"}


def _crest_clear(A, L, S, h1, h2):
    """Reference geometry: the least clearance between the sight line (eye h1, object h2, S apart horizontally) and the
    road profile over every observer position (grades A and 0 joined by a parabola of length L; the clearance does
    not depend on the individual grades). Negative: the line of sight is blocked somewhere."""
    def y(x):
        if x <= 0:
            return A * x
        if x >= L:
            return A * L / 2
        return A * x - A * x * x / (2 * L)

    def least(x0):
        x1 = x0 + S
        ya, yb = y(x0) + h1, y(x1) + h2
        ml = (yb - ya) / S
        cand = [x0, x1] + [b for b in (0.0, L) if x0 < b < x1]
        if L > 0:
            xs = L * (1 - ml / A)
            if max(x0, 0.0) < xs < min(x1, L):
                cand.append(xs)
        return min(ya + ml * (x - x0) - y(x) for x in cand)
    from scipy.optimize import minimize_scalar
    a, b = -S, max(L, 0.0)
    n = 80
    grid = [a + (b - a) * i / n for i in range(n + 1)]
    vals = [least(x) for x in grid]
    i = min(range(len(vals)), key=vals.__getitem__)
    lo, hi = grid[max(0, i - 1)], grid[min(n, i + 1)]
    best = vals[i]
    if hi > lo:
        r = minimize_scalar(least, bounds=(lo, hi), method="bounded", options={"xatol": 1e-13 * max(1.0, S + L)})
        best = min(best, float(r.fun))
    return best


def _ref_crest_length(g1, g2, S, h1=1.08, h2=0.60):
    A = g1 - g2
    if _crest_clear(A, 0.0, S, h1, h2) >= 0:
        return {"L": 0.0, "case": "no curve needed"}
    hi = max(S, 1.0)
    while _crest_clear(A, hi, S, h1, h2) < 0:
        hi *= 2
    L = _brentq(lambda x: _crest_clear(A, x, S, h1, h2), 0.0, hi)
    return {"L": L, "case": "S < L" if S <= L else "S > L"}


_CREST_IN = [_GR("g1", "grade before the curve, decimal (positive uphill)", ("g1", "initial grade"), required=True),
             _GR("g2", "grade after the curve, decimal", ("g2", "final grade"), required=True)]
_EYE = [_L("h1", "driver eye height (default 1.08 m)", ("eye height",), required=False),
        _L("h2", "object height (default 0.60 m)", ("object height",), required=False)]

proc("transport_crest_curve_length", "Minimum length of a crest vertical curve for a sight distance",
     "highway geometry",
     "The shortest crest vertical curve between grades g1 and g2 that provides the sight distance S (AASHTO: eye "
     "height 1.08 m, object height 0.60 m unless stated), choosing the S < L or S > L form.",
     _CREST_IN + [_L("S", "required sight distance", ("sight distance", "stopping sight distance"))] + _EYE,
     [_o("L", "length", "minimum curve length (m)"), _o("case", "text", "S < L, S > L or no curve needed")],
     crest_length, _ref_crest_length,
     lambda g1, g2, S, h1=1.08, h2=0.60: (lambda A, k: {"L": _r(A * S * S / (2 * k)), "case": "S < L"})(g1 - g2, 2 * _crest_k(h1, h2)),
     lambda rng: {"g1": rng.uniform(0.0, 0.06), "g2": rng.uniform(-0.06, -0.005), "S": rng.uniform(60, 400),
                  "h1": rng.choice([1.08, rng.uniform(1.0, 2.4)]), "h2": rng.choice([0.6, rng.uniform(0.15, 1.3)])},
     [{"g1": 3, "g2": -2, "S": 185}, {"g1": 1, "g2": -1.5, "S": 200}],
     ["crest vertical curve length", "minimum length of a crest vertical curve", "crest curve length"],
     conditions=["g1 > g2", "S > 0"], invalid=[{"g1": -2, "g2": 3, "S": 185}],
     method="S < L: L = A S^2 / (200 (sqrt(h1) + sqrt(h2))^2); S > L: L = 2S - 200 (sqrt(h1) + sqrt(h2))^2 / A (A in %)",
     latex=r"L = \frac{A S^2}{200\left(\sqrt{h_1} + \sqrt{h_2}\right)^2}\ \ (S<L),\qquad L = 2S - \frac{200\left(\sqrt{h_1}+\sqrt{h_2}\right)^2}{A}\ \ (S>L)")


def crest_sight(g1, g2, L, h1=1.08, h2=0.60):
    _finite(g1=g1, g2=g2)
    _nonneg(L=L)
    _pos(h1=h1, h2=h2)
    A = g1 - g2
    _need(A > 0, "a crest curve needs g1 > g2")
    k = _crest_k(h1, h2)
    S = math.sqrt(2 * k * L / A)
    if L > 0 and S <= L:
        return {"S": _r(S), "case": "S < L"}
    return {"S": _r(L / 2 + k / A), "case": "S > L"}


def _ref_crest_sight(g1, g2, L, h1=1.08, h2=0.60):
    A = g1 - g2
    hi = 10.0
    while _crest_clear(A, L, hi, h1, h2) >= 0:
        hi *= 2
    S = _brentq(lambda s: _crest_clear(A, L, s, h1, h2), 1e-9, hi)
    return {"S": S, "case": "S < L" if S <= L else "S > L"}


proc("transport_crest_curve_sight_distance", "Sight distance available over a crest vertical curve",
     "highway geometry",
     "The sight distance a crest vertical curve of length L between grades g1 and g2 provides (eye height 1.08 m, "
     "object height 0.60 m unless stated).",
     _CREST_IN + [_L("L", "curve length", ("curve length", "length"))] + _EYE,
     [_o("S", "length", "available sight distance (m)"), _o("case", "text", "S < L or S > L")],
     crest_sight, _ref_crest_sight,
     lambda g1, g2, L, h1=1.08, h2=0.60: {"S": _r(math.sqrt(2 * _crest_k(h1, h2) * L / (2 * (g1 - g2)))), "case": "S < L"},
     lambda rng: {"g1": rng.uniform(0.0, 0.06), "g2": rng.uniform(-0.06, -0.005), "L": rng.uniform(20, 600),
                  "h1": rng.choice([1.08, rng.uniform(1.0, 2.4)]), "h2": rng.choice([0.6, rng.uniform(0.15, 1.3)])},
     [{"g1": 3, "g2": -2, "L": 300}, {"g1": 2, "g2": -1, "L": 60}],
     ["crest vertical curve sight distance", "sight distance on a crest curve", "crest curve sight distance"],
     conditions=["g1 > g2", "L >= 0"], invalid=[{"g1": -2, "g2": 3, "L": 300}],
     method="S = sqrt(200 (sqrt(h1) + sqrt(h2))^2 L / A) if that is <= L, else S = L/2 + 100 (sqrt(h1) + sqrt(h2))^2 / A",
     latex=r"S = \sqrt{\frac{200\left(\sqrt{h_1}+\sqrt{h_2}\right)^2 L}{A}}")


# =============================================================================================== horizontal sight offset
def hso(R, S):
    _pos(R=R, S=S)
    _need(S < math.pi * R, "the sight distance must be shorter than half the circumference")
    return {"M": _r(2 * R * math.sin(S / (4 * R)) ** 2)}


def _arc_path(R, S):
    """Reference: the path of a vehicle driven along the curve (heading turning at 1/R per metre), integrated."""
    return _ivp(lambda s, y: [math.cos(y[2]), math.sin(y[2]), 1.0 / R], S, [0.0, 0.0, 0.0], atol=1e-13)


def _ref_offset(R, S):
    """The largest distance between the chord (the sight line) and the driven path, maximised numerically."""
    sol = _arc_path(R, S)
    x2, y2 = sol.sol(S)[0], sol.sol(S)[1]
    n = math.hypot(x2, y2)
    dist = lambda s: abs(sol.sol(s)[0] * y2 - sol.sol(s)[1] * x2) / n
    from scipy.optimize import minimize_scalar
    r = minimize_scalar(lambda s: -dist(s), bounds=(0.0, S), method="bounded", options={"xatol": 1e-12 * S})
    return max(-float(r.fun), dist(S / 2))


proc("transport_horizontal_sight_offset", "Horizontal sightline offset on a circular curve", "highway geometry",
     "The clearance M needed from the centre of the inside lane to an obstruction (middle ordinate) so that a driver "
     "on a curve of radius R sees the sight distance S, both driver and object on the curve.",
     [_L("R", "radius of the inside lane centreline", ("radius",)), _L("S", "sight distance", ("sight distance",))],
     [_o("M", "length", "horizontal sightline offset (m)")],
     hso, lambda R, S: {"M": _ref_offset(R, S)}, lambda R, S: {"M": _r(R * (1 - math.cos(S / R)))},
     lambda rng: (lambda R: {"R": R, "S": rng.uniform(0.05, 1.5) * R})(rng.uniform(50, 1500)),
     [{"R": 300, "S": 160}, {"R": 150, "S": 110}],
     ["horizontal sightline offset", "horizontal sight offset", "middle ordinate for sight distance"],
     conditions=["R > 0", "0 < S < pi R"], invalid=[{"R": 100, "S": 400}],
     method="M = R (1 - cos(S / (2R)))", latex=r"M = R\left(1 - \cos\frac{S}{2R}\right)")


def hso_sight(R, M):
    _pos(R=R, M=M)
    _need(M < R, "the offset must be less than the radius")
    return {"S": _r(2 * R * math.acos(1 - M / R))}


def _ref_hso_sight(R, M):
    S = _brentq(lambda s: _ref_offset(R, s) - M, 1e-9 * R, (math.pi - 1e-9) * R)
    return {"S": S}


proc("transport_sight_distance_from_offset", "Sight distance available on a curve with a roadside obstruction",
     "highway geometry",
     "The sight distance available on a curve of radius R (inside lane centreline) when an obstruction stands at the "
     "offset M from that centreline.",
     [_L("R", "radius of the inside lane centreline", ("radius",)), _L("M", "offset to the obstruction", ("offset",
                                                                                                      "middle ordinate"))],
     [_o("S", "length", "available sight distance (m)")],
     hso_sight, _ref_hso_sight, lambda R, M: {"S": _r(R * math.acos(1 - M / R))},
     lambda rng: (lambda R: {"R": R, "M": rng.uniform(0.01, 0.6) * R})(rng.uniform(50, 1500)),
     [{"R": 300, "M": 9}, {"R": 200, "M": 6.5}],
     ["sight distance from offset", "sight distance with an obstruction on a curve", "available sight distance on a curve"],
     conditions=["0 < M < R"], invalid=[{"R": 100, "M": 150}],
     method="S = 2R acos(1 - M / R)", latex=r"S = 2R\,\arccos\left(1 - \frac{M}{R}\right)")


# =============================================================================================== clothoid transition curve
def clothoid(R, L_s, s=None):
    _pos(R=R, L_s=L_s)
    s = L_s if s is None else s
    _pos(s=s)
    _need(s <= L_s * (1 + 1e-12), "the station must lie on the spiral (s <= L_s)")
    A2 = R * L_s
    th = s * s / (2 * A2)
    _need(th <= math.pi / 2, "the tangent angle exceeds 90 degrees (spiral too long for this radius)")
    x = y = 0.0
    for n in range(60):
        tx = (-1) ** n * th ** (2 * n) / ((4 * n + 1) * math.factorial(2 * n))
        ty = (-1) ** n * th ** (2 * n + 1) / ((4 * n + 3) * math.factorial(2 * n + 1))
        x += tx
        y += ty
        if abs(tx) < 1e-18 and abs(ty) < 1e-18:
            break
    return {"x": _r(s * x), "y": _r(s * y), "theta": _r(math.degrees(th))}


def _ref_clothoid(R, L_s, s=None):
    """Fresnel integrals (scipy.special.fresnel) for the coordinates; the tangent angle by integrating the linearly
    growing curvature s / (R L_s) along the spiral (quadrature)."""
    from scipy.special import fresnel
    s = L_s if s is None else s
    A = math.sqrt(R * L_s)
    k = A * math.sqrt(math.pi)
    S_, C_ = fresnel(s / k)
    th = _quad(lambda u: u / (R * L_s), 0.0, s)
    return {"x": float(k * C_), "y": float(k * S_), "theta": math.degrees(th)}


proc("transport_clothoid_coordinates", "Clothoid (spiral) transition curve coordinates", "highway geometry",
     "Coordinates along the tangent (x) and offset from it (y), and the tangent deflection angle, at distance s along "
     "a clothoid transition of length L_s that ends at a circular curve of radius R (A^2 = R L_s).",
     [_L("R", "radius of the circular curve", ("radius",)), _L("L_s", "spiral length", ("spiral length",
                                                                                     "transition length")),
      _L("s", "distance along the spiral (default L_s)", ("station", "distance along the spiral"), required=False)],
     [_o("x", "length", "distance along the initial tangent (m)"), _o("y", "length", "offset from the tangent (m)"),
      _o("theta", "angle_deg", "tangent deflection angle (degrees)")],
     clothoid, _ref_clothoid,
     lambda R, L_s, s=None: (lambda o: dict(o, y=_r(o["y"] * 2)))(clothoid(R, L_s, s)),
     lambda rng: (lambda R, Ls: {"R": R, "L_s": Ls, "s": rng.uniform(0.05, 1.0) * Ls})(
         rng.uniform(50, 2000), rng.uniform(20, 200)) if True else None,
     [{"R": 300, "L_s": 80}, {"R": 500, "L_s": 120, "s": 60}],
     ["clothoid coordinates", "spiral transition curve coordinates", "transition curve coordinates"],
     conditions=["0 < s <= L_s", "tangent angle <= 90 degrees"], invalid=[{"R": 300, "L_s": 80, "s": 120}],
     method="x = s (1 - theta^2/10 + theta^4/216 - ...), y = s (theta/3 - theta^3/42 + ...), theta = s^2 / (2 R L_s)",
     latex=r"x = \int_0^s \cos\frac{u^2}{2A^2}\,du,\quad y = \int_0^s \sin\frac{u^2}{2A^2}\,du,\quad \theta = \frac{s^2}{2RL_s}")


# =============================================================================================== traffic flow models
def _vph(q):
    return _r(q * 3600)


def _vpkm(k):
    return _r(k * 1000)


def greenshields_cap(vf, kj):
    _pos(vf=vf, kj=kj)
    return {"q_max": _vph(vf * kj / 4), "k_m": _vpkm(kj / 2), "v_m": _kmh(vf / 2)}


def _ref_greenshields_cap(vf, kj):
    """The flow-density curve q(k) = k vf (1 - k/kj) as a NumPy polynomial, maximised where its derivative vanishes."""
    from numpy.polynomial import Polynomial
    q = Polynomial([0.0, vf, -vf / kj])
    km = float(q.deriv().roots()[0])
    qm = float(q(km))
    return {"q_max": qm * 3600, "k_m": km * 1000, "v_m": vf * (1 - km / kj) * 3.6}


proc("transport_greenshields_capacity", "Greenshields model: capacity, optimum speed and density", "traffic flow",
     "Capacity (maximum flow) of a traffic stream following Greenshields' linear speed-density model "
     "v = vf (1 - k/kj), with the density and speed at which it occurs.",
     [_V("vf", "free-flow speed", ("free flow speed", "free-flow speed")),
      _K("kj", "jam density", ("jam density",))],
     [_o("q_max", "traffic_flow", "capacity (veh/h)"), _o("k_m", "traffic_density", "density at capacity (veh/km)"),
      _o("v_m", "speed_kmh", "speed at capacity (km/h)")],
     greenshields_cap, _ref_greenshields_cap,
     lambda vf, kj: {"q_max": _vph(vf * kj / 2), "k_m": _vpkm(kj / 2), "v_m": _kmh(vf / 2)},
     lambda rng: {"vf": rng.uniform(40, 130) / 3.6, "kj": rng.uniform(80, 200) / 1000},
     [{"vf": 100, "kj": 120}, {"vf": 60, "kj": 150}],
     ["greenshields capacity", "capacity of a greenshields traffic stream", "greenshields model capacity"],
     conditions=["vf > 0", "kj > 0"], invalid=[{"vf": 100, "kj": -120}],
     method="q_max = vf kj / 4 at k = kj/2, v = vf/2", latex=r"q_{max} = \frac{v_f k_j}{4},\quad k_m = \frac{k_j}{2},\quad v_m = \frac{v_f}{2}")


def greenshields_at_flow(vf, kj, q):
    _pos(vf=vf, kj=kj, q=q)
    disc = 1 - 4 * q / (vf * kj)
    _need(disc >= -1e-12, "the flow exceeds the capacity vf kj / 4 of this traffic stream")
    sd = math.sqrt(max(0.0, disc))
    k1 = 2 * q / (vf * (1 + sd))
    k2 = kj * (1 + sd) / 2
    return {"k_1": _vpkm(k1), "v_1": _kmh(vf * (1 - k1 / kj)), "k_2": _vpkm(k2), "v_2": _kmh(vf * (1 - k2 / kj))}


def _ref_greenshields_at_flow(vf, kj, q):
    import numpy as np
    k1, k2 = sorted(float(r.real) for r in np.roots([vf / kj, -vf, q]))
    return {"k_1": k1 * 1000, "v_1": vf * (1 - k1 / kj) * 3.6, "k_2": k2 * 1000, "v_2": vf * (1 - k2 / kj) * 3.6}


proc("transport_greenshields_speed_at_flow", "Greenshields model: densities and speeds at a given flow", "traffic flow",
     "The two traffic states (uncongested and congested) of a Greenshields stream that carry the flow q: their "
     "densities and speeds.",
     [_V("vf", "free-flow speed", ("free flow speed", "free-flow speed")), _K("kj", "jam density", ("jam density",)),
      _Q("q", "traffic flow", ("flow", "traffic flow", "volume"))],
     [_o("k_1", "traffic_density", "uncongested density (veh/km)"), _o("v_1", "speed_kmh", "uncongested speed (km/h)"),
      _o("k_2", "traffic_density", "congested density (veh/km)"), _o("v_2", "speed_kmh", "congested speed (km/h)")],
     greenshields_at_flow, _ref_greenshields_at_flow,
     lambda vf, kj, q: (lambda o: {"k_1": o["k_2"], "v_1": o["v_2"], "k_2": o["k_1"], "v_2": o["v_1"]})(
         greenshields_at_flow(vf, kj, q)),
     lambda rng: (lambda vf, kj: {"vf": vf, "kj": kj, "q": rng.uniform(0.05, 0.92) * vf * kj / 4})(
         rng.uniform(40, 130) / 3.6, rng.uniform(80, 200) / 1000),
     [{"vf": 100, "kj": 120, "q": 2000}, {"vf": 80, "kj": 140, "q": 1200}],
     ["greenshields speed at a flow", "greenshields density at a flow", "greenshields states at a flow"],
     conditions=["0 < q <= vf kj / 4"], invalid=[{"vf": 100, "kj": 120, "q": 4000}],
     method="k = (kj/2)(1 -/+ sqrt(1 - 4q/(vf kj))), v = vf (1 - k/kj)",
     latex=r"k = \frac{k_j}{2}\left(1 \mp \sqrt{1 - \frac{4q}{v_f k_j}}\right)")


# --- fits of the classical speed-density models to observations ----------------------------------------------------------
def _linfit(x, y):
    n = len(x)
    mx, my = sum(x) / n, sum(y) / n
    sxx = sum((a - mx) ** 2 for a in x)
    syy = sum((b - my) ** 2 for b in y)
    sxy = sum((a - mx) * (b - my) for a, b in zip(x, y))
    _need(sxx > 0, "the densities must not all be equal")
    _need(syy > 0, "the speeds must not all be equal")
    b = sxy / sxx
    return my - b * mx, b, sxy * sxy / (sxx * syy)


_SYM_CAP = {}


def _sym_capacity(model):
    """Reference: the density at capacity and the capacity of a speed-density model by symbolic calculus (SymPy:
    dq/dk = 0 for q = k v(k)), lambdified once."""
    if model not in _SYM_CAP:
        import sympy as sp
        k, a, b = sp.symbols("k a b", positive=True)
        v = {"greenshields": a * (1 - k / b), "greenberg": a * sp.log(b / k), "underwood": a * sp.exp(-k / b)}[model]
        q = k * v
        km = [s for s in sp.solve(sp.diff(q, k), k) if s.is_positive is not False][0]
        _SYM_CAP[model] = (sp.lambdify((a, b), km, "math"), sp.lambdify((a, b), q.subs(k, km), "math"),
                           sp.lambdify((a, b), v.subs(k, km), "math"))
    return _SYM_CAP[model]


def _fit_inputs():
    return [_VEC("speeds", "observed space-mean speeds in km/h", ("speeds",), min_length=3),
            _VEC("densities", "observed densities in veh/km, in the same order", ("densities",), min_length=3)]


def _check_obs(speeds, densities):
    v = _vec("speeds", speeds, positive=True)
    k = _vec("densities", densities, positive=True)
    _need(len(v) == len(k), "give one density for every speed")
    _need(len(v) >= 3, "give at least three observations")
    return v, k


def greenshields_fit(speeds, densities):
    v, k = _check_obs(speeds, densities)
    a, b, r2 = _linfit(k, v)
    _need(a > 0 and b < 0, "the speeds do not fall with density: the Greenshields line does not fit")
    vf, kj = a, -a / b
    return {"vf": _r(vf), "kj": _r(kj), "q_max": _r(vf * kj / 4), "r2": _r(r2)}


def _ref_fit(model, speeds, densities):
    from scipy.stats import linregress
    import math as m_
    v, k = list(map(float, speeds)), list(map(float, densities))
    if model == "greenshields":
        res = linregress(k, v)
        p1, p2 = res.intercept, -res.intercept / res.slope
    elif model == "greenberg":
        res = linregress([m_.log(x) for x in k], v)
        p1 = -res.slope
        p2 = m_.exp(res.intercept / p1)
    else:
        res = linregress(k, [m_.log(x) for x in v])
        p1, p2 = m_.exp(res.intercept), -1 / res.slope
    _km, qm, _vm = _sym_capacity(model)
    return p1, p2, float(qm(p1, p2)), res.rvalue ** 2


def _rand_obs(model):
    def gen(rng):
        n = rng.randint(4, 12)
        a, b = rng.uniform(50, 120), rng.uniform(90, 200)
        ks = sorted(rng.uniform(0.05, 0.9) * b for _ in range(n))
        if model == "greenshields":
            vs = [a * (1 - kk / b) * rng.uniform(0.9, 1.1) for kk in ks]
        elif model == "greenberg":
            a = rng.uniform(15, 40)
            vs = [a * math.log(b / kk) * rng.uniform(0.9, 1.1) for kk in ks]
        else:
            b = rng.uniform(25, 70)
            ks = sorted(rng.uniform(0.1, 3.0) * b for _ in range(n))
            vs = [a * math.exp(-kk / b) * rng.uniform(0.9, 1.1) for kk in ks]
        return {"speeds": [round(x, 2) for x in vs], "densities": [round(x, 2) for x in ks]}
    return gen


_OBS1 = {"speeds": [82, 70, 58, 47, 33, 21], "densities": [18, 32, 46, 60, 78, 95]}
_OBS2 = {"speeds": [61.5, 48.2, 39.0, 27.4, 15.8], "densities": [25, 42, 58, 80, 105]}

proc("transport_greenshields_fit", "Fit the Greenshields model to speed-density observations", "traffic flow",
     "Least-squares fit of Greenshields' line v = vf - (vf/kj) k to observed speeds (km/h) and densities (veh/km): the "
     "free-flow speed, jam density, capacity and the coefficient of determination of the line.",
     _fit_inputs(),
     [_o("vf", "speed_kmh", "free-flow speed (km/h)"), _o("kj", "traffic_density", "jam density (veh/km)"),
      _o("q_max", "traffic_flow", "capacity (veh/h)"), _o("r2", "number", "coefficient of determination of the fit")],
     greenshields_fit, lambda speeds, densities: dict(zip(("vf", "kj", "q_max", "r2"), _ref_fit("greenshields", speeds, densities))),
     lambda speeds, densities: (lambda o: dict(o, q_max=_r(o["vf"] * o["kj"] / 2)))(greenshields_fit(speeds, densities)),
     _rand_obs("greenshields"), [_OBS1, _OBS2],
     ["greenshields fit", "fit the greenshields model", "greenshields regression"],
     conditions=["at least three observations", "speeds fall with density"],
     invalid=[{"speeds": [82, 70, 58], "densities": [18, 32]}],
     method="least squares of v on k: intercept vf, slope -vf/kj; q_max = vf kj / 4",
     latex=r"v = v_f - \frac{v_f}{k_j} k,\quad q_{max} = \frac{v_f k_j}{4}")


def greenberg_fit(speeds, densities):
    v, k = _check_obs(speeds, densities)
    a, b, r2 = _linfit([math.log(x) for x in k], v)
    _need(b < 0, "the speeds do not fall with density: the Greenberg model does not fit")
    v0 = -b
    kj = math.exp(a / v0)
    return {"v0": _r(v0), "kj": _r(kj), "q_max": _r(v0 * kj / math.e), "r2": _r(r2)}


proc("transport_greenberg_fit", "Fit the Greenberg (logarithmic) model to speed-density observations", "traffic flow",
     "Least-squares fit of Greenberg's model v = v0 ln(kj / k) (v against ln k) to observed speeds (km/h) and densities "
     "(veh/km): the optimum speed v0, jam density, capacity v0 kj / e and the coefficient of determination.",
     _fit_inputs(),
     [_o("v0", "speed_kmh", "optimum speed v0 (km/h)"), _o("kj", "traffic_density", "jam density (veh/km)"),
      _o("q_max", "traffic_flow", "capacity (veh/h)"), _o("r2", "number", "coefficient of determination (v on ln k)")],
     greenberg_fit, lambda speeds, densities: dict(zip(("v0", "kj", "q_max", "r2"), _ref_fit("greenberg", speeds, densities))),
     lambda speeds, densities: (lambda o: dict(o, q_max=_r(o["v0"] * o["kj"] / 2)))(greenberg_fit(speeds, densities)),
     _rand_obs("greenberg"), [_OBS1, _OBS2],
     ["greenberg fit", "fit the greenberg model", "greenberg regression"],
     conditions=["at least three observations", "speeds fall with density"],
     invalid=[{"speeds": [82, 70, 58], "densities": [18, 32]}],
     method="least squares of v on ln k: slope -v0, intercept v0 ln kj; q_max = v0 kj / e",
     latex=r"v = v_0 \ln\frac{k_j}{k},\quad q_{max} = \frac{v_0 k_j}{e}")


def underwood_fit(speeds, densities):
    v, k = _check_obs(speeds, densities)
    a, b, r2 = _linfit(k, [math.log(x) for x in v])
    _need(b < 0, "the speeds do not fall with density: the Underwood model does not fit")
    vf, k0 = math.exp(a), -1 / b
    return {"vf": _r(vf), "k0": _r(k0), "q_max": _r(vf * k0 / math.e), "r2": _r(r2)}


proc("transport_underwood_fit", "Fit the Underwood (exponential) model to speed-density observations", "traffic flow",
     "Least-squares fit of Underwood's model v = vf exp(-k / k0) (ln v against k) to observed speeds (km/h) and "
     "densities (veh/km): the free-flow speed, the optimum density k0, capacity vf k0 / e and the coefficient of "
     "determination.",
     _fit_inputs(),
     [_o("vf", "speed_kmh", "free-flow speed (km/h)"), _o("k0", "traffic_density", "optimum density (veh/km)"),
      _o("q_max", "traffic_flow", "capacity (veh/h)"), _o("r2", "number", "coefficient of determination (ln v on k)")],
     underwood_fit, lambda speeds, densities: dict(zip(("vf", "k0", "q_max", "r2"), _ref_fit("underwood", speeds, densities))),
     lambda speeds, densities: (lambda o: dict(o, q_max=_r(o["vf"] * o["k0"])))(underwood_fit(speeds, densities)),
     _rand_obs("underwood"), [_OBS1, _OBS2],
     ["underwood fit", "fit the underwood model", "underwood regression"],
     conditions=["at least three observations", "speeds fall with density"],
     invalid=[{"speeds": [82, 70, 58], "densities": [18, 32]}],
     method="least squares of ln v on k: intercept ln vf, slope -1/k0; q_max = vf k0 / e",
     latex=r"v = v_f\,e^{-k/k_0},\quad q_{max} = \frac{v_f k_0}{e}")


def greenberg_cap(v0, kj):
    _pos(v0=v0, kj=kj)
    return {"q_max": _vph(v0 * kj / math.e), "k_m": _vpkm(kj / math.e)}


def _ref_sym_cap(model, a, b):
    km, qm, vm = _sym_capacity(model)
    return float(km(a, b)), float(qm(a, b)), float(vm(a, b))


proc("transport_greenberg_capacity", "Greenberg model: capacity and density at capacity", "traffic flow",
     "Capacity of a traffic stream following Greenberg's model v = v0 ln(kj / k) and the density at which it occurs "
     "(the speed there is v0).",
     [_V("v0", "optimum speed v0", ("optimum speed",)), _K("kj", "jam density", ("jam density",))],
     [_o("q_max", "traffic_flow", "capacity (veh/h)"), _o("k_m", "traffic_density", "density at capacity (veh/km)")],
     greenberg_cap, lambda v0, kj: (lambda r: {"q_max": r[1] * 3600, "k_m": r[0] * 1000})(_ref_sym_cap("greenberg", v0, kj)),
     lambda v0, kj: {"q_max": _vph(v0 * kj / 2), "k_m": _vpkm(kj / 2)},
     lambda rng: {"v0": rng.uniform(15, 45) / 3.6, "kj": rng.uniform(80, 200) / 1000},
     [{"v0": 30, "kj": 140}, {"v0": 25, "kj": 180}],
     ["greenberg capacity", "greenberg model capacity"],
     conditions=["v0 > 0", "kj > 0"], invalid=[{"v0": -30, "kj": 140}],
     method="q = v0 k ln(kj/k) is largest at k = kj / e: q_max = v0 kj / e", latex=r"q_{max} = \frac{v_0 k_j}{e}")


def underwood_cap(vf, k0):
    _pos(vf=vf, k0=k0)
    return {"q_max": _vph(vf * k0 / math.e), "v_m": _kmh(vf / math.e), "k_m": _vpkm(k0)}


proc("transport_underwood_capacity", "Underwood model: capacity and speed at capacity", "traffic flow",
     "Capacity of a traffic stream following Underwood's model v = vf exp(-k / k0), with the speed and density at "
     "capacity.",
     [_V("vf", "free-flow speed", ("free flow speed", "free-flow speed")), _K("k0", "optimum density k0", ("optimum density",))],
     [_o("q_max", "traffic_flow", "capacity (veh/h)"), _o("v_m", "speed_kmh", "speed at capacity (km/h)"),
      _o("k_m", "traffic_density", "density at capacity (veh/km)")],
     underwood_cap, lambda vf, k0: (lambda r: {"q_max": r[1] * 3600, "v_m": r[2] * 3.6, "k_m": r[0] * 1000})(
         _ref_sym_cap("underwood", vf, k0)),
     lambda vf, k0: {"q_max": _vph(vf * k0 / 2), "v_m": _kmh(vf / 2), "k_m": _vpkm(k0)},
     lambda rng: {"vf": rng.uniform(40, 130) / 3.6, "k0": rng.uniform(20, 80) / 1000},
     [{"vf": 100, "k0": 50}, {"vf": 80, "k0": 35}],
     ["underwood capacity", "underwood model capacity"],
     conditions=["vf > 0", "k0 > 0"], invalid=[{"vf": 100, "k0": -50}],
     method="q = vf k exp(-k/k0) is largest at k = k0: q_max = vf k0 / e, v = vf / e",
     latex=r"q_{max} = \frac{v_f k_0}{e},\quad v_m = \frac{v_f}{e}")


# =============================================================================================== shockwaves
def _micro_wave(q1, k1, q2, k2, n=24):
    """Reference: a microscopic simulation. Vehicles of the upstream state (spacing 1/k1, speed q1/k1) take the
    downstream state (speed q2/k2) at the moment their gap to the vehicle ahead has become 1/k2; the boundary between
    the states is traced through these events and its speed measured from the first and last event."""
    v1, v2, s1, s2 = q1 / k1, q2 / k2, 1 / k1, 1 / k2
    T, X = [0.0], [0.0]
    for i in range(1, n + 1):
        dt = (s2 - s1) / (v2 - v1)
        t = T[-1] + dt
        T.append(t)
        X.append(-i * s1 + v1 * t)
    return (X[-1] - X[0]) / (T[-1] - T[0])


def _wave_check(q1, k1, q2, k2):
    _pos(k1=k1, k2=k2)
    _nonneg(q1=q1, q2=q2)
    v1, v2 = q1 / k1, q2 / k2
    _need(k1 != k2 and v1 != v2, "the two states must differ in density and speed")
    _need((v1 - v2) * (k2 - k1) > 0, "the states are inconsistent: a denser state must be slower")


def shockwave(q1, k1, q2, k2):
    _wave_check(q1, k1, q2, k2)
    u = (q2 - q1) / (k2 - k1)
    u = _snap(u, max(q1, q2) / min(k1, k2))
    return {"u": _kmh(u), "u_ms": _r(u), "direction": "stationary" if u == 0 else ("downstream" if u > 0 else "upstream")}


def _ref_shockwave(q1, k1, q2, k2):
    u = _micro_wave(q1, k1, q2, k2)
    u = 0.0 if abs(u) <= 1e-12 * max(q1, q2) / min(k1, k2) else u
    return {"u": u * 3.6, "u_ms": u, "direction": "stationary" if u == 0 else ("downstream" if u > 0 else "upstream")}


def _rand_states(rng):
    k1, k2 = rng.uniform(5, 140) / 1000, rng.uniform(5, 140) / 1000
    while abs(k1 - k2) < 0.01:
        k2 = rng.uniform(5, 140) / 1000
    lo, hi = min(k1, k2), max(k1, k2)
    vhi, vlo = rng.uniform(15, 33), rng.uniform(0.0, 12)
    v = {lo: vhi, hi: vlo}
    return {"q1": k1 * v[k1], "k1": k1, "q2": k2 * v[k2], "k2": k2}


proc("transport_shockwave_speed", "Shockwave speed between two traffic states", "traffic flow",
     "Speed of the boundary (shockwave) between an upstream traffic state (q1, k1) and a downstream state (q2, k2): "
     "u = (q2 - q1) / (k2 - k1), negative when it moves upstream (for example the back of a queue).",
     [_Q("q1", "upstream flow", ("upstream flow",)), _K("k1", "upstream density", ("upstream density",)),
      _Q("q2", "downstream flow", ("downstream flow",)), _K("k2", "downstream density", ("downstream density",))],
     [_o("u", "speed_kmh", "shockwave speed (km/h), positive downstream"), _o("u_ms", "speed", "shockwave speed (m/s)"),
      _o("direction", "text", "upstream, downstream or stationary")],
     shockwave, _ref_shockwave,
     lambda q1, k1, q2, k2: (lambda u: {"u": _kmh(u), "u_ms": _r(u), "direction": "?"})((q2 + q1) / (k2 - k1)),
     _rand_states,
     [{"q1": 1800, "k1": 25, "q2": 1200, "k2": 80}, {"q1": 0, "k1": 150, "q2": 1900, "k2": 40}],
     ["shockwave speed", "shock wave speed", "speed of a shockwave"],
     conditions=["k1, k2 > 0, q1, q2 >= 0", "the denser state is the slower one"],
     invalid=[{"q1": 3000, "k1": 60, "q2": 500, "k2": 25}],
     method="u = (q2 - q1) / (k2 - k1)", latex=r"u_w = \frac{q_2 - q_1}{k_2 - k_1}")


def gs_shockwave(vf, kj, k1, k2):
    _pos(vf=vf, kj=kj, k1=k1, k2=k2)
    _need(k1 < k2 < kj, "a Greenshields shock needs k1 < k2 < kj (a lighter state running into a denser one)")
    u = vf * (1 - (k1 + k2) / kj)
    u = _snap(u, vf)
    return {"u": _kmh(u), "u_ms": _r(u)}


def _ref_gs_shockwave(vf, kj, k1, k2):
    q = lambda k: k * vf * (1 - k / kj)
    u = _micro_wave(q(k1), k1, q(k2), k2)
    u = 0.0 if abs(u) <= 1e-12 * vf else u
    return {"u": u * 3.6, "u_ms": u}


proc("transport_greenshields_shockwave", "Shockwave speed in a Greenshields traffic stream", "traffic flow",
     "Speed of the shockwave between two states of a Greenshields stream with densities k1 (upstream) < k2 "
     "(downstream): u = vf (1 - (k1 + k2) / kj).",
     [_V("vf", "free-flow speed", ("free flow speed", "free-flow speed")), _K("kj", "jam density", ("jam density",)),
      _K("k1", "upstream density", ("upstream density",)), _K("k2", "downstream density", ("downstream density",))],
     [_o("u", "speed_kmh", "shockwave speed (km/h), positive downstream"), _o("u_ms", "speed", "shockwave speed (m/s)")],
     gs_shockwave, _ref_gs_shockwave, lambda vf, kj, k1, k2: (lambda u: {"u": _kmh(u), "u_ms": _r(u)})(vf * (1 - (k2 - k1) / kj)),
     lambda rng: (lambda kj: (lambda a, b: {"vf": rng.uniform(50, 120) / 3.6, "kj": kj, "k1": min(a, b), "k2": max(a, b)})(
         rng.uniform(0.02, 0.97) * kj, rng.uniform(0.02, 0.97) * kj))(rng.uniform(90, 200) / 1000),
     [{"vf": 90, "kj": 130, "k1": 30, "k2": 90}, {"vf": 100, "kj": 120, "k1": 20, "k2": 45}],
     ["greenshields shockwave", "greenshields shock wave speed"],
     conditions=["0 < k1 < k2 < kj"], invalid=[{"vf": 90, "kj": 130, "k1": 100, "k2": 30}],
     method="u = vf (1 - (k1 + k2)/kj)", latex=r"u_w = v_f\left(1 - \frac{k_1 + k_2}{k_j}\right)")


def signal_shock_queue(q_a, k_a, k_j, q_s, k_s, r):
    _pos(q_a=q_a, k_a=k_a, k_j=k_j, q_s=q_s, k_s=k_s, r=r)
    _need(k_a < k_j and k_s < k_j, "the arrival and discharge densities must be below the jam density")
    w1 = -q_a / (k_j - k_a)
    w2 = -q_s / (k_j - k_s)
    _need(w2 < w1, "the discharge wave must be faster than the queue growth (q_s/(k_j - k_s) > q_a/(k_j - k_a))")
    t = w2 * r / (w2 - w1)
    return {"L_max": _r(-w1 * t), "t_max": _r(t), "w_queue": _kmh(w1), "w_discharge": _kmh(w2)}


def _ref_signal_shock(q_a, k_a, k_j, q_s, k_s, r):
    """Reference: a microscopic simulation of the queue at the signal. Arriving vehicles (spacing 1/k_a, speed
    q_a/k_a) stop when their gap to the stopped vehicle ahead is 1/k_j; from the start of green (time r) each starts
    at the discharge speed q_s/k_s when its gap to the departing vehicle ahead has become 1/k_s. The stop events and
    the start events each lie on a line (the two waves); the line through the stop events and the line through the
    start events are intersected (NumPy) to give the longest queue and its time."""
    import numpy as np
    va, vs, sa, sj, ss = q_a / k_a, q_s / k_s, 1 / k_a, 1 / k_j, 1 / k_s
    n = 30
    Xs, Ts = [0.0], [0.0]                         # stop positions and times
    for i in range(1, n + 1):
        X = Xs[-1] - sj
        Ts.append((X + i * sa) / va)
        Xs.append(X)
    S = [r]                                       # start times of the stopped vehicles
    for i in range(1, n + 1):
        S.append(S[-1] + (ss - sj) / vs)
    w1 = (Xs[-1] - Xs[1]) / (Ts[-1] - Ts[1])
    w2 = (Xs[-1] - Xs[1]) / (S[-1] - S[1])
    # x = w1 t (back of the queue) and x = w2 (t - r) (discharge front)
    t, x = np.linalg.solve(np.array([[w1, -1.0], [w2, -1.0]]), np.array([0.0, w2 * r]))
    return {"L_max": -float(x), "t_max": float(t), "w_queue": w1 * 3.6, "w_discharge": w2 * 3.6}


proc("transport_signal_queue_shockwave", "Queue at a signal by shockwave analysis", "traffic signals",
     "Kinematic-wave analysis of a signal: the queue grows upstream during the red at the shock speed between the "
     "arrival state (q_a, k_a) and jam density k_j, and from the start of green the discharge wave (saturation state "
     "q_s, k_s) chases it. The longest queue, the time it is reached (from the start of red) and both wave speeds.",
     [_Q("q_a", "arrival flow", ("arrival flow",)), _K("k_a", "arrival density", ("arrival density",)),
      _K("k_j", "jam density", ("jam density",)), _Q("q_s", "saturation (discharge) flow", ("saturation flow",
                                                                                        "discharge flow")),
      _K("k_s", "density of the discharging stream", ("discharge density", "saturation density")),
      _T("r", "red time", ("red time", "red"))],
     [_o("L_max", "length", "longest queue (m)"), _o("t_max", "time", "time from the start of red to the longest queue (s)"),
      _o("w_queue", "speed_kmh", "speed of the back of the queue (km/h)"),
      _o("w_discharge", "speed_kmh", "speed of the discharge wave (km/h)")],
     signal_shock_queue, _ref_signal_shock,
     lambda q_a, k_a, k_j, q_s, k_s, r: (lambda w1, w2: (lambda t: {"L_max": _r(-w1 * t), "t_max": _r(t),
                                                                     "w_queue": _kmh(w1), "w_discharge": _kmh(w2)})(
         w2 * r / (w2 - w1)))(-q_a / (k_j - k_a), -q_s / (k_j + k_s)),
     lambda rng: (lambda kj: (lambda ka, ks: {"q_a": ka * rng.uniform(12, 25), "k_a": ka, "k_j": kj,
                                              "q_s": ks * rng.uniform(10, 16), "k_s": ks, "r": rng.uniform(15, 90)})(
         rng.uniform(0.05, 0.25) * kj, rng.uniform(0.3, 0.5) * kj))(rng.uniform(120, 180) / 1000),
     [{"q_a": 1000, "k_a": 20, "k_j": 140, "q_s": 1800, "k_s": 50, "r": 40},
      {"q_a": 700, "k_a": 15, "k_j": 150, "q_s": 1600, "k_s": 45, "r": 30}],
     ["queue at a signal by shockwave analysis", "signal shockwave queue", "shockwave queue at a signal"],
     conditions=["k_a, k_s < k_j", "the discharge wave is faster than the queue growth"],
     invalid=[{"q_a": 1000, "k_a": 160, "k_j": 140, "q_s": 1800, "k_s": 50, "r": 40}],
     method="w1 = -q_a/(k_j - k_a), w2 = -q_s/(k_j - k_s); they meet at t = w2 r/(w2 - w1); L_max = -w1 t",
     latex=r"t^* = \frac{w_2\, r}{w_2 - w_1},\quad L_{max} = -w_1 t^*")


# =============================================================================================== deterministic queues
def _newell_departures(A, S, breaks):
    """Reference: Newell's cumulative-curve construction of a queue, D(t) = min over u <= t of A(u) + S(t) - S(u),
    with A the cumulative arrivals and S the cumulative service capacity (both piecewise linear: the minimum is taken
    over their breakpoints and u = t)."""
    def D(t):
        return min([A(u) + S(t) - S(u) for u in breaks if u <= t] + [A(t)])
    return D


def signal_dd1(lam, s, r, g):
    _pos(lam=lam, s=s, r=r, g=g)
    _need(lam < s, "the arrival flow must be below the saturation flow")
    C = r + g
    tc = lam * r / (s - lam)
    _need(tc <= g * (1 + 1e-12), "the queue does not clear within the green (oversaturated: lam C > s g)")
    rho = lam / s
    return {"Q_max": _r(lam * r), "t_clear": _r(tc), "D_total": _r(lam * r * r / (2 * (1 - rho))),
            "d_uniform": _r(r * r / (2 * C * (1 - rho))), "P_stopped": _r((r + tc) / C)}


def _ref_signal_dd1(lam, s, r, g):
    C = r + g
    A = lambda t: lam * t
    S = lambda t: s * max(0.0, t - r)
    D = _newell_departures(A, S, [0.0, r])
    T = _brentq(lambda t: A(t) - S(t), r, C)                        # the cumulative curves meet: the queue is gone
    Dt = _quad(lambda t: A(t) - D(t), 0.0, C, points=[r, T])
    qmax = max(A(t) - D(t) for t in (0.0, r, T, C))
    return {"Q_max": qmax, "t_clear": T - r, "D_total": Dt, "d_uniform": Dt / A(C), "P_stopped": A(T) / A(C)}


proc("transport_signal_queue_uniform_delay", "Queue and uniform delay at a signal (deterministic D/D/1)",
     "traffic signals",
     "Uniform arrivals at a fixed-time signal with saturation flow s, effective red r and effective green g "
     "(undersaturated): the largest queue, the time after the start of green until it clears, the total delay per "
     "cycle, the average (uniform) delay per vehicle and the share of vehicles that stop.",
     [_Q("lam", "arrival flow", ("arrival flow", "flow")), _Q("s", "saturation flow", ("saturation flow",)),
      _T("r", "effective red", ("red", "effective red")), _T("g", "effective green", ("green", "effective green"))],
     [_o("Q_max", "number", "largest queue (veh)"), _o("t_clear", "time", "time after the start of green to clear the queue (s)"),
      _o("D_total", "vehicle_seconds", "total delay per cycle (veh s)"),
      _o("d_uniform", "time", "average delay per vehicle (s)"), _o("P_stopped", "number", "share of vehicles that stop")],
     signal_dd1, _ref_signal_dd1,
     lambda lam, s, r, g: (lambda o: dict(o, D_total=_r(lam * r * r / 2), d_uniform=_r(r * r / (2 * (r + g)))))(
         signal_dd1(lam, s, r, g)),
     lambda rng: (lambda s, r, g: {"s": s, "r": r, "g": g, "lam": rng.uniform(0.1, 0.95) * s * g / (r + g)})(
         rng.uniform(1200, 2000) / 3600, rng.uniform(15, 70), rng.uniform(15, 70)),
     [{"lam": 600, "s": 1800, "r": 40, "g": 30}, {"lam": 800, "s": 1900, "r": 35, "g": 45}],
     ["signal queue and uniform delay", "uniform delay at a signal", "deterministic queue at a signal", "d/d/1 signal queue"],
     conditions=["lam < s", "the queue clears within the green (lam C <= s g)"],
     invalid=[{"lam": 1200, "s": 1800, "r": 40, "g": 30}],
     method="Q = lam r, t_c = lam r/(s - lam), D = lam r^2 / (2(1 - lam/s)), d = r^2 / (2C (1 - lam/s))",
     latex=r"d = \frac{r^2}{2C\,(1 - \lambda/s)} = \frac{C (1 - g/C)^2}{2 (1 - \lambda/s)}")


def bottleneck(lam, mu, mu_r, t_r):
    _pos(lam=lam, mu=mu, mu_r=mu_r, t_r=t_r)
    _need(mu_r < lam < mu, "need reduced capacity < arrival flow < full capacity")
    Q = (lam - mu_r) * t_r
    T = t_r + Q / (mu - lam)
    return {"Q_max": _r(Q), "T": _r(T), "T_min": _r(T / 60), "D_total": _r(Q * T / 2 / 3600)}


def _ref_bottleneck(lam, mu, mu_r, t_r):
    A = lambda t: lam * t
    S = lambda t: mu_r * t if t <= t_r else mu_r * t_r + mu * (t - t_r)
    D = _newell_departures(A, S, [0.0, t_r])
    hi = t_r * 2
    while A(hi) - S(hi) > 0:
        hi *= 2
    T = _brentq(lambda t: A(t) - S(t), t_r, hi)
    Dt = _quad(lambda t: A(t) - D(t), 0.0, T, points=[t_r])
    return {"Q_max": max(A(t) - D(t) for t in (0.0, t_r, T)), "T": T, "T_min": T / 60, "D_total": Dt / 3600}


proc("transport_bottleneck_queue", "Queue and delay at a temporary bottleneck (incident, lane closure)",
     "traffic flow",
     "Deterministic queue when the capacity of a road falls from mu to mu_r for a time t_r (an incident or a work "
     "zone) while vehicles arrive at lam: the longest queue, how long the queue lasts and the total delay.",
     [_Q("lam", "arrival flow", ("arrival flow", "flow")), _Q("mu", "normal capacity", ("capacity", "normal capacity")),
      _Q("mu_r", "reduced capacity", ("reduced capacity",)), _T("t_r", "duration of the reduction", ("duration",
                                                                                                   "incident duration"), show="min")],
     [_o("Q_max", "number", "longest queue (veh)"), _o("T", "time", "duration of the queue from the start of the reduction (s)"),
      _o("T_min", "time_min", "duration of the queue (min)"), _o("D_total", "vehicle_hours", "total delay (veh h)")],
     bottleneck, _ref_bottleneck,
     lambda lam, mu, mu_r, t_r: (lambda o: dict(o, D_total=_r(o["Q_max"] * o["T"] / 3600)))(bottleneck(lam, mu, mu_r, t_r)),
     lambda rng: (lambda mu: (lambda lam: {"mu": mu, "lam": lam, "mu_r": rng.uniform(0.2, 0.9) * lam,
                                           "t_r": rng.uniform(5, 120) * 60})(rng.uniform(0.5, 0.95) * mu))(
         rng.uniform(2000, 6000) / 3600),
     [{"lam": 3200, "mu": 4000, "mu_r": 1800, "t_r": 30}, {"lam": 1500, "mu": 2000, "mu_r": 1000, "t_r": 20}],
     ["bottleneck queue", "incident queue", "queue at a lane closure"],
     conditions=["mu_r < lam < mu"], invalid=[{"lam": 3200, "mu": 3000, "mu_r": 1800, "t_r": 30}],
     method="Q = (lam - mu_r) t_r, T = t_r + Q/(mu - lam), total delay = Q T / 2",
     latex=r"Q_{max} = (\lambda - \mu_r) t_r,\quad T = t_r + \frac{Q_{max}}{\mu - \lambda},\quad D = \tfrac12 Q_{max} T")


def interval_queue(arrivals, capacity):
    a = _vec("arrivals", arrivals, nonneg=True)
    c = _vec("capacity", capacity, nonneg=True)
    if len(c) == 1:
        c = c * len(a)
    _need(len(c) == len(a), "give one capacity for every interval (or a single capacity for all)")
    q, queue, served = 0.0, [], []
    for x, y in zip(a, c):
        nq = max(0.0, q + x - y)
        served.append(q + x - nq)
        q = nq
        queue.append(q)
    return {"queue": _r(queue), "Q_max": _r(max(queue)), "served": _r(served)}


def _ref_interval_queue(arrivals, capacity):
    import numpy as np
    a = np.asarray(arrivals, float)
    c = np.asarray(capacity, float)
    if len(c) == 1:
        c = np.repeat(c, len(a))
    A, S = np.cumsum(a), np.cumsum(c)
    D = S + np.minimum(0.0, np.minimum.accumulate(A - S))          # Newell: D_k = min over j <= k of A_j + S_k - S_j
    D = np.minimum(D, A)
    Q = A - D
    served = np.diff(np.concatenate([[0.0], D]))
    return {"queue": Q.tolist(), "Q_max": float(Q.max()), "served": served.tolist()}


proc("transport_interval_queue", "Queue from arrival counts and capacity per interval", "traffic flow",
     "Vehicles queued at the end of each counting interval when the arrivals per interval are served up to the "
     "capacity per interval (deterministic queue, first in first out), the longest queue and the vehicles served per "
     "interval.",
     [_VEC("arrivals", "arrivals in each interval (veh)", ("arrivals", "arrival counts")),
      _VEC("capacity", "capacity of each interval (veh), or one value for all", ("capacity", "capacities"))],
     [_o("queue", "number", "queue at the end of each interval (veh)"), _o("Q_max", "number", "longest queue (veh)"),
      _o("served", "number", "vehicles served in each interval (veh)")],
     interval_queue, _ref_interval_queue,
     lambda arrivals, capacity: (lambda o: dict(o, Q_max=_r(o["Q_max"] + max(arrivals))))(interval_queue(arrivals, capacity)),
     lambda rng: (lambda n, cap: {"arrivals": [round(rng.uniform(0.3, 1.6) * cap) for _ in range(n)],
                                  "capacity": [cap] if rng.random() < 0.5 else [round(rng.uniform(0.6, 1.2) * cap) for _ in range(n)]})(
         rng.randint(4, 16), rng.randint(20, 200)),
     [{"arrivals": [90, 120, 140, 110, 80, 60], "capacity": [100]},
      {"arrivals": [30, 55, 70, 40, 20], "capacity": [45, 45, 30, 45, 45]}],
     ["interval queue", "queue from arrival counts", "queue per interval"],
     conditions=["counts >= 0", "capacity: one value or one per interval"],
     invalid=[{"arrivals": [90, 120, 140], "capacity": [100, 90]}],
     method="Q_k = max(0, Q_{k-1} + a_k - c_k)", latex=r"Q_k = \max\left(0,\ Q_{k-1} + a_k - c_k\right)")


# =============================================================================================== signal timing
def green_splits(C, L, y):
    _pos(C=C)
    _nonneg(L=L)
    ys = _vec("y", y, positive=True)
    _need(len(ys) >= 2, "give the flow ratio of at least two critical phases")
    _need(C > L, "the cycle must be longer than the lost time")
    Y = sum(ys)
    _need(Y < 1, "the critical flow ratios add up to 1 or more: no cycle can serve them")
    return {"g": _r([yi / Y * (C - L) for yi in ys]), "X_c": _r(Y * C / (C - L))}


def _ref_green_splits(C, L, y):
    """Greens that give every critical phase the same degree of saturation x (y_i C / g_i = x) and use the whole
    effective green time (sum g_i = C - L), solved as one linear system in (g_1 .. g_n, 1/x) by NumPy."""
    import numpy as np
    n = len(y)
    A = np.zeros((n + 1, n + 1))
    b = np.zeros(n + 1)
    for i, yi in enumerate(y):
        A[i, i], A[i, n] = 1.0, -yi * C
    A[n, :n] = 1.0
    b[n] = C - L
    sol = np.linalg.solve(A, b)
    return {"g": sol[:n].tolist(), "X_c": 1.0 / sol[n]}


proc("transport_signal_green_splits", "Green splits by equal degree of saturation (flow-ratio method)",
     "traffic signals",
     "Effective green time of each critical phase for a given cycle C and total lost time L, shared in proportion to "
     "the critical flow ratios y_i = v_i / s_i (equal degree of saturation), and the critical degree of saturation "
     "X_c. Greens in s.",
     [_T("C", "cycle length", ("cycle", "cycle length")), _T("L", "total lost time per cycle", ("lost time",)),
      _VEC("y", "critical flow ratio v/s of each phase", ("flow ratios", "y"), min_length=2)],
     [_o("g", "number", "effective green of each phase (s)"), _o("X_c", "number", "critical degree of saturation")],
     green_splits, _ref_green_splits,
     lambda C, L, y: {"g": _r([yi / sum(y) * C for yi in y]), "X_c": _r(sum(y))},
     lambda rng: (lambda n: (lambda ys: {"C": rng.uniform(1.3, 3.0) * 3 * n * 1.0 / (1 - sum(ys)) + 3 * n + 10,
                                         "L": 3.0 * n + rng.uniform(0, 4), "y": [round(v, 3) for v in ys]})(
         [rng.uniform(0.05, 0.85 / n) for _ in range(n)]))(rng.randint(2, 5)),
     [{"C": 90, "L": 10, "y": [0.30, 0.25, 0.15]}, {"C": 60, "L": 8, "y": [0.4, 0.3]}],
     ["signal green splits", "green time allocation", "green splits"],
     conditions=["C > L", "sum of y < 1"], invalid=[{"C": 90, "L": 10, "y": [0.6, 0.5]}],
     method="g_i = (y_i / Y)(C - L), X_c = Y C / (C - L), Y = sum y_i",
     latex=r"g_i = \frac{y_i}{Y}(C - L),\quad X_c = \frac{Y\,C}{C - L}")


# =============================================================================================== arrivals, headways, gaps
def _pois_terms(lam, n):
    lp = lambda i: i * math.log(lam) - lam - math.lgamma(i + 1)
    p_n = math.exp(lp(n))
    le = sum(math.exp(lp(i)) for i in range(n + 1))
    gt, i = 0.0, n + 1
    while True:
        t = math.exp(lp(i))
        gt += t
        if i > lam and t < 1e-18 * max(gt, 1e-300):
            break
        i += 1
        if i > n + 100000:
            break
    return p_n, le, gt


def poisson_arrivals(q, t, n):
    _pos(q=q, t=t)
    _need(isinstance(n, int) and n >= 0, "n must be a whole number >= 0")
    lam = q * t
    _need(lam <= 1000, "the expected count q t must be at most 1000")
    p_n, le, gt = _pois_terms(lam, n)
    return {"P_n": _r(p_n), "P_le": _r(le), "P_ge": _r(p_n + gt)}


def _ref_poisson(q, t, n):
    from scipy.stats import poisson
    lam = q * t
    return {"P_n": float(poisson.pmf(n, lam)), "P_le": float(poisson.cdf(n, lam)), "P_ge": float(poisson.sf(n - 1, lam))}


proc("transport_poisson_arrivals", "Probability of n vehicle arrivals in an interval (Poisson)", "traffic flow",
     "Probability that exactly n, at most n and at least n vehicles arrive in an interval t when arrivals are random "
     "(Poisson) at the flow q.",
     [_Q("q", "traffic flow", ("flow", "traffic flow", "arrival rate")), _T("t", "interval length", ("interval",)),
      _in("n", "number", "number", "number of arrivals (whole number)", ("arrivals", "number of arrivals"), typ="integer", min=0)],
     [_o("P_n", "number", "P(exactly n arrivals)"), _o("P_le", "number", "P(at most n arrivals)"),
      _o("P_ge", "number", "P(at least n arrivals)")],
     poisson_arrivals, _ref_poisson,
     lambda q, t, n: (lambda lam: {"P_n": _r(lam ** n / math.factorial(n)), "P_le": 0.5, "P_ge": 0.5})(q * t),
     lambda rng: (lambda lam: {"q": lam / 60, "t": 60.0, "n": rng.randint(0, int(2 * lam) + 2)})(rng.uniform(0.5, 30)),
     [{"q": 360, "t": 30, "n": 2}, {"q": 900, "t": 60, "n": 10}],
     ["poisson arrivals", "probability of arrivals", "probability of n arrivals"],
     conditions=["q > 0", "t > 0", "n a whole number >= 0"], invalid=[{"q": -360, "t": 30, "n": 2}],
     method="P(n) = (q t)^n e^(-q t) / n!", latex=r"P(n) = \frac{(qt)^n e^{-qt}}{n!}")


def headway_prob(q, h, tau=0.0):
    _pos(q=q)
    _nonneg(h=h, tau=tau)
    mean = 1 / q
    _need(tau < mean, "the minimum headway must be shorter than the mean headway 1/q")
    if h <= tau:
        return {"P_ge": 1.0, "P_lt": 0.0}
    z = (h - tau) / (mean - tau)
    return {"P_ge": _r(math.exp(-z)), "P_lt": _r(-math.expm1(-z))}


def _ref_headway(q, h, tau=0.0):
    from scipy.stats import expon
    d = expon(loc=tau, scale=1 / q - tau)
    return {"P_ge": float(d.sf(h)), "P_lt": float(d.cdf(h))}


proc("transport_headway_probability", "Probability of a headway longer than h (negative exponential)", "traffic flow",
     "Probability that the time headway between random vehicles at flow q is at least h (and shorter than h), for the "
     "negative exponential distribution or the shifted one with a minimum headway tau.",
     [_Q("q", "traffic flow", ("flow", "traffic flow")), _T("h", "headway", ("headway", "gap")),
      _T("tau", "minimum headway (default 0)", ("minimum headway",), required=False)],
     [_o("P_ge", "number", "P(headway >= h)"), _o("P_lt", "number", "P(headway < h)")],
     headway_prob, _ref_headway,
     lambda q, h, tau=0.0: (lambda p: {"P_ge": _r(p), "P_lt": _r(1 - p)})(math.exp(-q * max(0.0, h - tau))),
     lambda rng: (lambda q: {"q": q, "h": rng.uniform(0.5, 3.0) / q, "tau": rng.uniform(0.3, 2.0)})(rng.uniform(200, 1500) / 3600),
     [{"q": 600, "h": 8}, {"q": 1200, "h": 4, "tau": 1}],
     ["headway probability", "probability of a headway", "negative exponential headway"],
     conditions=["q > 0", "tau < 1/q"], invalid=[{"q": 1200, "h": 4, "tau": 5}],
     method="P(H >= h) = exp(-(h - tau)/(1/q - tau))", latex=r"P(H \ge h) = e^{-(h-\tau)/(\bar h - \tau)},\ \bar h = 1/q")


def gap_capacity(q, t_c, t_f, model="harders"):
    _pos(q=q, t_c=t_c, t_f=t_f)
    if model == "harders":
        c = q * math.exp(-q * t_c) / -math.expm1(-q * t_f)
    else:
        _need(t_c > t_f / 2, "Siegloch's model needs t_c > t_f / 2")
        c = math.exp(-q * (t_c - t_f / 2)) / t_f
    return {"c": _vph(c)}


def _ref_gap_capacity(q, t_c, t_f, model="harders"):
    """Expected number of minor-stream vehicles that enter one major-stream gap, integrated over the exponential gap
    distribution (SciPy quadrature), times the number of gaps per second q."""
    dens = lambda h: q * math.exp(-q * h)
    if model == "harders":
        E, k = 0.0, 1
        while True:                     # k vehicles enter a gap of length t_c + (k-1) t_f <= h < t_c + k t_f
            a, b = t_c + (k - 1) * t_f, t_c + k * t_f
            piece = k * _quad(dens, a, b)
            E += piece
            if piece < 1e-18 * E:
                break
            k += 1
    else:
        t0 = t_c - t_f / 2
        E = _quad(lambda h: (h - t0) / t_f * dens(h), t0, math.inf)
    return {"c": q * E * 3600}


proc("transport_gap_acceptance_capacity", "Capacity of a minor stream by gap acceptance", "unsignalized intersections",
     "Capacity of a minor (stop- or yield-controlled) movement crossing a random major stream of flow q, with "
     "critical gap t_c and follow-up time t_f: Harders' formula (as in the HCM) or Siegloch's.",
     [_Q("q", "major-stream flow", ("major flow", "conflicting flow")), _T("t_c", "critical gap", ("critical gap",)),
      _T("t_f", "follow-up time", ("follow up time", "follow-up time")),
      _in("model", "number", "number", "harders (HCM, default) or siegloch", ("model",), typ="choice",
          choices=["harders", "siegloch"], required=False)],
     [_o("c", "traffic_flow", "capacity of the minor movement (veh/h)")],
     gap_capacity, _ref_gap_capacity,
     lambda q, t_c, t_f, model="harders": {"c": _vph(q * math.exp(-q * t_c) / -math.expm1(-q * t_c))},
     lambda rng: {"q": rng.uniform(100, 1500) / 3600, "t_c": rng.uniform(4.0, 7.5), "t_f": rng.uniform(2.0, 4.0),
                  "model": rng.choice(["harders", "siegloch"])},
     [{"q": 800, "t_c": 6.5, "t_f": 3.3}, {"q": 600, "t_c": 5.5, "t_f": 3.0, "model": "siegloch"}],
     ["gap acceptance capacity", "minor stream capacity", "capacity of a minor movement"],
     conditions=["q, t_c, t_f > 0", "Siegloch: t_c > t_f / 2"], invalid=[{"q": 800, "t_c": -6.5, "t_f": 3.3}],
     method="Harders: c = q e^(-q t_c) / (1 - e^(-q t_f)); Siegloch: c = e^(-q (t_c - t_f/2)) / t_f",
     latex=r"c = \frac{q\,e^{-q t_c}}{1 - e^{-q t_f}}")


# =============================================================================================== spot speeds
def _speeds(v):
    s = _vec("speeds", v, positive=True)
    _need(len(s) >= 2, "give at least two speeds")
    return s


def mean_speeds(speeds):
    s = _speeds(speeds)
    n = len(s)
    tms = math.fsum(s) / n
    sms = n / math.fsum(1 / x for x in s)
    sd = math.sqrt(math.fsum((x - tms) ** 2 for x in s) / (n - 1))
    return {"TMS": _r(tms), "SMS": _r(sms), "s": _r(_snap(sd, tms))}


def _ref_mean_speeds(speeds):
    import numpy as np
    from scipy.stats import hmean
    a = np.asarray(speeds, float)
    return {"TMS": float(np.mean(a)), "SMS": float(hmean(a)), "s": float(np.std(a, ddof=1))}


proc("transport_time_and_space_mean_speed", "Time-mean and space-mean speed of spot speeds", "traffic studies",
     "From spot speeds measured at a point: the time-mean speed (arithmetic mean), the space-mean speed (harmonic "
     "mean, the speed that matches travel times over a fixed length) and the sample standard deviation. Speeds in "
     "km/h.",
     [_VEC("speeds", "spot speeds in km/h", ("speeds", "spot speeds"), min_length=2)],
     [_o("TMS", "speed_kmh", "time-mean speed (km/h)"), _o("SMS", "speed_kmh", "space-mean speed (km/h)"),
      _o("s", "speed_kmh", "sample standard deviation (km/h)")],
     mean_speeds, _ref_mean_speeds, lambda speeds: (lambda o: dict(o, SMS=o["TMS"]))(mean_speeds(speeds)),
     lambda rng: {"speeds": [round(rng.uniform(30, 120), 1) for _ in range(rng.randint(3, 30))]},
     [{"speeds": [52, 61, 48, 70, 55, 63]}, {"speeds": [88, 95, 102, 79, 110, 92, 85]}],
     ["time mean and space mean speed", "space mean speed", "time mean speed"],
     conditions=["at least two positive speeds"], invalid=[{"speeds": [52, -61, 48]}],
     method="TMS = sum v / n; SMS = n / sum(1/v); s = sqrt(sum (v - TMS)^2 / (n - 1))",
     latex=r"\bar v_t = \frac{1}{n}\sum v_i,\quad \bar v_s = \frac{n}{\sum 1/v_i}")


def _pct(sorted_v, p):
    """Linear interpolation between closest ranks (the 'inclusive' definition: position (n - 1) p / 100)."""
    pos = (len(sorted_v) - 1) * p / 100
    i = int(math.floor(pos))
    f = pos - i
    if i + 1 >= len(sorted_v):
        return sorted_v[-1]
    return sorted_v[i] + f * (sorted_v[i + 1] - sorted_v[i])


def speed_percentiles(speeds):
    s = sorted(_speeds(speeds))
    return {"p85": _r(_pct(s, 85)), "p50": _r(_pct(s, 50)), "p15": _r(_pct(s, 15))}


def _ref_percentiles(speeds):
    import numpy as np
    a = np.asarray(speeds, float)
    return {"p85": float(np.percentile(a, 85)), "p50": float(np.percentile(a, 50)), "p15": float(np.percentile(a, 15))}


proc("transport_spot_speed_percentiles", "85th, 50th and 15th percentile spot speeds", "traffic studies",
     "Percentile speeds of a spot-speed sample (85th percentile for speed limits and design, median, 15th) by linear "
     "interpolation between ranks (position (n - 1) p / 100, as Excel PERCENTILE.INC). Speeds in km/h.",
     [_VEC("speeds", "spot speeds in km/h", ("speeds", "spot speeds"), min_length=2)],
     [_o("p85", "speed_kmh", "85th percentile speed (km/h)"), _o("p50", "speed_kmh", "median speed (km/h)"),
      _o("p15", "speed_kmh", "15th percentile speed (km/h)")],
     speed_percentiles, _ref_percentiles,
     lambda speeds: (lambda s: {"p85": s[min(len(s) - 1, math.ceil(0.85 * len(s)) - 1)] + 0.0, "p50": _r(_pct(s, 50)),
                                "p15": _r(_pct(s, 15))})(sorted(speeds)),
     lambda rng: {"speeds": [round(rng.uniform(30, 120), 1) for _ in range(rng.randint(5, 40))]},
     [{"speeds": [52, 61, 48, 70, 55, 63, 58, 66, 49, 72]}, {"speeds": [88, 95, 102, 79, 110, 92, 85]}],
     ["spot speed percentiles", "percentile speeds", "percentile spot speeds"],
     conditions=["at least two positive speeds"], invalid=[{"speeds": [52, -61, 48]}],
     method="sort; the p-th percentile at position (n - 1) p / 100 by linear interpolation",
     latex=r"v_p = v_{(i)} + f\,(v_{(i+1)} - v_{(i)}),\quad i + f = (n-1)\,p/100")


# =============================================================================================== safe following gap
def following_gap(v, t_r, a_l, a_f):
    _pos(v=v, t_r=t_r, a_l=a_l, a_f=a_f)
    Tl, Tf = v / a_l, t_r + v / a_f

    def xl(t):
        t = min(t, Tl)
        return v * t - a_l * t * t / 2

    def xf(t):
        if t <= t_r:
            return v * t
        u = min(t, Tf) - t_r
        return v * t_r + v * u - a_f * u * u / 2
    cand = [t_r, max(Tl, Tf)]
    if a_f > a_l:
        ts = a_f * t_r / (a_f - a_l)                      # equal speeds while both still move
        if t_r < ts < min(Tl, Tf):
            cand.append(ts)
    return {"gap": _r(max(xf(t) - xl(t) for t in cand))}


def _ref_following_gap(v, t_r, a_l, a_f):
    """Both vehicles simulated (ODE, each stopping at rest); the largest closing of the gap found on a time grid and
    refined by bounded minimisation (SciPy)."""
    from scipy.optimize import minimize_scalar

    def vehicle(t0, a):
        ev = lambda t, y: y[1]
        ev.terminal = True
        s = _ivp(lambda t, y: [y[1], -a], 4 * v / a + 1, [0.0, v], events=ev)
        T = float(s.t_events[0][0])
        return lambda t: (v * t if t <= t0 else float(s.sol(min(t - t0, T))[0]) + v * t0), T + t0
    xl, Tl = vehicle(0.0, a_l)
    xf, Tf = vehicle(t_r, a_f)
    d = lambda t: xf(t) - xl(t)
    end = max(Tl, Tf)
    n = 400
    grid = [end * i / n for i in range(n + 1)]
    vals = [d(t) for t in grid]
    i = max(range(n + 1), key=vals.__getitem__)
    best = vals[i]
    lo, hi = grid[max(0, i - 1)], grid[min(n, i + 1)]
    r = minimize_scalar(lambda t: -d(t), bounds=(lo, hi), method="bounded", options={"xatol": 1e-12 * end})
    return {"gap": max(best, -float(r.fun))}


proc("transport_safe_following_gap", "Minimum safe following gap when the leader brakes", "vehicle performance",
     "Smallest initial gap that avoids a rear-end collision when a leader brakes at a_l and the follower, at the same "
     "initial speed, reacts after t_r and brakes at a_f (the closest approach can happen while both still move).",
     [_V("v", "initial speed of both vehicles", ("speed",)), _T("t_r", "follower reaction time", ("reaction time",)),
      _A("a_l", "deceleration of the leader", ("leader deceleration",)),
      _A("a_f", "deceleration of the follower", ("follower deceleration",))],
     [_o("gap", "length", "minimum safe initial gap (m)")],
     following_gap, _ref_following_gap,
     lambda v, t_r, a_l, a_f: {"gap": _r(max(0.0, v * v / (2 * a_f) - v * v / (2 * a_l)))},
     lambda rng: {"v": rng.uniform(8, 35), "t_r": rng.uniform(0.5, 2.0), "a_l": rng.uniform(3, 9), "a_f": rng.uniform(3, 9)},
     [{"v": 100, "t_r": 1.0, "a_l": 8, "a_f": 6}, {"v": 80, "t_r": 1.2, "a_l": 5, "a_f": 7}],
     ["safe following gap", "minimum following distance", "safe following distance"],
     conditions=["all inputs positive"], invalid=[{"v": 100, "t_r": 1.0, "a_l": -8, "a_f": 6}],
     method="gap = max over time of (x_follower - x_leader): at the end, or when the speeds are equal",
     latex=r"g_{min} = \max_t\left[x_f(t) - x_l(t)\right]")


# =============================================================================================== stochastic queues
def _ctmc_queue(lam, mu, c, N):
    """Reference: the birth-death chain of the queue (truncated at N states, NumPy linear solve of the balance
    equations); waiting times from what arrivals see (PASTA): an arrival that finds n >= c waits (n - c + 1)/(c mu)."""
    import numpy as np
    n = np.arange(N + 1)
    Q = np.zeros((N + 1, N + 1))
    Q[n[:-1], n[:-1] + 1] = lam / mu
    Q[n[1:], n[1:] - 1] = np.minimum(n[1:], c)
    Q[n, n] = -Q.sum(axis=1)
    A = Q.T.copy()
    A[-1, :] = 1.0                                 # one balance equation replaced by the normalisation
    b = np.zeros(N + 1)
    b[-1] = 1.0
    p = np.linalg.solve(A, b)
    L = float((n * p).sum())
    Lq = float((np.maximum(n - c, 0) * p).sum())
    Wq = float((p[c:] * (n[c:] - c + 1)).sum() / (c * mu))
    W = Wq + 1 / mu
    return p, L, Lq, W, Wq


def mm1(lam, mu):
    _pos(lam=lam, mu=mu)
    _need(lam < mu, "the arrival rate must be below the service rate (rho < 1)")
    rho = lam / mu
    return {"rho": _r(rho), "L": _r(rho / (1 - rho)), "Lq": _r(rho * rho / (1 - rho)), "W": _r(1 / (mu - lam)),
            "Wq": _r(rho / (mu - lam)), "P0": _r(1 - rho)}


def _ref_mm1(lam, mu):
    p, L, Lq, W, Wq = _ctmc_queue(lam, mu, 1, int(60 / (1 - lam / mu)) + 60)
    return {"rho": 1 - float(p[0]), "L": L, "Lq": Lq, "W": W, "Wq": Wq, "P0": float(p[0])}


_QIN = [_Q("lam", "arrival rate", ("arrival rate", "arrivals")), _Q("mu", "service rate per server", ("service rate",))]

proc("transport_mm1_queue", "Single-server random queue (M/M/1), e.g. a toll booth", "queueing",
     "Poisson arrivals at lam and exponential service at mu at one server (a toll booth, a ramp meter): utilisation, "
     "mean number in the system and in the queue, mean time in the system and waiting, and the probability that the "
     "server is idle. Rates in veh/h.",
     _QIN,
     [_o("rho", "number", "utilisation lam / mu"), _o("L", "number", "mean number in the system (veh)"),
      _o("Lq", "number", "mean number waiting (veh)"), _o("W", "time", "mean time in the system (s)"),
      _o("Wq", "time", "mean waiting time (s)"), _o("P0", "number", "probability the system is empty")],
     mm1, _ref_mm1,
     lambda lam, mu: (lambda o: dict(o, W=_r(1 / mu), Wq=_r(lam / mu / mu)))(mm1(lam, mu)),
     lambda rng: (lambda mu: {"mu": mu, "lam": rng.uniform(0.1, 0.93) * mu})(rng.uniform(100, 1500) / 3600),
     [{"lam": 400, "mu": 500}, {"lam": 300, "mu": 450}],
     ["single server random queue", "single server queue", "markovian single server queue"],
     conditions=["lam < mu"], invalid=[{"lam": 600, "mu": 500}],
     method="rho = lam/mu, L = rho/(1 - rho), Lq = rho^2/(1 - rho), W = 1/(mu - lam), Wq = rho/(mu - lam)",
     latex=r"L = \frac{\rho}{1-\rho},\quad W = \frac{1}{\mu - \lambda},\quad W_q = \frac{\rho}{\mu - \lambda}")


def mmc(lam, mu, c):
    _pos(lam=lam, mu=mu)
    _need(isinstance(c, int) and 1 <= c <= 200, "the number of servers must be a whole number from 1 to 200")
    a = lam / mu
    rho = a / c
    _need(rho < 1, "the arrival rate must be below c mu (rho < 1)")
    term, s = 1.0, 1.0                          # sum_{k<c} a^k/k! with term = a^k/k!
    for k in range(1, c):
        term *= a / k
        s += term
    last = term * a / c                          # a^c / c!
    Pw = last / (1 - rho) / (s + last / (1 - rho))
    Lq = Pw * rho / (1 - rho)
    Wq = Lq / lam
    return {"P_wait": _r(Pw), "Lq": _r(Lq), "Wq": _r(Wq), "W": _r(Wq + 1 / mu), "L": _r(Lq + a)}


def _ref_mmc(lam, mu, c):
    rho = lam / (c * mu)
    N = int(c + 60 / (1 - rho) + 60)
    p, L, Lq, W, Wq = _ctmc_queue(lam, mu, c, N)
    return {"P_wait": float(p[c:].sum()), "Lq": Lq, "Wq": Wq, "W": W, "L": L}


proc("transport_mmc_queue", "Multi-server random queue (M/M/c, Erlang C), e.g. a toll plaza", "queueing",
     "Poisson arrivals at lam served by c parallel exponential servers at mu each (toll lanes, loading bays, ticket "
     "windows): probability of waiting (Erlang C), mean queue, mean wait, mean time in the system and mean number in "
     "the system. Rates in veh/h.",
     _QIN + [_in("c", "number", "number", "number of servers (whole number)", ("servers", "number of servers"),
                 typ="integer", min=1)],
     [_o("P_wait", "number", "probability an arrival waits (Erlang C)"), _o("Lq", "number", "mean number waiting (veh)"),
      _o("Wq", "time", "mean waiting time (s)"), _o("W", "time", "mean time in the system (s)"),
      _o("L", "number", "mean number in the system (veh)")],
     mmc, _ref_mmc,
     lambda lam, mu, c: (lambda o: dict(o, Wq=_r(o["Wq"] * c), W=_r(o["Wq"] * c + 1 / mu)))(mmc(lam, mu, c)),
     lambda rng: (lambda c, mu: {"c": c, "mu": mu, "lam": rng.uniform(0.45, 0.92) * c * mu})(rng.randint(1, 8),
                                                                                         rng.uniform(100, 600) / 3600),
     [{"lam": 1500, "mu": 400, "c": 5}, {"lam": 900, "mu": 350, "c": 3}],
     ["multi server random queue", "erlang c queue", "multi server queue", "toll plaza queue"],
     conditions=["lam < c mu", "1 <= c <= 200"], invalid=[{"lam": 2500, "mu": 400, "c": 5}],
     method="Erlang C: P_wait = (a^c/c!)/(1 - rho) / (sum_{k<c} a^k/k! + (a^c/c!)/(1 - rho)); Lq = P_wait rho/(1 - rho)",
     latex=r"P_W = \frac{\frac{a^c}{c!}\frac{1}{1-\rho}}{\sum_{k=0}^{c-1}\frac{a^k}{k!} + \frac{a^c}{c!}\frac{1}{1-\rho}},\quad L_q = P_W\frac{\rho}{1-\rho}")


def md1(lam, mu):
    _pos(lam=lam, mu=mu)
    _need(lam < mu, "the arrival rate must be below the service rate (rho < 1)")
    rho = lam / mu
    Lq = rho * rho / (2 * (1 - rho))
    return {"Lq": _r(Lq), "Wq": _r(rho / (2 * mu * (1 - rho))), "W": _r((2 - rho) / (2 * mu * (1 - rho))),
            "L": _r(Lq + rho)}


def _ref_md1(lam, mu):
    """Reference: the queue left behind by departing vehicles as an embedded Markov chain (Poisson arrivals during one
    constant service time), solved by NumPy; by the Poisson-arrival property the distribution at departures is the
    time-average one, and the times follow from Little's law."""
    import numpy as np
    from scipy.stats import poisson
    rho = lam / mu
    N = int(100 + 60 / (1 - rho))
    a = poisson.pmf(np.arange(N + 2), rho)
    P = np.zeros((N + 1, N + 1))
    for i in range(N + 1):
        base = max(i - 1, 0)
        P[i, base:] = a[:N + 1 - base]
        P[i, N] += 1.0 - P[i].sum()
    A = P.T - np.eye(N + 1)
    A[-1, :] = 1.0
    b = np.zeros(N + 1)
    b[-1] = 1.0
    pi = np.linalg.solve(A, b)
    L = float((np.arange(N + 1) * pi).sum())
    Lq = L - (1 - float(pi[0]))
    return {"Lq": Lq, "Wq": Lq / lam, "W": L / lam, "L": L}


proc("transport_md1_queue", "Single server with constant service time (M/D/1)", "queueing",
     "Poisson arrivals at lam and a constant service time 1/mu (an automated toll gate, a ramp meter): mean queue, "
     "mean wait, mean time in the system and mean number in the system. Rates in veh/h.",
     _QIN,
     [_o("Lq", "number", "mean number waiting (veh)"), _o("Wq", "time", "mean waiting time (s)"),
      _o("W", "time", "mean time in the system (s)"), _o("L", "number", "mean number in the system (veh)")],
     md1, _ref_md1,
     lambda lam, mu: (lambda o: dict(o, Lq=_r(2 * o["Lq"]), L=_r(2 * o["Lq"] + lam / mu)))(md1(lam, mu)),
     lambda rng: (lambda mu: {"mu": mu, "lam": rng.uniform(0.1, 0.88) * mu})(rng.uniform(100, 1500) / 3600),
     [{"lam": 400, "mu": 500}, {"lam": 1200, "mu": 1800}],
     ["constant service time queue", "deterministic service time queue"],
     conditions=["lam < mu"], invalid=[{"lam": 600, "mu": 500}],
     method="Lq = rho^2 / (2(1 - rho)), Wq = rho / (2 mu (1 - rho)), W = (2 - rho)/(2 mu (1 - rho))",
     latex=r"L_q = \frac{\rho^2}{2(1-\rho)},\quad W_q = \frac{\rho}{2\mu(1-\rho)}")


def erlang_b(lam, mu, c):
    _pos(lam=lam, mu=mu)
    _need(isinstance(c, int) and 1 <= c <= 1000, "the number of servers must be a whole number from 1 to 1000")
    a = lam / mu
    B = 1.0
    for k in range(1, c + 1):
        B = a * B / (k + a * B)
    return {"P_block": _r(B), "carried": _r(a * (1 - B)), "lost_per_h": _vph(lam * B)}


def _ref_erlang_b(lam, mu, c):
    p, *_ = _ctmc_queue(lam, mu, c, c)
    B = float(p[c])
    return {"P_block": B, "carried": float(sum(n * p[n] for n in range(c + 1))), "lost_per_h": lam * B * 3600}


proc("transport_erlang_b_blocking", "Blocking probability with no waiting room (Erlang B), e.g. parking spaces",
     "queueing",
     "Arrivals at lam (veh/h) to c spaces or servers held for a mean time 1/mu with no waiting room (a car park, "
     "loading bays, taxi ranks): the probability an arrival is turned away (Erlang B), the mean number of occupied "
     "spaces and the vehicles lost per hour. Rates in veh/h.",
     _QIN + [_in("c", "number", "number", "number of spaces or servers (whole number)", ("spaces", "servers"),
                 typ="integer", min=1)],
     [_o("P_block", "number", "probability an arrival is turned away"), _o("carried", "number", "mean number of occupied spaces"),
      _o("lost_per_h", "traffic_flow", "vehicles turned away (veh/h)")],
     erlang_b, _ref_erlang_b,
     lambda lam, mu, c: (lambda a: (lambda B: {"P_block": _r(B), "carried": _r(a * (1 - B)), "lost_per_h": _vph(lam * B)})(
         a ** c / math.factorial(c)))(lam / mu),
     lambda rng: (lambda c, mu: {"c": c, "mu": mu, "lam": rng.uniform(0.75, 1.3) * c * mu})(rng.randint(1, 60),
                                                                                          rng.uniform(0.5, 6) / 3600),
     [{"lam": 60, "mu": 0.5, "c": 120}, {"lam": 40, "mu": 4, "c": 12}],
     ["erlang b blocking", "erlang b", "blocking probability"],
     conditions=["1 <= c <= 1000"], invalid=[{"lam": 60, "mu": -0.5, "c": 120}],
     method="Erlang B recursion B(k) = a B(k-1) / (k + a B(k-1)), a = lam/mu", latex=r"B(c, a) = \frac{a^c/c!}{\sum_{k=0}^{c} a^k/k!}")


# =============================================================================================== deterministic FIFO queue
def fifo_queue(arrivals, service, c=1):
    t = _vec("arrivals", arrivals, nonneg=True)
    s = _vec("service", service, positive=True)
    _need(isinstance(c, int) and c >= 1, "the number of servers must be a whole number >= 1")
    if len(s) == 1:
        s = s * len(t)
    _need(len(s) == len(t), "give one service time for every arrival (or one for all)")
    _need(all(b >= a for a, b in zip(t, t[1:])), "arrival times must be in increasing order")
    free = [t[0]] * c                              # Kiefer-Wolfowitz: the times the c servers become free, sorted
    waits, dep = [], []
    for a, x in zip(t, s):
        start = max(a, free[0])
        waits.append(start - a)
        dep.append(start + x)
        free[0] = start + x
        free.sort()
    events = sorted([(a, 1) for a, w in zip(t, waits) if w > 0] + [(a + w, -1) for a, w in zip(t, waits) if w > 0],
                    key=lambda e: (e[0], e[1]))
    q = qmax = 0
    for _, d in events:
        q += d
        qmax = max(qmax, q)
    n = len(t)
    return {"waits": _r(waits), "mean_wait": _r(math.fsum(waits) / n), "max_wait": _r(max(waits)),
            "mean_time_in_system": _r(math.fsum(w + x for w, x in zip(waits, s)) / n), "last_departure": _r(max(dep)),
            "max_queue": float(qmax)}


def _ref_fifo(arrivals, service, c=1):
    """Reference: an event-driven simulation (a heap of arrival and departure events, a first-in-first-out waiting
    line and a count of busy servers), recording every customer's start of service and the longest waiting line."""
    from collections import deque
    t = list(arrivals)
    s = list(service) * (len(t) if len(service) == 1 else 1)
    ev = [(a, 1, i) for i, a in enumerate(t)]       # departures (kind 0) are handled before arrivals at the same time
    heapq.heapify(ev)
    line, busy, start, qmax = deque(), 0, [None] * len(t), 0
    while ev:
        time_, kind, i = heapq.heappop(ev)
        if kind == 0:
            busy -= 1
            if line:
                j = line.popleft()
                start[j] = time_
                busy += 1
                heapq.heappush(ev, (time_ + s[j], 0, j))
        elif busy < c and not line:
            start[i] = time_
            busy += 1
            heapq.heappush(ev, (time_ + s[i], 0, i))
        else:
            line.append(i)
            qmax = max(qmax, len(line))
    waits = [b - a for a, b in zip(t, start)]
    n = len(t)
    return {"waits": waits, "mean_wait": sum(waits) / n, "max_wait": max(waits),
            "mean_time_in_system": sum(w + x for w, x in zip(waits, s)) / n,
            "last_departure": max(b + x for b, x in zip(start, s)), "max_queue": float(qmax)}


proc("transport_fifo_queue_simulation", "First-come-first-served queue from arrival and service times",
     "queueing",
     "Waiting time of every vehicle (or passenger) at c identical servers in first-come-first-served order from "
     "given arrival times and service times (a toll plaza, a bus stop with c berths): mean and longest wait, mean "
     "time in the system, when the last one leaves and the longest waiting line. Times in s.",
     [_VEC("arrivals", "arrival times in s, increasing", ("arrivals", "arrival times")),
      _VEC("service", "service times in s (one per arrival, or one for all)", ("service", "service times")),
      _in("c", "number", "number", "number of servers (default 1)", ("servers",), typ="integer", min=1, required=False)],
     [_o("waits", "number", "waiting time of each arrival (s)"), _o("mean_wait", "time", "mean waiting time (s)"),
      _o("max_wait", "time", "longest wait (s)"), _o("mean_time_in_system", "time", "mean time in the system (s)"),
      _o("last_departure", "time", "time the last one leaves (s)"), _o("max_queue", "number", "longest waiting line")],
     fifo_queue, _ref_fifo,
     lambda arrivals, service, c=1: (lambda o: dict(o, mean_wait=_r(o["mean_wait"] + 1.0),
                                                    waits=_r([w + 1.0 for w in o["waits"]])))(fifo_queue(arrivals, service, c)),
     lambda rng: (lambda n, c: (lambda gaps: {"arrivals": [round(sum(gaps[:i + 1]), 3) for i in range(n)],
                                              "service": [round(rng.uniform(2, 14 * c), 3) for _ in range(n)], "c": c})(
         [rng.uniform(0.5, 10) for _ in range(n)]))(rng.randint(3, 40), rng.randint(1, 4)),
     [{"arrivals": [0, 5, 7, 12, 14, 20, 21], "service": [8, 6, 9, 4, 7, 5, 6]},
      {"arrivals": [0, 2, 3, 5, 9, 10, 11, 15], "service": [12], "c": 2}],
     ["fifo queue simulation", "first come first served queue", "queue simulation from arrival times"],
     conditions=["arrival times increasing", "service times > 0", "c >= 1"],
     invalid=[{"arrivals": [5, 0, 7], "service": [8, 6, 9]}],
     method="each arrival starts at max(arrival, earliest free server) (Kiefer-Wolfowitz recursion)",
     latex=r"S_n = \max(A_n,\ \min_j F_j),\quad W_n = S_n - A_n")


# =============================================================================================== railway operations
def running_time(D, v_max, a, b):
    _pos(D=D, v_max=v_max, a=a, b=b)
    da, db = v_max * v_max / (2 * a), v_max * v_max / (2 * b)
    if da + db <= D:
        return {"T": _r(D / v_max + v_max / (2 * a) + v_max / (2 * b)), "v_peak": _r(v_max),
                "profile": "accelerate-cruise-brake"}
    vp = math.sqrt(2 * D * a * b / (a + b))
    return {"T": _r(vp / a + vp / b), "v_peak": _r(vp), "profile": "accelerate-brake"}


def _ref_running_time(D, v_max, a, b):
    """The fastest speed profile v(x) = min(v_max, sqrt(2 a x), sqrt(2 b (D - x))) and the running time as the
    integral of dx / v(x) (SciPy quadrature; the 1/sqrt end singularities are integrated with algebraic weights);
    the switching points are found by brentq."""
    from scipy.integrate import quad
    va = lambda x: math.sqrt(2 * a * max(x, 0.0))
    vb = lambda x: math.sqrt(2 * b * max(D - x, 0.0))
    x1 = _brentq(lambda x: va(x) - min(v_max, vb(x)), 0.0, D)
    x2 = _brentq(lambda x: min(v_max, va(x)) - vb(x), 0.0, D)
    kw = dict(epsabs=0.0, epsrel=1e-13, limit=200)
    T1 = quad(lambda x: 1 / math.sqrt(2 * a), 0.0, x1, weight="alg", wvar=(-0.5, 0.0), **kw)[0]
    T3 = quad(lambda x: 1 / math.sqrt(2 * b), x2, D, weight="alg", wvar=(0.0, -0.5), **kw)[0]
    T2 = (x2 - x1) / v_max if x2 > x1 else 0.0
    vp = max(va(x1), vb(x2))
    return {"T": T1 + T2 + T3, "v_peak": min(vp, v_max),
            "profile": "accelerate-cruise-brake" if x2 - x1 > 1e-12 * D else "accelerate-brake"}


proc("transport_train_running_time", "Running time between stations (accelerate, cruise, brake)", "railway operations",
     "Minimum running time of a train (or bus) between two stops a distance D apart with acceleration a, braking rate b "
     "and line speed v_max; when the stops are too close to reach v_max the profile is accelerate-brake.",
     [_L("D", "distance between stops", ("distance", "station spacing", "stop spacing"), show="km"),
      _V("v_max", "maximum (line) speed", ("line speed", "maximum speed")), _A("a", "acceleration", ("acceleration",)),
      _A("b", "braking rate", ("braking rate", "deceleration"))],
     [_o("T", "time", "running time stop to stop (s)"), _o("v_peak", "speed", "highest speed reached (m/s)"),
      _o("profile", "text", "accelerate-cruise-brake or accelerate-brake")],
     running_time, _ref_running_time,
     lambda D, v_max, a, b: {"T": _r(D / v_max + v_max / a + v_max / b), "v_peak": _r(v_max), "profile": "accelerate-cruise-brake"},
     lambda rng: {"D": rng.uniform(300, 5000), "v_max": rng.uniform(10, 45), "a": rng.uniform(0.4, 1.3), "b": rng.uniform(0.5, 1.3)},
     [{"D": 1.5, "v_max": 80, "a": 1.0, "b": 1.2}, {"D": 0.6, "v_max": 100, "a": 0.8, "b": 1.0}],
     ["train running time", "running time between stations", "station to station running time"],
     conditions=["all inputs positive"], invalid=[{"D": -1.5, "v_max": 80, "a": 1.0, "b": 1.2}],
     method="if v^2/(2a) + v^2/(2b) <= D: T = D/v + v/(2a) + v/(2b); else v_p = sqrt(2 D a b / (a + b)), T = v_p/a + v_p/b",
     latex=r"T = \frac{D}{v} + \frac{v}{2a} + \frac{v}{2b}")


def station_headway(v, a, b, L_t, t_d, s_m=0.0):
    _pos(v=v, a=a, b=b, L_t=L_t)
    _nonneg(t_d=t_d, s_m=s_m)
    s = L_t + s_m
    tc = math.sqrt(2 * s / a) if s <= v * v / (2 * a) else v / (2 * a) + s / v
    h = t_d + tc + v / b
    return {"h": _r(h), "trains_per_h": _r(3600 / h)}


def _ref_station_headway(v, a, b, L_t, t_d, s_m=0.0):
    """The departing train is simulated (ODE: accelerate at a to the line speed, then cruise) until its rear has
    cleared the platform end plus the margin; the follower runs on its braking curve and stops at the platform, so the
    headway is clearance + braking time + dwell. The resulting schedule is checked against the moving-block (brick-wall)
    rule on a fine time grid: the point where the follower could stop never passes the leader's rear minus the margin."""
    s = L_t + s_m
    ev = lambda t, y: y[1] - v
    ev.terminal = True
    s1 = _ivp(lambda t, y: [y[1], a], 4 * v / a + 1, [0.0, 0.0], events=ev)
    ta = float(s1.t_events[0][0])
    xa = float(s1.y_events[0][0][0])
    xl = lambda t: 0.0 if t <= 0 else (float(s1.sol(t)[0]) if t <= ta else xa + v * (t - ta))
    if xl(ta) >= s:
        tc = _brentq(lambda t: xl(t) - s, 0.0, ta)
    else:
        s2 = _ivp(lambda t, y: [v], s / v + 1, [xa])
        tc = ta + _first_crossing(s2, 0, s)
    h = t_d + tc + v / b
    tb = h - t_d - v / b                                  # the follower starts braking
    stop_point = lambda t: v * (t - tb) if t < tb else 0.0
    worst = min(xl(t) - s - stop_point(t) for t in [tb - 3 * h + 3 * h * i / 2000 for i in range(2001)])
    if worst < -1e-9 * s:
        raise AssertionError("the schedule breaks the brick-wall rule")
    return {"h": h, "trains_per_h": 3600 / h}


proc("transport_rail_station_headway", "Minimum headway of trains at a station stop (moving block)", "railway operations",
     "Shortest headway between trains serving a station under a brick-wall (moving-block) separation: the leader dwells "
     "t_d, accelerates at a until its rear clears the platform plus the safety margin s_m, while the follower approaches "
     "at the line speed v and brakes at b to stop at the platform; and the corresponding line capacity in trains per hour.",
     [_V("v", "line (approach) speed", ("line speed", "approach speed")), _A("a", "acceleration", ("acceleration",)),
      _A("b", "braking rate", ("braking rate",)), _L("L_t", "train length", ("train length",)),
      _T("t_d", "dwell time", ("dwell time",)), _L("s_m", "safety margin (default 0)", ("safety margin",), required=False)],
     [_o("h", "time", "minimum headway (s)"), _o("trains_per_h", "service_frequency", "line capacity (trains/h)")],
     station_headway, _ref_station_headway,
     lambda v, a, b, L_t, t_d, s_m=0.0: (lambda h: {"h": _r(h), "trains_per_h": _r(3600 / h)})(
         t_d + math.sqrt(2 * (L_t + s_m) / a) + v / (2 * b)),
     lambda rng: {"v": rng.uniform(10, 30), "a": rng.uniform(0.6, 1.3), "b": rng.uniform(0.7, 1.3),
                  "L_t": rng.uniform(40, 220), "t_d": rng.uniform(15, 60), "s_m": rng.uniform(0, 100)},
     [{"v": 80, "a": 1.0, "b": 1.1, "L_t": 150, "t_d": 30, "s_m": 50}, {"v": 60, "a": 1.2, "b": 1.0, "L_t": 75, "t_d": 20}],
     ["station headway", "minimum train headway", "rail station headway"],
     conditions=["v, a, b, L_t > 0", "t_d, s_m >= 0"], invalid=[{"v": 80, "a": -1.0, "b": 1.1, "L_t": 150, "t_d": 30}],
     method="h = t_d + t_clear + v/b, t_clear = sqrt(2 (L + s)/a) (or v/(2a) + (L + s)/v once the line speed is reached)",
     latex=r"h = t_d + \sqrt{\frac{2(L_t + s_m)}{a}} + \frac{v}{b}")


def _davis(Ad, Bd, Cd):
    _nonneg(A=Ad, B=Bd, C=Cd)
    return lambda v: Ad + Bd * v + Cd * v * v


def train_braking(m, v1, F_b, A, B, C, G=0.0, gamma=1.06, v2=0.0):
    _pos(m=m, v1=v1, F_b=F_b, gamma=gamma)
    _nonneg(v2=v2)
    _finite(G=G)
    _need(v2 < v1, "the final speed must be below the initial speed")
    R = _davis(A, B, C)
    gr = m * G_ * math.sin(math.atan(G))
    ret = lambda v: F_b + R(v) + gr
    _need(ret(v2) > 0, "the train does not slow down on this downgrade with this braking force")   # ret grows with v
    import mpmath
    with mpmath.workdps(25):
        S = mpmath.quad(lambda v: gamma * m * v / ret(v), [v2, v1])
        t = mpmath.quad(lambda v: gamma * m / ret(v), [v2, v1])
    return {"S": _r(float(S)), "t": _r(float(t))}


def _ref_train_braking(m, v1, F_b, A, B, C, G=0.0, gamma=1.06, v2=0.0):
    gr = m * 9.81 * math.sin(math.atan(G))
    ev = lambda t, y: y[1] - v2
    ev.terminal = True
    sol = _ivp(lambda t, y: [y[1], -(F_b + A + B * y[1] + C * y[1] * y[1] + gr) / (gamma * m)], 1e5, [0.0, v1], events=ev)
    return {"S": float(sol.y_events[0][0][0]), "t": float(sol.t_events[0][0])}


_DAVIS_IN = [_F("A", "Davis coefficient A (constant resistance)", ("davis a",), show="kN"),
             _in("B", "davis_b", "davis_b", "Davis coefficient B (per speed)", ("davis b",), "kN/(km/h)"),
             _in("C", "davis_c", "davis_c", "Davis coefficient C (per speed squared)", ("davis c",), "kN/(km/h)^2")]


def _rand_davis(rng, m):
    t = m / 1000
    return {"A": rng.uniform(5, 15) * t, "B": rng.uniform(0.02, 0.15) * t * 3.6, "C": rng.uniform(0.4, 1.2) * 12.96}


proc("transport_train_braking_distance", "Train braking distance with Davis resistance and gradient", "railway operations",
     "Braking distance and time of a train of mass m braking with a constant brake force F_b from v1 to v2, helped by "
     "the Davis train resistance R = A + B v + C v^2 and the gradient (positive uphill), with a rotating-mass factor gamma.",
     [_M("m", "train mass", ("mass", "train mass"), show="t"), _V("v1", "initial speed", ("initial speed", "speed")),
      _F("F_b", "brake force", ("brake force", "braking force"), show="kN")] + _DAVIS_IN +
     [_GR(show="‰"), _N("gamma", "rotating-mass factor (default 1.06)", ("mass factor",), required=False),
      _V("v2", "final speed (default 0)", ("final speed",), required=False)],
     [_o("S", "length", "braking distance (m)"), _o("t", "time", "braking time (s)")],
     train_braking, _ref_train_braking,
     lambda m, v1, F_b, A, B, C, G=0.0, gamma=1.06, v2=0.0: train_braking(m, v1, F_b, A, B, C, G, 1.0, v2),
     lambda rng: (lambda m: dict(_rand_davis(rng, m), m=m, v1=rng.uniform(15, 50), F_b=m * rng.uniform(0.4, 1.0),
                                 G=rng.uniform(-0.02, 0.02), gamma=rng.uniform(1.04, 1.1),
                                 v2=rng.choice([0.0, rng.uniform(0, 10)])))(rng.uniform(100e3, 2e6)),
     [{"m": 400, "v1": 120, "F_b": 320, "A": 6, "B": 0.05, "C": 0.0008}, {"m": 1200, "v1": 90, "F_b": 600, "A": 14,
                                                                        "B": 0.1, "C": 0.0012, "G": -8, "v2": 30}],
     ["train braking distance", "braking distance of a train"],
     conditions=["F_b + R(v) + grade force > 0 over the speed range", "v2 < v1"],
     invalid=[{"m": 400, "v1": 120, "F_b": -320, "A": 6, "B": 0.05, "C": 0.0008}],
     method="S = integral from v2 to v1 of gamma m v dv / (F_b + A + B v + C v^2 + m g sin(theta)) (quadrature)",
     latex=r"S = \int_{v_2}^{v_1}\frac{\gamma m\,v\,dv}{F_b + A + Bv + Cv^2 + mg\sin\theta}")


def train_accel(m, v1, v2, F_max, P, A, B, C, G=0.0, gamma=1.08):
    _pos(m=m, v2=v2, F_max=F_max, P=P, gamma=gamma)
    _nonneg(v1=v1)
    _finite(G=G)
    _need(v2 > v1, "the final speed must exceed the initial speed")
    R = _davis(A, B, C)
    gr = m * G_ * math.sin(math.atan(G))
    vc = P / F_max
    net = lambda v: (F_max if v <= vc else P / v) - R(v) - gr
    _need(net(v2) > 1e-9 * F_max and net(v1) > 0, "the tractive effort does not exceed the resistance up to the final speed")
    import mpmath
    pts = [v1] + ([vc] if v1 < vc < v2 else []) + [v2]
    with mpmath.workdps(25):
        t = mpmath.quad(lambda v: gamma * m / net(v), pts)
        d = mpmath.quad(lambda v: gamma * m * v / net(v), pts)
    return {"t": _r(float(t)), "d": _r(float(d))}


def _ref_train_accel(m, v1, v2, F_max, P, A, B, C, G=0.0, gamma=1.08):
    """Two simulated phases (ODE): constant tractive effort up to the speed P / F_max, then constant power."""
    gr = m * 9.81 * math.sin(math.atan(G))
    vc = P / F_max
    t0, x0, v = 0.0, 0.0, v1
    for phase in ("effort", "power"):
        target = min(vc, v2) if phase == "effort" else v2
        if v >= target:
            continue
        Ft = (lambda u: F_max) if phase == "effort" else (lambda u: P / u)
        ev = lambda t, y, tg=target: y[1] - tg
        ev.terminal = True
        sol = _ivp(lambda t, y: [y[1], (Ft(y[1]) - A - B * y[1] - C * y[1] * y[1] - gr) / (gamma * m)], 1e6, [0.0, v],
                   events=ev)
        t0 += float(sol.t_events[0][0])
        x0 += float(sol.y_events[0][0][0])
        v = target
    return {"t": t0, "d": x0}


proc("transport_train_acceleration", "Train acceleration time and distance (tractive effort curve, Davis resistance)",
     "railway operations",
     "Time and distance for a train to accelerate from v1 to v2 with a tractive effort F_max up to the speed P / F_max "
     "and constant power P above it, against the Davis resistance A + B v + C v^2 and the gradient, with a "
     "rotating-mass factor gamma.",
     [_M("m", "train mass", ("mass", "train mass"), show="t"), _V("v1", "initial speed", ("initial speed",)),
      _V("v2", "final speed", ("final speed",)), _F("F_max", "maximum (starting) tractive effort", ("tractive effort",), show="kN"),
      _P("P", "power at the wheel rim", ("power",))] + _DAVIS_IN +
     [_GR(show="‰"), _N("gamma", "rotating-mass factor (default 1.08)", ("mass factor",), required=False)],
     [_o("t", "time", "acceleration time (s)"), _o("d", "length", "distance travelled (m)")],
     train_accel, _ref_train_accel,
     lambda m, v1, v2, F_max, P, A, B, C, G=0.0, gamma=1.08: (lambda o: dict(o, t=_r(o["t"] * 0.9)))(
         train_accel(m, v1, v2, F_max, P, A, B, C, G, gamma)),
     lambda rng: (lambda m: (lambda F, P: dict(_rand_davis(rng, m), m=m, F_max=F, P=P, v1=rng.uniform(0, 8),
                                               v2=rng.uniform(12, 30), G=rng.uniform(-0.01, 0.01),
                                               gamma=rng.uniform(1.04, 1.12)))(
         m * rng.uniform(0.15, 0.4), m * rng.uniform(3, 8)))(rng.uniform(100e3, 1.5e6)),
     [{"m": 400, "v1": 0, "v2": 100, "F_max": 150, "P": 4000, "A": 6, "B": 0.05, "C": 0.0008},
      {"m": 1500, "v1": 20, "v2": 80, "F_max": 400, "P": 6000, "A": 18, "B": 0.12, "C": 0.0012, "G": 5}],
     ["train acceleration", "train acceleration time", "time for a train to accelerate"],
     conditions=["tractive effort exceeds resistance up to v2", "v2 > v1 >= 0"],
     invalid=[{"m": 400, "v1": 100, "v2": 0, "F_max": 150, "P": 4000, "A": 6, "B": 0.05, "C": 0.0008}],
     method="t = integral of gamma m dv / (F(v) - R(v) - m g sin(theta)), F(v) = min(F_max, P/v) (quadrature)",
     latex=r"t = \int_{v_1}^{v_2}\frac{\gamma m\,dv}{\min(F_{max}, P/v) - (A + Bv + Cv^2) - mg\sin\theta}")


# =============================================================================================== road networks
def _graph(roads, speeds, directed):
    _need(isinstance(roads, list) and roads and all(len(e) == 3 for e in roads),
          "give every road with its length: 'A-B 2.5' (km)")
    sp = _vec("speeds", speeds, positive=True)
    _need(len(sp) == len(roads), "give one speed for every road, in the same order")
    adj = {}
    for (u, v, L), s in zip(roads, sp):
        _need(L > 0, "road lengths must be positive")
        _need(u != v, "a road must join two different places")
        t = 60.0 * L / s
        for a, b in ((u, v),) + (() if directed else ((v, u),)):
            adj.setdefault(a, {})
            adj.setdefault(b, {})
            if b not in adj[a] or t < adj[a][b][0]:
                adj[a][b] = (t, L)
    return adj


def _dijkstra(adj, src):
    """Fastest times from src; ties broken by fewer links, then the alphabetically first node sequence."""
    best = {src: (0.0, 0, [src], 0.0)}
    heap = [(0.0, 0, [src], 0.0)]
    done = set()
    while heap:
        t, n, path, dist = heapq.heappop(heap)
        u = path[-1]
        if u in done:
            continue
        done.add(u)
        for v, (w, L) in adj[u].items():
            cand = (t + w, n + 1, path + [v], dist + L)
            if v not in best or cand[:3] < best[v][:3]:
                best[v] = cand
                heapq.heappush(heap, cand)
    return best


def _truthy(x):
    return x if isinstance(x, bool) else str(x).strip().lower() in ("true", "yes", "1")


def fastest_route(roads, speeds, origin, destination, directed=False):
    directed = _truthy(directed)
    adj = _graph(roads, speeds, directed)
    _need(origin in adj and destination in adj, "the origin and the destination must be places on the network")
    _need(origin != destination, "the origin and the destination must differ")
    best = _dijkstra(adj, origin)
    _need(destination in best, "the destination cannot be reached from the origin")
    t, _n, path, dist = best[destination]
    return {"route": "-".join(path), "time": _r(t), "distance": _r(dist)}


def _nx_graph(roads, speeds, directed):
    import networkx as nx
    g = nx.DiGraph() if _truthy(directed) else nx.Graph()
    for (u, v, L), s in zip(roads, speeds):
        t = 60.0 * L / s
        if not g.has_edge(u, v) or t < g[u][v]["time"]:
            g.add_edge(u, v, time=t, length=L)
    return g


def _ref_route(roads, speeds, origin, destination, directed=False):
    import networkx as nx
    g = _nx_graph(roads, speeds, directed)
    path = nx.dijkstra_path(g, origin, destination, weight="time")
    return {"route": "-".join(path), "time": nx.path_weight(g, path, "time"), "distance": nx.path_weight(g, path, "length")}


def _rand_network(rng, n_min=4, n_max=9):
    n = rng.randint(n_min, n_max)
    names = [chr(65 + i) for i in range(n)]
    pairs = set()
    order = names[:]
    rng.shuffle(order)
    for i in range(1, n):                              # a random spanning tree keeps the network connected
        pairs.add(tuple(sorted((order[i], order[rng.randrange(i)]))))
    while len(pairs) < min(n * (n - 1) // 2, n + rng.randint(1, n)):
        a, b = rng.sample(names, 2)
        pairs.add(tuple(sorted((a, b))))
    pairs = sorted(pairs)
    roads = ", ".join(f"{a}-{b} {rng.uniform(0.5, 20):.3f}" for a, b in pairs)
    speeds = [round(rng.uniform(20, 120), 1) for _ in pairs]
    return names, roads, speeds


def _rand_route(rng):
    names, roads, speeds = _rand_network(rng)
    o, d = rng.sample(names, 2)
    return {"roads": roads, "speeds": speeds, "origin": o, "destination": d}


_NET_IN = [_in("roads", "number", "number", "roads as 'A-B length' with lengths in km", ("roads", "network", "links"),
               typ="edges", min_length=1),
           _VEC("speeds", "speed on each road in km/h, in the same order", ("speeds",))]
_NET1 = "A-B 4, A-C 2.5, B-C 1.2, B-D 5, C-D 8, C-E 10, D-E 2"
_NET1_SPEEDS = [60, 50, 40, 80, 100, 90, 50]

proc("transport_fastest_route", "Fastest route on a road network (link lengths and speeds)", "route planning",
     "The quickest route between two places on a road network whose links are given by length (km) and speed (km/h): "
     "the route, its travel time in minutes and its length in km. Roads are two-way unless directed = yes.",
     _NET_IN + [_in("origin", "number", "number", "starting place", ("origin", "from"), typ="text"),
                _in("destination", "number", "number", "destination", ("destination", "to"), typ="text"),
                _in("directed", "number", "number", "one-way links as written (default no)", ("directed", "one way"),
                    typ="boolean", required=False)],
     [_o("route", "text", "fastest route (places joined by '-')"), _o("time", "time_min", "travel time (min)"),
      _o("distance", "length_km", "route length (km)")],
     fastest_route, _ref_route,
     lambda roads, speeds, origin, destination, directed=False: (lambda o: dict(o, time=_r(o["time"] / 60)))(
         fastest_route(roads, speeds, origin, destination, directed)),
     _rand_route,
     [{"roads": _NET1, "speeds": _NET1_SPEEDS, "origin": "A", "destination": "E"},
      {"roads": "P-Q 3, Q-R 4, P-R 9, R-S 2, Q-S 7", "speeds": [30, 60, 100, 50, 40], "origin": "P", "destination": "S",
       "directed": "yes"}],
     ["fastest route", "quickest route", "shortest travel time route"],
     conditions=["one positive length and speed per road", "the destination is reachable"],
     invalid=[{"roads": _NET1, "speeds": _NET1_SPEEDS, "origin": "A", "destination": "Z"}],
     method="Dijkstra's algorithm on link travel times 60 L / v (min)", latex=r"t_{link} = \frac{60\,L}{v},\quad \min \sum t_{link}")


def skim(roads, speeds, directed=False):
    directed = _truthy(directed)
    adj = _graph(roads, speeds, directed)
    nodes = sorted(adj)
    _need(len(nodes) <= 60, "at most 60 places")
    mat = []
    for o in nodes:
        best = _dijkstra(adj, o)
        _need(len(best) == len(nodes), "every place must be reachable from every other")
        mat.append([_r(best[d][0]) for d in nodes])
    return {"places": ", ".join(nodes), "times": mat}


def _ref_skim(roads, speeds, directed=False):
    import networkx as nx
    g = _nx_graph(roads, speeds, directed)
    nodes = sorted(g.nodes)
    d = dict(nx.all_pairs_dijkstra_path_length(g, weight="time"))
    return {"places": ", ".join(nodes), "times": [[float(d[o][t]) for t in nodes] for o in nodes]}


proc("transport_travel_time_skim", "Zone-to-zone travel time matrix (skim) of a road network", "route planning",
     "The fastest travel time in minutes between every pair of places of a road network given by link lengths (km) and "
     "speeds (km/h), as a matrix in alphabetical order of the places (the skim used for trip distribution).",
     _NET_IN + [_in("directed", "number", "number", "one-way links as written (default no)", ("directed", "one way"),
                    typ="boolean", required=False)],
     [_o("places", "text", "places in the order of the matrix"), _o("times", "number", "travel time matrix (min)")],
     skim, _ref_skim,
     lambda roads, speeds, directed=False: (lambda o: dict(o, times=[[_r(x * 1.1) for x in row] for row in o["times"]]))(
         skim(roads, speeds, directed)),
     lambda rng: (lambda net: {"roads": net[1], "speeds": net[2]})(_rand_network(rng, 3, 8)),
     [{"roads": _NET1, "speeds": _NET1_SPEEDS}, {"roads": "P-Q 3, Q-R 4, P-R 9, R-S 2, Q-S 7", "speeds": [30, 60, 100, 50, 40]}],
     ["travel time skim", "travel time matrix", "skim matrix"],
     conditions=["every place reachable", "at most 60 places"], invalid=[{"roads": "A-B 4, C-D 3", "speeds": [60, 50]}],
     method="Dijkstra from every place", latex=r"t_{ij} = \min_{paths\ i \to j} \sum t_{link}")


def aon(network, od):
    _need(isinstance(network, list) and network and all(len(e) == 3 for e in network),
          "give every link with its travel time: 'A-B 4' (min)")
    _need(isinstance(od, list) and od and all(len(e) == 3 for e in od), "give every trip pair with its demand: 'A-D 500'")
    adj, seen = {}, set()
    for u, v, t in network:
        _need(t > 0, "travel times must be positive")
        _need((u, v) not in seen and u != v, "each one-way link may be listed once")
        seen.add((u, v))
        adj.setdefault(u, {})[v] = (t, 0.0)
        adj.setdefault(v, {})
    flows = {(u, v): 0.0 for u, v, _t in network}
    for o, d, q in od:
        _need(q >= 0, "demands must not be negative")
        _need(o in adj and d in adj and o != d, "every trip pair must join two different places of the network")
        best = _dijkstra(adj, o)
        _need(d in best, f"{d} cannot be reached from {o}")
        path = best[d][2]
        for a, b in zip(path, path[1:]):
            flows[(a, b)] += q
    f = [flows[(u, v)] for u, v, _t in network]
    vht = math.fsum(x * t for x, (_u, _v, t) in zip(f, network)) / 60
    return {"flows": _r(f), "VHT": _r(vht)}


def _ref_aon(network, od):
    import networkx as nx
    g = nx.DiGraph()
    for u, v, t in network:
        g.add_edge(u, v, time=t, flow=0.0)
    for o, d, q in od:
        p = nx.dijkstra_path(g, o, d, weight="time")
        for a, b in zip(p, p[1:]):
            g[a][b]["flow"] += q
    f = [g[u][v]["flow"] for u, v, _t in network]
    return {"flows": f, "VHT": sum(g[u][v]["flow"] * g[u][v]["time"] for u, v in g.edges) / 60}


def _rand_aon(rng):
    names, roads, _sp = _rand_network(rng, 4, 8)
    links = []
    for part in roads.split(", "):
        ab, _L = part.split(" ")
        a, b = ab.split("-")
        links.append(f"{a}-{b} {rng.uniform(1, 15):.3f}")
        links.append(f"{b}-{a} {rng.uniform(1, 15):.3f}")
    pairs = set()
    while len(pairs) < rng.randint(1, 6):
        pairs.add(tuple(rng.sample(names, 2)))
    od = ", ".join(f"{a}-{b} {rng.randint(10, 2000)}" for a, b in sorted(pairs))
    return {"network": ", ".join(links), "od": od}


proc("transport_all_or_nothing_assignment", "All-or-nothing traffic assignment", "travel demand",
     "Loads every origin-destination demand onto its fastest path through a network of one-way links with travel "
     "times (min): the flow on each link (in the order given) and the total vehicle-hours travelled.",
     [_in("network", "number", "number", "one-way links as 'A-B time' with travel times in min", ("network", "links"),
          typ="edges", min_length=1),
      _in("od", "number", "number", "trip demands as 'origin-destination trips' (veh)", ("od", "demand", "trips"),
          typ="edges", min_length=1)],
     [_o("flows", "number", "flow on each link in the order given (veh)"), _o("VHT", "vehicle_hours", "vehicle-hours travelled (veh h)")],
     aon, _ref_aon, lambda network, od: (lambda o: dict(o, VHT=_r(o["VHT"] * 60)))(aon(network, od)),
     _rand_aon,
     [{"network": "A-B 5, B-A 5, A-C 3, C-A 3, B-D 4, D-B 4, C-D 7, D-C 7, B-C 1, C-B 1",
       "od": "A-D 800, C-B 200, D-A 300"},
      {"network": "1-2 4, 2-3 6, 1-3 12, 3-4 3, 2-4 10", "od": "1-4 1000, 2-4 400"}],
     ["all or nothing assignment", "all-or-nothing assignment", "traffic assignment all or nothing"],
     conditions=["positive link times", "every trip pair connected"],
     invalid=[{"network": "A-B 5, B-C 4", "od": "C-A 100"}],
     method="for each trip pair the fastest path (Dijkstra); its demand is added to every link on it",
     latex=r"x_a = \sum_{rs} q_{rs}\,\delta_{a, p^*_{rs}}")


# =============================================================================================== trip distribution
def _furness(base, O, D, tol=1e-13, it=20000):
    n, m = len(O), len(D)
    T = [row[:] for row in base]
    for _ in range(it):
        for i in range(n):
            s = math.fsum(T[i])
            _need(s > 0, "every origin needs a positive entry")
            f = O[i] / s
            T[i] = [x * f for x in T[i]]
        err = 0.0
        for j in range(m):
            s = math.fsum(T[i][j] for i in range(n))
            _need(s > 0, "every destination needs a positive entry")
            f = D[j] / s
            for i in range(n):
                T[i][j] *= f
        for i in range(n):
            err = max(err, abs(math.fsum(T[i]) - O[i]) / O[i])
        if err < tol:
            return T
    raise ValueError("the balancing did not converge")


def _balance_check(O, D):
    O = _vec("productions", O, positive=True)
    D = _vec("attractions", D, positive=True)
    _need(abs(sum(O) - sum(D)) <= 1e-9 * sum(O), "total productions must equal total attractions")
    return O, D


def gravity(O, D, costs, beta, deterrence="exponential"):
    O, D = _balance_check(O, D)
    c = _mat("costs", costs, rows=len(O), cols=len(D), nonneg=True)
    _finite(beta=beta)
    _need(beta >= 0, "beta must not be negative")
    if deterrence == "power":
        _need(all(x > 0 for r in c for x in r), "the power deterrence function needs positive costs")
        f = [[x ** -beta for x in r] for r in c]
    else:
        f = [[math.exp(-beta * x) for x in r] for r in c]
    base = [[f[i][j] * D[j] for j in range(len(D))] for i in range(len(O))]
    return {"trips": [_r(row) for row in _furness(base, O, D)]}


def _ref_balance(base, O, D):
    """Reference: the balanced matrix as the minimiser of the convex dual of the balancing problem,
    phi(a, b) = sum_ij exp(a_i + b_j) t_ij - sum_i o_i a_i - sum_j d_j b_j (log-factors, b_last = 0, totals scaled to
    1), found by SciPy's trust-region method with the exact Hessian and polished by Newton steps: a different
    algorithm from the alternating row and column scaling of the procedure."""
    import numpy as np
    from scipy.optimize import minimize
    B = np.asarray(base, float)
    O, D = np.asarray(O, float), np.asarray(D, float)
    S = O.sum()
    o, d, Bs = O / S, D / S, B / B.max()
    n, m = B.shape

    def T(z):
        a, b = z[:n], np.concatenate([z[n:], [0.0]])
        return np.exp(a[:, None] + b[None, :]) * Bs

    def phi(z):
        return T(z).sum() - o @ z[:n] - d[:-1] @ z[n:]

    def grad(z):
        t = T(z)
        return np.concatenate([t.sum(1) - o, t.sum(0)[:-1] - d[:-1]])

    def hess(z):
        t = T(z)
        H = np.zeros((n + m - 1, n + m - 1))
        H[:n, :n] = np.diag(t.sum(1))
        H[n:, n:] = np.diag(t.sum(0)[:-1])
        H[:n, n:] = t[:, :-1]
        H[n:, :n] = t[:, :-1].T
        return H
    z0 = np.concatenate([np.log(o / Bs.sum(1)), np.zeros(m - 1)])
    z = minimize(phi, z0, jac=grad, hess=hess, method="trust-exact", options={"gtol": 1e-14}).x
    for _ in range(60):
        g = grad(z)
        if np.max(np.abs(g)) < 1e-17:
            break
        z = z - np.linalg.solve(hess(z), g)
    t = T(z)
    res = np.concatenate([(t.sum(1) - o) / o, (t.sum(0) - d) / d])
    if not np.all(np.isfinite(t)) or np.max(np.abs(res)) > 1e-12:
        raise RuntimeError("the reference solve did not converge")
    return (t * S).tolist()


def _ref_gravity(O, D, costs, beta, deterrence="exponential"):
    f = [[(x ** -beta if deterrence == "power" else math.exp(-beta * x)) for x in r] for r in costs]
    return {"trips": _ref_balance([[f[i][j] * D[j] for j in range(len(D))] for i in range(len(O))], O, D)}


def _rand_pd(rng, n, m):
    O = [rng.randint(100, 5000) for _ in range(n)]
    D = [rng.randint(100, 5000) for _ in range(m)]
    s = sum(O)
    D = [x * s / sum(D) for x in D]
    D = [round(x, 6) for x in D]
    D[-1] = s - sum(D[:-1])
    return O, D


def _rand_gravity(rng):
    n, m = rng.randint(2, 5), rng.randint(2, 5)
    O, D = _rand_pd(rng, n, m)
    costs = [[round(rng.uniform(2, 40), 2) for _ in range(m)] for _ in range(n)]
    det = rng.choice(["exponential", "power"])
    return {"O": O, "D": D, "costs": costs, "beta": rng.uniform(0.02, 0.15) if det == "exponential" else rng.uniform(0.5, 2.5),
            "deterrence": det}


proc("transport_gravity_model", "Doubly constrained gravity model of trip distribution", "travel demand",
     "Trips between zones T_ij = A_i O_i B_j D_j f(c_ij) that match both the productions O_i and the attractions D_j, "
     "with deterrence f(c) = exp(-beta c) or c^-beta, balanced by Furness iterations until every total matches to 1e-13.",
     [_VEC("O", "trip productions of the origin zones", ("productions",)),
      _VEC("D", "trip attractions of the destination zones", ("attractions",)),
      _MAT("costs", "travel cost (time) matrix, rows origins, in min", ("costs", "cost matrix")),
      _N("beta", "deterrence parameter", ("beta",)),
      _in("deterrence", "number", "number", "exponential (default) or power", ("deterrence", "function"), typ="choice",
          choices=["exponential", "power"], required=False)],
     [_o("trips", "number", "trip matrix (rows origins, columns destinations)")],
     gravity, _ref_gravity,
     lambda O, D, costs, beta, deterrence="exponential": {"trips": [[_r(o * d / sum(D)) for d in D] for o in O]},
     _rand_gravity,
     [{"O": [1000, 1500, 800], "D": [1200, 900, 1200], "costs": [[5, 12, 20], [10, 6, 15], [18, 14, 7]], "beta": 0.1},
      {"O": [500, 700], "D": [600, 600], "costs": [[4, 9], [8, 3]], "beta": 2, "deterrence": "power"}],
     ["gravity model", "doubly constrained gravity model", "trip distribution gravity model"],
     conditions=["sum of productions = sum of attractions", "beta >= 0", "power deterrence: positive costs"],
     invalid=[{"O": [1000, 1500], "D": [1200, 900], "costs": [[5, 12], [10, 6]], "beta": 0.1}],
     method="Furness (iterative proportional fitting) of O_i D_j f(c_ij) to the row and column totals",
     latex=r"T_{ij} = A_i O_i B_j D_j f(c_{ij}),\quad \sum_j T_{ij} = O_i,\ \sum_i T_{ij} = D_j")


def furness(base, O, D):
    O, D = _balance_check(O, D)
    t = _mat("base", base, rows=len(O), cols=len(D), nonneg=True)
    return {"trips": [_r(row) for row in _furness(t, O, D)]}


proc("transport_furness_balancing", "Growth-factor (Furness) balancing of a trip matrix", "travel demand",
     "Scales a base-year trip matrix to future row totals (productions) and column totals (attractions) by the "
     "Furness (biproportional) method, iterated until every total matches to 1e-13.",
     [_MAT("base", "base-year trip matrix (rows origins)", ("base matrix", "base")),
      _VEC("O", "target row totals (productions)", ("productions", "row totals")),
      _VEC("D", "target column totals (attractions)", ("attractions", "column totals"))],
     [_o("trips", "number", "balanced trip matrix")],
     furness, lambda base, O, D: {"trips": _ref_balance(base, O, D)},
     lambda base, O, D: {"trips": [[_r(base[i][j] * O[i] / sum(base[i])) for j in range(len(D))] for i in range(len(O))]},
     lambda rng: (lambda n, m: (lambda OD: {"base": [[rng.randint(5, 900) for _ in range(m)] for _ in range(n)],
                                            "O": OD[0], "D": OD[1]})(_rand_pd(rng, n, m)))(rng.randint(2, 5), rng.randint(2, 5)),
     [{"base": [[100, 200, 150], [300, 100, 250], [50, 150, 200]], "O": [600, 800, 500], "D": [550, 600, 750]},
      {"base": [[40, 60], [80, 20]], "O": [130, 110], "D": [140, 100]}],
     ["furness balancing", "growth factor balancing", "furness method"],
     conditions=["sum of row totals = sum of column totals", "non-negative base matrix with a positive entry in every row and column"],
     invalid=[{"base": [[100, 200], [300, 100]], "O": [600, 800], "D": [550, 600]}],
     method="alternately scale rows to O_i and columns to D_j until both match", latex=r"T_{ij} = t_{ij}\,a_i\,b_j")


# =============================================================================================== mode choice (logit)
def _softmax(V):
    mx = max(V)
    e = [math.exp(v - mx) for v in V]
    s = math.fsum(e)
    return [x / s for x in e], mx + math.log(s)


def logit_shares(V):
    v = _vec("utilities", V)
    _need(len(v) >= 2, "give the utilities of at least two alternatives")
    P, ls = _softmax(v)
    return {"P": _r(P), "logsum": _r(ls)}


def _ref_logit(V):
    import numpy as np
    from scipy.special import logsumexp, softmax
    return {"P": softmax(np.asarray(V, float)).tolist(), "logsum": float(logsumexp(np.asarray(V, float)))}


proc("transport_logit_mode_shares", "Multinomial logit mode shares from utilities", "mode choice",
     "Choice probabilities (mode shares) of a multinomial logit model from the systematic utilities of the "
     "alternatives, and the logsum (expected maximum utility).",
     [_VEC("V", "utility of each alternative", ("utilities", "V"), min_length=2)],
     [_o("P", "number", "probability of each alternative"), _o("logsum", "number", "logsum ln(sum exp V)")],
     logit_shares, _ref_logit,
     lambda V: (lambda o: {"P": _r(o[0]), "logsum": _r(-o[1])})(_softmax([-x for x in V])),
     lambda rng: {"V": [round(rng.uniform(-4, 2), 3) for _ in range(rng.randint(2, 6))]},
     [{"V": [-1.2, -0.8, -2.1]}, {"V": [0.5, -0.3]}],
     ["logit mode shares", "multinomial logit shares", "logit choice probabilities"],
     conditions=["at least two alternatives"], invalid=[{"V": [0.5]}],
     method="P_i = exp(V_i) / sum_j exp(V_j)", latex=r"P_i = \frac{e^{V_i}}{\sum_j e^{V_j}}")


def logit_attributes(X, beta, asc=None):
    x = _mat("X", X)
    b = _vec("beta", beta, n=len(x[0]))
    n = len(x)
    _need(n >= 2, "give at least two alternatives (rows)")
    a = _vec("asc", asc, n=n) if asc is not None else [0.0] * n
    V = [a[i] + math.fsum(x[i][k] * b[k] for k in range(len(b))) for i in range(n)]
    P, _ls = _softmax(V)
    return {"V": _r(V), "P": _r(P)}


def _ref_logit_attr(X, beta, asc=None):
    import numpy as np
    from scipy.special import softmax
    V = np.asarray(X, float) @ np.asarray(beta, float) + (np.asarray(asc, float) if asc is not None else 0.0)
    return {"V": V.tolist(), "P": softmax(V).tolist()}


proc("transport_logit_from_attributes", "Logit mode choice from attributes and coefficients", "mode choice",
     "Utilities V_i = ASC_i + sum_k beta_k x_ik of each mode from its attributes (rows of X: modes; columns: attributes "
     "such as time in min and cost) and the coefficients, and the resulting logit choice probabilities.",
     [_MAT("X", "attributes: one row per mode, one column per attribute", ("attributes", "X")),
      _VEC("beta", "coefficient of each attribute", ("coefficients", "beta")),
      _VEC("asc", "alternative-specific constant of each mode (default 0)", ("constants", "asc"), required=False)],
     [_o("V", "number", "utility of each mode"), _o("P", "number", "probability of each mode")],
     logit_attributes, _ref_logit_attr,
     lambda X, beta, asc=None: (lambda o: dict(o, P=_r(_softmax([v for v in o["V"]])[0][::-1])))(logit_attributes(X, beta, asc)),
     lambda rng: (lambda n, k: {"X": [[round(rng.uniform(1, 60), 2) for _ in range(k)] for _ in range(n)],
                                "beta": [round(rng.uniform(-0.12, -0.01), 4) for _ in range(k)],
                                "asc": [0.0] + [round(rng.uniform(-1.5, 1.5), 3) for _ in range(n - 1)]})(
         rng.randint(2, 5), rng.randint(1, 3)),
     [{"X": [[30, 2.5], [45, 1.5], [60, 0.8]], "beta": [-0.05, -0.4], "asc": [0, -0.5, -1.0]},
      {"X": [[25, 4], [40, 2]], "beta": [-0.03, -0.25]}],
     ["logit from attributes", "logit mode choice from attributes", "mode choice probabilities from attributes"],
     conditions=["one coefficient per attribute column", "at least two modes"],
     invalid=[{"X": [[30, 2.5], [45, 1.5]], "beta": [-0.05]}],
     method="V = X beta + ASC, P = softmax(V)", latex=r"V_i = ASC_i + \sum_k \beta_k x_{ik},\quad P_i = \frac{e^{V_i}}{\sum_j e^{V_j}}")


def logit_elasticities(V, x, beta):
    v = _vec("utilities", V)
    xs = _vec("x", x, n=len(v))
    _finite(beta=beta)
    _need(len(v) >= 2, "give at least two alternatives")
    P, _ls = _softmax(v)
    return {"P": _r(P), "direct": _r([_snap(beta * xi * (1 - p), 1.0) for xi, p in zip(xs, P)]),
            "cross": _r([_snap(-beta * xi * p, 1.0) for xi, p in zip(xs, P)])}


def _ref_elasticities(V, x, beta):
    """Point elasticities by complex-step differentiation of the logit probabilities (NumPy): dP_j / dx_i from the
    imaginary part of P(V + i h beta e_i) / h."""
    import numpy as np
    V = np.asarray(V, float)
    n = len(V)
    h = 1e-30

    def P(z):
        e = np.exp(z - V.max())
        return e / e.sum()
    P0 = P(V.astype(complex)).real
    direct, cross = [], []
    for i in range(n):
        dz = np.zeros(n, complex)
        dz[i] = 1j * h * beta
        dP = P(V + dz).imag / h
        j = (i + 1) % n
        direct.append(float(dP[i] * x[i] / P0[i]))
        cross.append(float(dP[j] * x[i] / P0[j]))
    return {"P": P0.tolist(), "direct": direct, "cross": cross}


proc("transport_logit_elasticities", "Direct and cross elasticities of logit choice probabilities", "mode choice",
     "Point elasticities of logit probabilities with respect to an attribute x (such as cost or time) that enters the "
     "utility linearly with coefficient beta: direct E_ii = beta x_i (1 - P_i) and cross E_ji = -beta x_i P_i (the same "
     "for every other alternative j, the IIA property).",
     [_VEC("V", "utility of each alternative", ("utilities", "V"), min_length=2),
      _VEC("x", "value of the attribute for each alternative", ("attribute", "x")), _N("beta", "coefficient of the attribute", ("beta", "coefficient"))],
     [_o("P", "number", "probability of each alternative"), _o("direct", "number", "direct elasticity of each alternative"),
      _o("cross", "number", "cross elasticity of the other alternatives with respect to each alternative's attribute")],
     logit_elasticities, _ref_elasticities,
     lambda V, x, beta: (lambda o: dict(o, direct=_r([beta * xi * p for xi, p in zip(x, _softmax(V)[0])])))(
         logit_elasticities(V, x, beta)),
     lambda rng: (lambda n: {"V": [round(rng.uniform(-3, 1), 3) for _ in range(n)], "x": [round(rng.uniform(1, 60), 2) for _ in range(n)],
                             "beta": round(rng.uniform(-0.1, -0.01), 4)})(rng.randint(2, 6)),
     [{"V": [-1.2, -0.8, -2.1], "x": [30, 45, 20], "beta": -0.04}, {"V": [0.2, -0.4], "x": [3.5, 2.0], "beta": -0.3}],
     ["logit elasticities", "mode choice elasticities", "elasticities of logit probabilities"],
     conditions=["one attribute value per alternative"], invalid=[{"V": [-1.2, -0.8, -2.1], "x": [30, 45], "beta": -0.04}],
     method="E_ii = beta x_i (1 - P_i), E_ji = -beta x_i P_i", latex=r"E^{P_i}_{x_i} = \beta x_i (1 - P_i),\quad E^{P_j}_{x_i} = -\beta x_i P_i")


def _nests(nest, lam, n):
    k = _vec("nest", nest, n=n)
    _need(all(v == int(v) and v >= 1 for v in k), "nest numbers must be whole numbers 1, 2, ...")
    k = [int(v) for v in k]
    K = max(k)
    lm = _vec("lam", lam, n=K)
    _need(set(k) == set(range(1, K + 1)), "every nest 1..K must hold at least one alternative")
    _need(all(0 < x <= 1 for x in lm), "nest parameters must lie in (0, 1]")
    return k, lm, K


def nested_logit(V, nest, lam):
    v = _vec("utilities", V)
    _need(len(v) >= 2, "give at least two alternatives")
    k, lm, K = _nests(nest, lam, len(v))
    I = []
    for m in range(1, K + 1):
        vals = [v[i] / lm[m - 1] for i in range(len(v)) if k[i] == m]
        mx = max(vals)
        I.append(mx + math.log(math.fsum(math.exp(x - mx) for x in vals)))
    PN, _ = _softmax([lm[m] * I[m] for m in range(K)])
    P = [math.exp(v[i] / lm[k[i] - 1] - I[k[i] - 1]) * PN[k[i] - 1] for i in range(len(v))]
    return {"P": _r(P), "P_nest": _r(PN)}


def _ref_nested(V, nest, lam):
    """Generalised extreme value route: P_i = y_i dG/dy_i / G with G(y) = sum_m (sum_{j in m} y_j^(1/lam_m))^lam_m and
    y = exp(V); the derivative by complex-step differentiation (NumPy)."""
    import numpy as np
    V = np.asarray(V, float)
    k = [int(v) for v in nest]
    y = np.exp(V)
    h = 1e-30

    def Gf(z):
        return sum(sum(z[i] ** (1 / lam[m - 1]) for i in range(len(z)) if k[i] == m) ** lam[m - 1]
                   for m in range(1, max(k) + 1))
    G0 = Gf(y.astype(complex)).real
    P = []
    for i in range(len(y)):
        z = y.astype(complex)
        z[i] += 1j * h
        P.append(float(y[i] * Gf(z).imag / h / G0))
    PN = [sum(P[i] for i in range(len(P)) if k[i] == m) for m in range(1, max(k) + 1)]
    return {"P": P, "P_nest": PN}


def _rand_nested(rng):
    n = rng.randint(3, 6)
    K = rng.randint(2, min(3, n))
    nest = list(range(1, K + 1)) + [rng.randint(1, K) for _ in range(n - K)]
    rng.shuffle(nest)
    return {"V": [round(rng.uniform(-3, 1), 3) for _ in range(n)], "nest": nest,
            "lam": [round(rng.uniform(0.3, 1.0), 3) for _ in range(K)]}


proc("transport_nested_logit", "Nested logit choice probabilities", "mode choice",
     "Choice probabilities of a two-level nested logit model (for example car / public transport nests holding bus and "
     "rail): alternatives grouped into nests 1..K with dissimilarity (logsum) parameters lam_k in (0, 1]; also the "
     "probability of each nest.",
     [_VEC("V", "utility of each alternative", ("utilities", "V"), min_length=2),
      _VEC("nest", "nest number (1, 2, ...) of each alternative", ("nests", "nest")),
      _VEC("lam", "dissimilarity parameter of each nest, in (0, 1]", ("nest parameters", "lambda"))],
     [_o("P", "number", "probability of each alternative"), _o("P_nest", "number", "probability of each nest")],
     nested_logit, _ref_nested,
     lambda V, nest, lam: (lambda P: {"P": _r(P), "P_nest": _r([sum(P[i] for i in range(len(P)) if int(nest[i]) == m)
                                                                for m in range(1, int(max(nest)) + 1)])})(_softmax(V)[0]),
     _rand_nested,
     [{"V": [-0.5, -1.2, -1.0], "nest": [1, 2, 2], "lam": [1.0, 0.5]},
      {"V": [0.2, -0.6, -0.4, -1.5], "nest": [1, 1, 2, 2], "lam": [0.7, 0.4]}],
     ["nested logit", "nested logit probabilities", "nested logit model"],
     conditions=["nests numbered 1..K, each used", "0 < lam_k <= 1"], invalid=[{"V": [-0.5, -1.2, -1.0], "nest": [1, 2, 2], "lam": [1.0, 1.5]}],
     method="I_k = ln sum_{j in k} exp(V_j / lam_k); P(k) = exp(lam_k I_k) / sum_m exp(lam_m I_m); P(i|k) = exp(V_i/lam_k - I_k)",
     latex=r"P_i = \frac{e^{V_i/\lambda_k}}{e^{I_k}}\cdot\frac{e^{\lambda_k I_k}}{\sum_m e^{\lambda_m I_m}},\quad I_k = \ln\sum_{j\in k} e^{V_j/\lambda_k}")


def logsum_benefit(V0, V1, beta_cost):
    a = _vec("V0", V0)
    b = _vec("V1", V1, n=len(a))
    _finite(beta_cost=beta_cost)
    _need(beta_cost < 0, "the cost coefficient must be negative (utility falls as cost rises)")
    l0, l1 = _softmax(a)[1], _softmax(b)[1]
    return {"dCS": _r(_snap((l1 - l0) / -beta_cost, max(abs(l1), abs(l0), 1.0) / -beta_cost)), "logsum0": _r(l0), "logsum1": _r(l1)}


def _ref_logsum(V0, V1, beta_cost):
    from scipy.special import logsumexp
    l0, l1 = float(logsumexp(V0)), float(logsumexp(V1))
    d = (l1 - l0) / -beta_cost
    return {"dCS": 0.0 if abs(d) <= 1e-12 * max(abs(l1), abs(l0), 1.0) / -beta_cost else d, "logsum0": l0, "logsum1": l1}


proc("transport_logsum_user_benefit", "User benefit of a change from the logit logsum", "mode choice",
     "Change in consumer surplus per trip (in money units) when the utilities of the alternatives change from V0 to V1, "
     "from the difference of the logsums divided by the (negative) cost coefficient.",
     [_VEC("V0", "utilities before the change", ("utilities before", "V0"), min_length=2),
      _VEC("V1", "utilities after the change", ("utilities after", "V1"), min_length=2),
      _N("beta_cost", "cost coefficient (utility per money unit, negative)", ("cost coefficient",))],
     [_o("dCS", "number", "change in consumer surplus per trip (money units)"), _o("logsum0", "number", "logsum before"),
      _o("logsum1", "number", "logsum after")],
     logsum_benefit, _ref_logsum,
     lambda V0, V1, beta_cost: (lambda o: dict(o, dCS=_r(-o["dCS"])))(logsum_benefit(V0, V1, beta_cost)),
     lambda rng: (lambda n: (lambda v0: {"V0": v0, "V1": [round(x + rng.uniform(-0.6, 0.8), 3) for x in v0],
                                         "beta_cost": round(rng.uniform(-0.5, -0.05), 4)})(
         [round(rng.uniform(-3, 1), 3) for _ in range(n)]))(rng.randint(2, 5)),
     [{"V0": [-1.2, -0.8, -2.1], "V1": [-1.2, -0.5, -1.6], "beta_cost": -0.25}, {"V0": [0.3, -0.7], "V1": [0.1, -0.2], "beta_cost": -0.1}],
     ["logsum user benefit", "consumer surplus change from logsum", "logsum benefit"],
     conditions=["same alternatives before and after", "beta_cost < 0"],
     invalid=[{"V0": [-1.2, -0.8, -2.1], "V1": [-1.2, -0.5, -1.6], "beta_cost": 0.25}],
     method="dCS = (ln sum exp V1 - ln sum exp V0) / (-beta_cost)",
     latex=r"\Delta CS = \frac{1}{-\beta_c}\left[\ln\sum_j e^{V^1_j} - \ln\sum_j e^{V^0_j}\right]")


# =============================================================================================== trip generation
def _lstsq_qr(A, y):
    """Least squares by Householder QR (plain Python)."""
    m, n = len(A), len(A[0])
    R = [row[:] for row in A]
    b = y[:]
    for k in range(n):
        norm = math.sqrt(math.fsum(R[i][k] ** 2 for i in range(k, m)))
        _need(norm > 1e-12 * max(1.0, max(abs(R[i][k]) for i in range(m))), "the predictors are collinear")
        alpha = -norm if R[k][k] >= 0 else norm
        v = [0.0] * m
        v[k] = R[k][k] - alpha
        for i in range(k + 1, m):
            v[i] = R[i][k]
        vv = math.fsum(x * x for x in v[k:])
        if vv == 0:
            continue
        for j in range(k, n):
            s = 2 * math.fsum(v[i] * R[i][j] for i in range(k, m)) / vv
            for i in range(k, m):
                R[i][j] -= s * v[i]
        s = 2 * math.fsum(v[i] * b[i] for i in range(k, m)) / vv
        for i in range(k, m):
            b[i] -= s * v[i]
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        _need(abs(R[i][i]) > 0, "the predictors are collinear")
        x[i] = (b[i] - math.fsum(R[i][j] * x[j] for j in range(i + 1, n))) / R[i][i]
    return x


def trip_regression(X, y, x_new=None):
    x = _mat("X", X)
    yy = _vec("y", y, n=len(x))
    p = len(x[0])
    _need(len(x) > p + 1, "need more zones than coefficients")
    A = [[1.0] + row for row in x]
    coef = _lstsq_qr(A, yy)
    fit = [math.fsum(a * c for a, c in zip(row, coef)) for row in A]
    my = math.fsum(yy) / len(yy)
    sst = math.fsum((v - my) ** 2 for v in yy)
    _need(sst > 0, "the trips must not all be equal")
    r2 = 1 - math.fsum((v - f) ** 2 for v, f in zip(yy, fit)) / sst
    out = {"coef": _r(coef), "r2": _r(r2)}
    if x_new is not None:
        xn = _vec("x_new", x_new, n=p)
        out["y_new"] = _r(coef[0] + math.fsum(a * c for a, c in zip(xn, coef[1:])))
    return out


def _ref_trip_regression(X, y, x_new=None):
    import numpy as np
    A = np.column_stack([np.ones(len(X)), np.asarray(X, float)])
    yy = np.asarray(y, float)
    coef = np.linalg.lstsq(A, yy, rcond=None)[0]
    res = yy - A @ coef
    out = {"coef": coef.tolist(), "r2": float(1 - (res @ res) / ((yy - yy.mean()) @ (yy - yy.mean())))}
    if x_new is not None:
        out["y_new"] = float(coef[0] + np.asarray(x_new, float) @ coef[1:])
    return out


def _rand_regression(rng):
    n, p = rng.randint(6, 20), rng.randint(1, 3)
    X = [[round(rng.uniform(50, 2000), 1) for _ in range(p)] for _ in range(n)]
    b = [rng.uniform(-50, 200)] + [rng.uniform(0.2, 3.0) for _ in range(p)]
    y = [round(b[0] + sum(bi * xi for bi, xi in zip(b[1:], row)) + rng.gauss(0, 80), 1) for row in X]
    return {"X": X, "y": y, "x_new": [round(rng.uniform(50, 2000), 1) for _ in range(p)]}


proc("transport_trip_generation_regression", "Trip generation by multiple linear regression", "travel demand",
     "Least-squares trip-generation model trips = b0 + b1 x1 + ... fitted to zonal data (rows of X: zones; columns: "
     "households, employment, cars, ...), its coefficient of determination, and optionally the trips predicted for a "
     "new zone.",
     [_MAT("X", "zonal predictors: one row per zone", ("predictors", "X")), _VEC("y", "observed trips of each zone", ("trips", "y")),
      _VEC("x_new", "predictors of a zone to forecast (optional)", ("new zone", "x_new"), required=False)],
     [_o("coef", "number", "coefficients b0 (intercept), b1, ..."), _o("r2", "number", "coefficient of determination"),
      _o("y_new", "number", "forecast trips of the new zone")],
     trip_regression, _ref_trip_regression,
     lambda X, y, x_new=None: (lambda c: {"coef": _r([0.0] + c), "r2": 0.5})(_lstsq_qr([list(map(float, r)) for r in X], list(map(float, y)))),
     _rand_regression,
     [{"X": [[120, 300], [250, 420], [400, 380], [180, 600], [520, 700], [300, 250], [450, 520]],
       "y": [1450, 2300, 3100, 2050, 4200, 2200, 3600], "x_new": [350, 500]},
      {"X": [[800], [1200], [1500], [2000], [2600]], "y": [2100, 3000, 3900, 5050, 6400]}],
     ["trip generation regression", "trip generation model", "regression of trips"],
     conditions=["more zones than coefficients", "predictors not collinear"],
     invalid=[{"X": [[120, 300], [250, 420]], "y": [1450, 2300]}],
     method="ordinary least squares with an intercept (Householder QR)", latex=r"\hat{\mathbf b} = \arg\min \lVert \mathbf y - [\mathbf 1\ X]\mathbf b\rVert^2")


# =============================================================================================== vehicle dynamics (bicycle model)
def _bike_check(m, L, l_f, C_f, C_r):
    _pos(m=m, C_f=C_f, C_r=C_r)
    _check_axles(L, l_f)
    return L - l_f, m * ((L - l_f) / (L * C_f) - l_f / (L * C_r))


def understeer(m, L, l_f, C_f, C_r):
    l_r, K = _bike_check(m, L, l_f, C_f, C_r)
    _need(abs(K) > 1e-12 * m / min(C_f, C_r), "neutral steer (K = 0): no characteristic or critical speed")
    v = math.sqrt(L / abs(K))
    return {"K": _r(K), "K_deg_per_g": _r(math.degrees(K * G_)), "behaviour": "understeer" if K > 0 else "oversteer",
            "v_c": _r(v), "v_c_kmh": _kmh(v)}


def _bike_ss(m, L, l_f, C_f, C_r, u, Iz=None, outputs="r"):
    """Reference: the linear single-track (bicycle) model in state space, states lateral velocity and yaw rate, input
    the front-wheel steer angle (python-control)."""
    import control
    import numpy as np
    l_r = L - l_f
    Iz = Iz if Iz is not None else m * l_f * l_r
    A = np.array([[-(C_f + C_r) / (m * u), (C_r * l_r - C_f * l_f) / (m * u) - u],
                  [(C_r * l_r - C_f * l_f) / (Iz * u), -(C_f * l_f ** 2 + C_r * l_r ** 2) / (Iz * u)]])
    B = np.array([[C_f / m], [C_f * l_f / Iz]])
    C = np.eye(2) if outputs == "both" else np.array([[0.0, 1.0]])
    return control.ss(A, B, C, np.zeros((C.shape[0], 1))), A


def _yaw_gain(m, L, l_f, C_f, C_r, u):
    import control
    import numpy as np
    return float(np.squeeze(control.dcgain(_bike_ss(m, L, l_f, C_f, C_r, u)[0])))


def _ref_understeer(m, L, l_f, C_f, C_r):
    """K from the steady-state yaw-rate gain of the dynamic model (r/delta = u / (L + K u^2)); the characteristic speed
    as the speed where the steer angle needed is twice the Ackermann angle (brentq on the model's gain); the critical
    speed as the speed where the model's state matrix becomes singular (stability boundary)."""
    import numpy as np
    u0 = 20.0
    K = (u0 / _yaw_gain(m, L, l_f, C_f, C_r, u0) - L) / u0 ** 2
    if K > 0:
        f = lambda u: _yaw_gain(m, L, l_f, C_f, C_r, u) * 2 * L / u - 1
    else:
        f = lambda u: float(np.linalg.det(_bike_ss(m, L, l_f, C_f, C_r, u)[1]))
    lo, hi = 0.5, 1.0
    while f(lo) * f(hi) > 0 and hi < 1e5:
        lo, hi = hi, hi * 2
    v = _brentq(f, lo, hi)
    return {"K": K, "K_deg_per_g": math.degrees(K * 9.81), "behaviour": "understeer" if K > 0 else "oversteer",
            "v_c": v, "v_c_kmh": v * 3.6}


_BIKE_IN = [_M("m", "vehicle mass", ("mass",)), _L("L", "wheelbase", ("wheelbase",)),
            _L("l_f", "distance from the front axle to the centre of gravity", ("cg to front axle",)),
            _in("C_f", "cornering_stiffness", "cornering", "cornering stiffness of the front axle", ("front cornering stiffness",), "N/rad"),
            _in("C_r", "cornering_stiffness", "cornering", "cornering stiffness of the rear axle", ("rear cornering stiffness",), "N/rad")]


def _rand_bike(rng, oversteer_share=0.4):
    L = rng.uniform(2.3, 3.2)
    l_f = rng.uniform(0.35, 0.6) * L
    Cf = rng.uniform(50e3, 150e3)
    ratio = rng.uniform(0.6, 0.9) if rng.random() < oversteer_share else rng.uniform(1.1, 1.8)
    Cr = Cf * ratio * l_f / (L - l_f)                 # ratio > 1: understeer, < 1: oversteer
    return {"m": rng.uniform(900, 2500), "L": L, "l_f": l_f, "C_f": Cf, "C_r": Cr}


proc("transport_understeer_gradient", "Understeer gradient and characteristic or critical speed (bicycle model)",
     "vehicle dynamics",
     "Understeer gradient K of the linear single-track model from axle cornering stiffnesses and the centre-of-gravity "
     "position, in rad/(m/s^2) and deg/g, whether the vehicle understeers or oversteers, and its characteristic speed "
     "(understeer: the steer angle is twice the Ackermann angle) or critical speed (oversteer: it becomes unstable).",
     _BIKE_IN,
     [_o("K", "understeer_gradient", "understeer gradient (rad s^2/m)"), _o("K_deg_per_g", "understeer_gradient_deg_per_g", "understeer gradient (deg/g)"),
      _o("behaviour", "text", "understeer or oversteer"), _o("v_c", "speed", "characteristic or critical speed (m/s)"),
      _o("v_c_kmh", "speed_kmh", "characteristic or critical speed (km/h)")],
     understeer, _ref_understeer,
     lambda m, L, l_f, C_f, C_r: (lambda K: {"K": _r(K), "K_deg_per_g": _r(math.degrees(K * G_)),
                                             "behaviour": "understeer" if K > 0 else "oversteer",
                                             "v_c": _r(math.sqrt(L / abs(K))), "v_c_kmh": _kmh(math.sqrt(L / abs(K)))})(
         m * ((L - l_f) / C_f - l_f / C_r)),
     _rand_bike,
     [{"m": 1500, "L": 2.7, "l_f": 1.2, "C_f": 90000, "C_r": 85000}, {"m": 1300, "L": 2.5, "l_f": 1.4, "C_f": 90000, "C_r": 60000}],
     ["understeer gradient", "characteristic speed", "critical speed of a vehicle"],
     conditions=["0 < l_f < L", "positive cornering stiffnesses", "not exactly neutral steer"],
     invalid=[{"m": 1500, "L": 2.7, "l_f": 3.0, "C_f": 90000, "C_r": 85000}],
     method="K = m (l_r / (L C_f) - l_f / (L C_r)); v_ch = sqrt(L / K) (K > 0), v_crit = sqrt(-L / K) (K < 0)",
     latex=r"K = \frac{m}{L}\left(\frac{l_r}{C_f} - \frac{l_f}{C_r}\right),\quad v_{ch} = \sqrt{L/K}")


def cornering(m, L, l_f, C_f, C_r, u, delta):
    l_r, K = _bike_check(m, L, l_f, C_f, C_r)
    _pos(u=u)
    _finite(delta=delta)
    _need(delta != 0, "give a non-zero steer angle")
    den = L + K * u * u
    _need(den > 0, "above the critical speed: the vehicle has no steady turn")
    r = u * delta / den
    beta = delta * (l_r - m * l_f * u * u / (L * C_r)) / den
    return {"r": _r(r), "a_y": _r(u * r), "R": _r(abs(u / r)), "beta": _r(math.degrees(beta))}


def _ref_cornering(m, L, l_f, C_f, C_r, u, delta):
    import control
    import numpy as np
    g = np.asarray(control.dcgain(_bike_ss(m, L, l_f, C_f, C_r, u, outputs="both")[0])).reshape(-1)
    vy, r = g[0] * delta, g[1] * delta
    return {"r": float(r), "a_y": float(u * r), "R": float(abs(u / r)), "beta": math.degrees(float(vy) / u)}


def _rand_corner(rng):
    x = _rand_bike(rng, 0.3)
    l_r, K = x["L"] - x["l_f"], x["m"] * ((x["L"] - x["l_f"]) / (x["L"] * x["C_f"]) - x["l_f"] / (x["L"] * x["C_r"]))
    umax = 40.0 if K >= 0 else 0.8 * math.sqrt(x["L"] / -K)
    x.update(u=rng.uniform(3, min(40.0, umax)), delta=math.radians(rng.choice([-1, 1]) * rng.uniform(0.3, 6)))
    return x


_STEER = _in("delta", "angle", "angle", "front-wheel steer angle (degrees in questions; rad in solve(), or '2 deg')",
             ("steer angle", "steering angle"), "deg")

proc("transport_steady_state_cornering", "Steady-state cornering: yaw rate, lateral acceleration, radius, sideslip",
     "vehicle dynamics",
     "Steady turning response of the linear bicycle model at speed u to a front-wheel steer angle delta: yaw rate, "
     "lateral acceleration, turn radius and the sideslip angle at the centre of gravity (small angles).",
     _BIKE_IN + [_V("u", "forward speed", ("speed",)), _STEER],
     [_o("r", "yaw_rate", "yaw rate (rad/s)"), _o("a_y", "acceleration", "lateral acceleration (m/s^2)"),
      _o("R", "length", "turn radius (m)"), _o("beta", "angle_deg", "sideslip angle at the centre of gravity (degrees)")],
     cornering, _ref_cornering,
     lambda m, L, l_f, C_f, C_r, u, delta: (lambda r: {"r": _r(r), "a_y": _r(u * r), "R": _r(abs(u / r)), "beta": 0.0})(u * delta / L),
     _rand_corner,
     [{"m": 1500, "L": 2.7, "l_f": 1.2, "C_f": 90000, "C_r": 85000, "u": 72, "delta": 2},
      {"m": 1300, "L": 2.5, "l_f": 1.4, "C_f": 60000, "C_r": 95000, "u": 54, "delta": 3}],
     ["steady state cornering", "steady-state yaw rate", "steady state turning response"],
     conditions=["below the critical speed", "delta != 0"],
     invalid=[{"m": 1500, "L": 2.7, "l_f": 1.2, "C_f": 90000, "C_r": 85000, "u": -72, "delta": 2}],
     method="r = u delta / (L + K u^2), a_y = u r, R = u / r, beta = delta (l_r - m l_f u^2 / (L C_r)) / (L + K u^2)",
     latex=r"\frac{r}{\delta} = \frac{u}{L + K u^2}")


def steer_for_radius(m, L, l_f, C_f, C_r, u, R):
    l_r, K = _bike_check(m, L, l_f, C_f, C_r)
    _pos(u=u, R=R)
    d = (L + K * u * u) / R
    _need(L + K * u * u > 0, "above the critical speed: no steady turn")
    return {"delta": _r(math.degrees(d))}


def _ref_steer(m, L, l_f, C_f, C_r, u, R):
    return {"delta": math.degrees((u / R) / _yaw_gain(m, L, l_f, C_f, C_r, u))}


def _rand_steer(rng):
    x = _rand_corner(rng)
    del x["delta"]
    x["R"] = rng.uniform(20, 800)
    return x


proc("transport_steer_angle_for_radius", "Steer angle needed for a turn of given radius (bicycle model)",
     "vehicle dynamics",
     "Front-wheel steer angle the linear bicycle model needs to hold a turn of radius R at speed u (Ackermann angle "
     "L/R plus the understeer term K u^2 / R).",
     _BIKE_IN + [_V("u", "forward speed", ("speed",)), _L("R", "turn radius", ("radius",))],
     [_o("delta", "angle_deg", "front-wheel steer angle (degrees)")],
     steer_for_radius, _ref_steer, lambda m, L, l_f, C_f, C_r, u, R: {"delta": _r(math.degrees(L / R))},
     _rand_steer,
     [{"m": 1500, "L": 2.7, "l_f": 1.2, "C_f": 90000, "C_r": 85000, "u": 72, "R": 150},
      {"m": 1300, "L": 2.5, "l_f": 1.4, "C_f": 60000, "C_r": 95000, "u": 54, "R": 80}],
     ["steer angle for a turn", "steering angle for a radius", "steer angle for radius"],
     conditions=["below the critical speed"], invalid=[{"m": 1500, "L": 2.7, "l_f": 1.2, "C_f": 90000, "C_r": 85000, "u": 72, "R": -150}],
     method="delta = L / R + K u^2 / R", latex=r"\delta = \frac{L}{R} + K\frac{u^2}{R}")


def yaw_modes(m, Iz, L, l_f, C_f, C_r, u):
    l_r, K = _bike_check(m, L, l_f, C_f, C_r)
    _pos(Iz=Iz, u=u)
    det = C_f * C_r * L * L / (m * Iz * u * u) * (1 + K * u * u / L)
    _need(det > 0, "above the critical speed: the yaw motion is unstable")
    tr = -(C_f + C_r) / (m * u) - (C_f * l_f ** 2 + C_r * l_r ** 2) / (Iz * u)
    wn = math.sqrt(det)
    return {"wn": _r(wn), "zeta": _r(-tr / (2 * wn)), "f_n": _r(wn / (2 * math.pi))}


def _ref_yaw_modes(m, Iz, L, l_f, C_f, C_r, u):
    import numpy as np
    p = np.linalg.eigvals(_bike_ss(m, L, l_f, C_f, C_r, u, Iz=Iz)[1])
    wn = math.sqrt(float((p[0] * p[1]).real))
    return {"wn": wn, "zeta": float(-(p[0] + p[1]).real / (2 * wn)), "f_n": wn / (2 * math.pi)}


def _rand_yaw(rng):
    x = _rand_corner(rng)
    del x["delta"]
    x["Iz"] = x["m"] * rng.uniform(0.8, 1.2) * x["l_f"] * (x["L"] - x["l_f"])
    return x


proc("transport_yaw_natural_frequency", "Yaw natural frequency and damping of the bicycle model", "vehicle dynamics",
     "Undamped natural frequency and damping ratio of the lateral-yaw motion of the linear bicycle model at speed u "
     "(from the characteristic equation of its two states), below the critical speed.",
     [_BIKE_IN[0], _in("Iz", "moment_of_inertia", "inertia", "yaw moment of inertia", ("yaw inertia",), "kg*m^2")] + _BIKE_IN[1:]
     + [_V("u", "forward speed", ("speed",))],
     [_o("wn", "angular_frequency", "undamped natural frequency (rad/s)"), _o("zeta", "damping_ratio", "damping ratio"),
      _o("f_n", "frequency", "undamped natural frequency (Hz)")],
     yaw_modes, _ref_yaw_modes,
     lambda m, Iz, L, l_f, C_f, C_r, u: (lambda o: dict(o, zeta=_r(o["zeta"] / 2)))(yaw_modes(m, Iz, L, l_f, C_f, C_r, u)),
     _rand_yaw,
     [{"m": 1500, "Iz": 2500, "L": 2.7, "l_f": 1.2, "C_f": 90000, "C_r": 85000, "u": 72},
      {"m": 1300, "Iz": 1900, "L": 2.5, "l_f": 1.4, "C_f": 60000, "C_r": 95000, "u": 54}],
     ["yaw natural frequency", "yaw damping ratio", "bicycle model natural frequency"],
     conditions=["below the critical speed"], invalid=[{"m": 1500, "Iz": -2500, "L": 2.7, "l_f": 1.2, "C_f": 90000, "C_r": 85000, "u": 72}],
     method="wn^2 = C_f C_r L^2 / (m Iz u^2) (1 + K u^2 / L), 2 zeta wn = (C_f + C_r)/(m u) + (C_f l_f^2 + C_r l_r^2)/(Iz u)",
     latex=r"\omega_n^2 = \frac{C_f C_r L^2}{m I_z u^2}\left(1 + \frac{K u^2}{L}\right)")


# =============================================================================================== quarter-car ride model
def quarter_car_freq(m_s, m_u, k_s, k_t):
    _pos(m_s=m_s, m_u=m_u, k_s=k_s, k_t=k_t)
    a, b, c = m_s * m_u, -(k_s * m_u + (k_s + k_t) * m_s), k_s * k_t
    disc = math.sqrt(b * b - 4 * a * c)
    hi = (-b + disc) / (2 * a)
    lo = c / (a * hi)
    return {"f_body": _r(math.sqrt(lo) / (2 * math.pi)), "f_wheel": _r(math.sqrt(hi) / (2 * math.pi))}


def _ref_quarter_freq(m_s, m_u, k_s, k_t):
    import numpy as np
    from scipy.linalg import eigh
    w2 = eigh(np.array([[k_s, -k_s], [-k_s, k_s + k_t]]), np.diag([m_s, m_u]), eigvals_only=True)
    return {"f_body": math.sqrt(w2[0]) / (2 * math.pi), "f_wheel": math.sqrt(w2[1]) / (2 * math.pi)}


_QC = [_M("m_s", "sprung mass (a quarter of the body)", ("sprung mass",)), _M("m_u", "unsprung mass (wheel)", ("unsprung mass",)),
       _in("k_s", "spring_constant", "stiffness", "suspension spring stiffness", ("spring stiffness", "suspension stiffness"), "kN/m")]
_KT = _in("k_t", "spring_constant", "stiffness", "tyre vertical stiffness", ("tyre stiffness", "tire stiffness"), "kN/m")

proc("transport_quarter_car_frequencies", "Quarter-car model: body and wheel-hop natural frequencies", "vehicle dynamics",
     "Undamped natural frequencies of the two-degree-of-freedom quarter-car ride model: the body (bounce) mode and the "
     "wheel-hop mode, from the sprung and unsprung masses, suspension and tyre stiffnesses.",
     _QC + [_KT],
     [_o("f_body", "frequency", "body (bounce) natural frequency (Hz)"), _o("f_wheel", "frequency", "wheel-hop natural frequency (Hz)")],
     quarter_car_freq, _ref_quarter_freq,
     lambda m_s, m_u, k_s, k_t: {"f_body": _r(math.sqrt(k_s / m_s) / (2 * math.pi)),
                                 "f_wheel": _r(math.sqrt((k_s + k_t) / m_u) / (2 * math.pi))},
     lambda rng: {"m_s": rng.uniform(200, 600), "m_u": rng.uniform(25, 70), "k_s": rng.uniform(15e3, 60e3), "k_t": rng.uniform(150e3, 320e3)},
     [{"m_s": 400, "m_u": 45, "k_s": 25, "k_t": 200}, {"m_s": 300, "m_u": 35, "k_s": 18, "k_t": 180}],
     ["quarter car natural frequencies", "quarter-car frequencies", "body and wheel hop frequencies"],
     conditions=["all inputs positive"], invalid=[{"m_s": 400, "m_u": -45, "k_s": 25, "k_t": 200}],
     method="roots of m_s m_u w^4 - (k_s m_u + (k_s + k_t) m_s) w^2 + k_s k_t = 0", latex=r"m_s m_u\omega^4 - \left[k_s m_u + (k_s + k_t) m_s\right]\omega^2 + k_s k_t = 0")


def quarter_car_tr(m_s, m_u, k_s, c_s, k_t, f):
    _pos(m_s=m_s, m_u=m_u, k_s=k_s, k_t=k_t, f=f)
    _nonneg(c_s=c_s)
    w = 2 * math.pi * f
    z = complex(k_s, w * c_s)
    a11, a12, a22 = z - w * w * m_s, -z, z + k_t - w * w * m_u
    det = a11 * a22 - a12 * a12
    _need(abs(det) > 0, "the frequency is exactly at an undamped resonance")
    xs = -a12 * k_t / det
    xu = a11 * k_t / det
    return {"T_body": _r(abs(xs)), "T_wheel": _r(abs(xu))}


def _ref_quarter_tr(m_s, m_u, k_s, c_s, k_t, f):
    import control
    import numpy as np
    A = np.array([[0, 0, 1, 0], [0, 0, 0, 1],
                  [-k_s / m_s, k_s / m_s, -c_s / m_s, c_s / m_s],
                  [k_s / m_u, -(k_s + k_t) / m_u, c_s / m_u, -c_s / m_u]])
    B = np.array([[0.0], [0.0], [0.0], [k_t / m_u]])
    C = np.array([[1.0, 0, 0, 0], [0, 1.0, 0, 0]])
    sysm = control.ss(A, B, C, np.zeros((2, 1)))
    H = np.asarray(sysm(1j * 2 * math.pi * f)).reshape(-1)
    return {"T_body": float(abs(H[0])), "T_wheel": float(abs(H[1]))}


proc("transport_quarter_car_transmissibility", "Quarter-car model: road-to-body transmissibility at a frequency",
     "vehicle dynamics",
     "Amplitude ratio between body (sprung-mass) motion and road displacement, and between wheel motion and road "
     "displacement, of the damped quarter-car model for a sinusoidal road input of frequency f.",
     _QC + [_in("c_s", "damping", "damping", "suspension damping coefficient", ("damping", "damper"), "N*s/m"), _KT,
            _in("f", "frequency", "frequency", "road input frequency", ("frequency",), "Hz")],
     [_o("T_body", "number", "body displacement / road displacement"), _o("T_wheel", "number", "wheel displacement / road displacement")],
     quarter_car_tr, _ref_quarter_tr,
     lambda m_s, m_u, k_s, c_s, k_t, f: (lambda w: {"T_body": _r(abs(complex(k_s, w * c_s) / complex(k_s - w * w * m_s, w * c_s))),
                                                   "T_wheel": 1.0})(2 * math.pi * f),
     lambda rng: {"m_s": rng.uniform(200, 600), "m_u": rng.uniform(25, 70), "k_s": rng.uniform(15e3, 60e3),
                  "c_s": rng.uniform(800, 4000), "k_t": rng.uniform(150e3, 320e3), "f": rng.uniform(0.3, 20)},
     [{"m_s": 400, "m_u": 45, "k_s": 25, "c_s": 1800, "k_t": 200, "f": 1.2}, {"m_s": 300, "m_u": 35, "k_s": 18, "c_s": 1200, "k_t": 180, "f": 10}],
     ["quarter car transmissibility", "quarter-car transmissibility", "ride transmissibility"],
     conditions=["masses and stiffnesses positive", "c_s >= 0"], invalid=[{"m_s": 400, "m_u": 45, "k_s": 25, "c_s": -1800, "k_t": 200, "f": 1.2}],
     method="solve the 2x2 complex dynamic-stiffness system [K + i w C - w^2 M] Z = [0, k_t] for the road input",
     latex=r"\left(K + i\omega C - \omega^2 M\right)\mathbf Z = \begin{bmatrix}0\\ k_t\end{bmatrix} Z_r")


# =============================================================================================== definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Webster optimum
# cycle length, the simplified e + f curve formula, sag-curve headlight sight distance, vehicle resistance forces and
# power, top speed, passing sight distance, travel-time reliability indices.

def _guard(fn):
    """A wrong variant that raises cannot be compared: return a failed marker instead."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


# Outputs of the documented examples of these additions, computed by their references on the build machine and
# rounded to 12 significant digits.
EXAMPLE_OUTPUTS.update({
    "transport_webster_optimum_cycle": [{"C0": 76.6666666667, "Y": 0.7, "C_min": 40.0, "green_total": 64.6666666667}, {"C0": 56.6666666667, "Y": 0.7, "C_min": 26.6666666667, "green_total": 48.6666666667}],
    "transport_simplified_curve_formula": [{"R": 652.196829539, "v": 27.7777777778, "v_kmh": 100.0}, {"R": 250.0, "v": 19.2311466117, "v_kmh": 69.2321278021}],
    "transport_sag_curve_headlight": [{"L": 222.964169381, "A": 5.0, "K": 44.5928338762}, {"L": 105.0, "A": 4.0, "K": 26.25}],
    "transport_vehicle_resistance": [{"F_roll": 220.725, "F_aero": 305.555555556, "F_grade": 0.0, "F_total": 526.280555556, "P": 14618.904321, "P_kW": 14.618904321}, {"F_roll": 1569.59992937, "F_aero": 1600.0, "F_grade": 58.8599973513, "F_total": 3228.45992672, "P": 71743.5539271, "P_kW": 71.7435539271}],
    "transport_top_speed": [{"v_max": 60.2702125482, "v_max_kmh": 216.972765173}, {"v_max": 39.8999180785, "v_max_kmh": 143.639705083}],
    "transport_passing_sight_distance": [{"PSD": 588.833466667, "d1": 95.5001333333, "d2": 260.0, "d4": 173.333333333}, {"PSD": 336.415733333, "d1": 48.0824, "d2": 155.0, "d4": 103.333333333}],
    "transport_travel_time_reliability": [{"mean": 26.4, "t95": 32.75, "buffer_index": 0.24053030303, "planning_time_index": 1.81944444444, "travel_time_index": 1.46666666667}, {"mean": 14.3333333333, "t95": 18.75, "buffer_index": 0.308139534884, "planning_time_index": 1.875, "travel_time_index": 1.43333333333}],
})


# --- Webster optimum cycle length -------------------------------------------------------------------------------------
def webster_cycle(L, y):
    _pos(L=L)
    _need(isinstance(y, (list, tuple)) and len(y) >= 2, "give the critical flow ratio of at least two phases")
    for v in y:
        _need(isinstance(v, (int, float)) and 0 < v < 1, "each flow ratio must lie between 0 and 1")
    Y = sum(y)
    _need(Y < 0.95, "the sum of the critical flow ratios must be below 0.95 (the intersection is near or over capacity)")
    C0 = (1.5 * L + 5) / (1 - Y)
    return {"C0": _r(C0), "Y": _r(Y), "C_min": _r(L / (1 - Y)), "green_total": _r(C0 - L)}


def _ref_webster_cycle(L, y):
    # Webster's expression in exact rational arithmetic (sympy) from the inputs as written
    import sympy as sp
    Ls = sp.Rational(repr(L))
    Y = sum(sp.Rational(repr(v)) for v in y)
    C0 = (sp.Rational(3, 2) * Ls + 5) / (1 - Y)
    return {"C0": float(C0), "Y": float(Y), "C_min": float(Ls / (1 - Y)), "green_total": float(C0 - Ls)}


proc("transport_webster_optimum_cycle", "Optimum cycle length by Webster's formula", "traffic signals",
     "Webster's optimum cycle length C0 = (1.5 L + 5)/(1 - Y) of a fixed-time signal from the total lost time L (s) and "
     "the critical flow ratios y_i = v_i/s_i (Y = sum y_i), with the minimum cycle L/(1 - Y) and the total effective "
     "green. Times in s.",
     [_T("L", "total lost time per cycle", ("lost time",)), _VEC("y", "critical flow ratio v/s of each phase", ("flow ratios", "y"), min_length=2)],
     [_o("C0", "time", "Webster optimum cycle length (s)"), _o("Y", "number", "sum of the critical flow ratios"),
      _o("C_min", "time", "minimum cycle length L/(1 - Y) (s)"), _o("green_total", "time", "total effective green C0 - L (s)")],
     webster_cycle, _ref_webster_cycle,
     _guard(lambda L, y: (lambda Y: {"C0": _r(1.5 * L / (1 - Y)), "Y": _r(Y), "C_min": _r(L / (1 - Y)), "green_total": _r(1.5 * L / (1 - Y) - L)})(sum(y))),
     lambda rng: (lambda n: {"L": rng.uniform(6, 20), "y": [round(rng.uniform(0.05, 0.85 / n), 3) for _ in range(n)]})(rng.randint(2, 5)),
     [{"L": 12, "y": [0.30, 0.25, 0.15]}, {"L": 8, "y": [0.4, 0.3]}],
     ["webster optimum cycle length", "webster optimum cycle", "webster cycle length", "optimum cycle length"],
     conditions=["0 < y_i < 1", "Y < 0.95", "fixed-time signal, Webster's delay model"], invalid=[{"L": 12, "y": [0.5, 0.5]}],
     method="C0 = (1.5 L + 5)/(1 - Y), Y = sum of critical y_i", latex=r"C_0 = \frac{1.5L + 5}{1 - Y}")


# --- simplified e + f curve formula -----------------------------------------------------------------------------------
def simplified_curve(e, f, v=None, R=None):
    _finite(e=e, f=f)
    _need(-0.1 <= e <= 0.2, "the superelevation must lie between -10% and 20%")
    _need(0 < f <= 0.5, "the side friction factor must lie in (0, 0.5]")
    _need(e + f > 0, "e + f must be positive")
    _need((v is None) != (R is None), "give the speed or the radius (one of them)")
    if v is not None:
        _pos(v=v)
        Rm = v * v / (G * (e + f))
        return {"R": _r(Rm), "v": _r(v), "v_kmh": _r(v * 3.6)}
    _pos(R=R)
    vm = math.sqrt(G * R * (e + f))
    return {"R": _r(R), "v": _r(vm), "v_kmh": _r(vm * 3.6)}


def _ref_simplified_curve(e, f, v=None, R=None):
    # the point-mass balance of forces along the radius with the small-angle simplification (normal force m g, lateral
    # m g (e + f)) solved for the unknown by brentq
    if v is not None:
        Rm = _brentq(lambda r: v * v / r - G * (e + f), 1e-6, 1e9)
        return {"R": Rm, "v": v, "v_kmh": v * 3.6}
    vm = _brentq(lambda s: s * s / R - G * (e + f), 0.0, 1e4)
    return {"R": R, "v": vm, "v_kmh": vm * 3.6}


proc("transport_simplified_curve_formula", "Curve radius or speed by the simplified e + f formula", "highway geometry",
     "The minimum radius R = v^2/(g (e + f)) for a speed, or the maximum speed sqrt(g R (e + f)) on a radius, by the "
     "simplified (small-angle) point-mass formula of horizontal curves (the exact form is minimum curve radius).",
     [_in("e", "grade", "grade", "superelevation rate, decimal", ("superelevation",), "%"),
      _N("f", "side friction factor", ("side friction",), kind="friction_coefficient"),
      _V("v", "design speed (give v or R)", ("speed", "design speed"), required=False),
      _L("R", "curve radius (give v or R)", ("radius",), required=False)],
     [_o("R", "length", "curve radius (m)"), _o("v", "speed", "speed (m/s)"), _o("v_kmh", "number", "speed (km/h)")],
     simplified_curve, _ref_simplified_curve,
     _guard(lambda e, f, v=None, R=None: (lambda o: dict(o, R=_r(v * v / (G * (e + f)) * (1 - e * f)) if v is not None else o["R"],
                                                       v=_r(math.sqrt(G * R * (e + f) / (1 - e * f))) if R is not None else o["v"],
                                                       v_kmh=_r(math.sqrt(G * R * (e + f) / (1 - e * f)) * 3.6) if R is not None else o["v_kmh"]))(simplified_curve(e, f, v, R))),
     lambda rng: dict({"e": rng.uniform(0.02, 0.12), "f": rng.uniform(0.08, 0.2)},
                      **({"v": rng.uniform(30, 130) * KMH} if rng.random() < 0.5 else {"R": rng.uniform(50, 2000)})),
     [{"e": 0.06, "f": 0.12, "v": 100}, {"e": 0.08, "f": 0.15, "R": 250}],
     ["simplified curve formula", "curve radius by the simplified formula", "simplified e plus f formula"],
     conditions=["-0.1 <= e <= 0.2", "0 < f <= 0.5", "the speed or the radius (one of them)", "small-angle (simplified) form"],
     invalid=[{"e": 0.06, "f": 0.9, "v": 100}],
     method="R = v^2/(g (e + f)); v = sqrt(g R (e + f))", latex=r"R = \frac{v^2}{g\,(e + f)}")


# --- sag curve headlight sight distance -------------------------------------------------------------------------------
def sag_headlight(g1, g2, S):
    _finite(g1=g1, g2=g2)
    _pos(S=S)
    A = (g2 - g1) * 100.0
    _need(A > 0, "a sag curve needs g2 > g1 (the grade increases)")
    _need(S <= 2000, "the sight distance must be at most 2000 m")
    Ls = A * S * S / (120 + 3.5 * S)
    if Ls >= S:
        L, case = Ls, "S < L"
    else:
        L, case = max(0.0, 2 * S - (120 + 3.5 * S) / A), "S > L"
    return {"L": _r(L), "A": _r(A), "K": _r(L / A), "case": case}


def _ref_sag_headlight(g1, g2, S):
    # the curve length at which the headlight beam (0.6 m high, rising at 1 in 57.14, i.e. the 3.5/200 slope of the
    # AASHTO coefficients) just meets the road S ahead: brentq on the parabola's offset against the beam height
    A = (g2 - g1) * 100.0
    beam = 0.6 + 0.0175 * S
    try:
        L = _brentq(lambda Lc: A * S * S / (200 * Lc) - beam, S, 1e9)    # S within the curve
        if L >= S:
            return {"L": L, "A": A, "K": L / A}
    except ValueError:
        pass
    # S beyond the curve: offset of the second tangent at S from the start, A (S - L/2)/100, equals the beam height
    if A * S / 100 - beam <= 0:
        return {"L": 0.0, "A": A, "K": 0.0}                           # no curve needed
    L = _brentq(lambda Lc: A * (S - Lc / 2) / 100 - beam, 0.0, 2 * S)
    return {"L": L, "A": A, "K": L / A}


proc("transport_sag_curve_headlight", "Sag vertical curve length for headlight sight distance", "highway geometry",
     "The minimum length of a sag vertical curve for the headlight sight distance S by the AASHTO formulas (headlight "
     "0.6 m high, beam rising 1 degree): L = A S^2/(120 + 3.5 S) when S < L, L = 2S - (120 + 3.5 S)/A when S > L, A = "
     "g2 - g1 in percent.",
     [_GR("g1", "entering grade, decimal", ("g1", "entering grade"), required=True),
      _GR("g2", "leaving grade, decimal", ("g2", "leaving grade"), required=True),
      _L("S", "headlight sight distance", ("sight distance",))],
     [_o("L", "length", "minimum curve length (m)"), _o("A", "number", "algebraic grade difference (%)"),
      _o("K", "number", "rate of vertical curvature L/A (m per %)"), {"symbol": "case", "kind": "number", "meaning": "S < L or S > L"}],
     sag_headlight, _ref_sag_headlight,
     _guard(lambda g1, g2, S: (lambda A: (lambda Ls: {"L": _r(Ls), "A": _r(A), "K": _r(Ls / A), "case": "S < L"})(A / 100 * S * S / (120 + 3.5 * S)))((g2 - g1) * 100)),
     lambda rng: (lambda g1: {"g1": g1, "g2": g1 + rng.uniform(0.01, 0.1), "S": rng.uniform(60, 300)})(rng.uniform(-0.07, 0.02)),
     [{"g1": -3, "g2": 2, "S": 185}, {"g1": -2, "g2": 2, "S": 120}],
     ["sag vertical curve length", "sag curve length for headlight sight distance", "headlight sight distance curve length", "sag curve length"],
     conditions=["g2 > g1 (a sag)", "AASHTO headlight height 0.6 m and 1 degree upward beam (coefficients 120 and 3.5)"],
     invalid=[{"g1": 2, "g2": -3, "S": 185}],
     method="S < L: L = A S^2/(120 + 3.5 S); S > L: L = 2S - (120 + 3.5 S)/A",
     latex=r"L = \frac{A S^2}{120 + 3.5 S}\ (S < L),\quad L = 2S - \frac{120 + 3.5S}{A}\ (S > L)")


# --- vehicle resistance forces and power ------------------------------------------------------------------------------
def _resist(m, v, C_D, A, rho, f_r, G_):
    th = math.atan(G_)
    Fr = f_r * m * G * math.cos(th)
    Fa = 0.5 * rho * C_D * A * v * v
    Fg = m * G * math.sin(th)
    return Fr, Fa, Fg


def vehicle_resistance(m, v, C_D, A, rho=1.2, f_r=0.015, G=0.0):
    _pos(m=m, C_D=C_D, A=A, rho=rho)
    _finite(v=v, f_r=f_r, G=G)
    _need(v >= 0, "the speed must be zero or positive")
    _need(0 <= f_r <= 0.1, "the rolling resistance coefficient must lie between 0 and 0.1")
    _need(abs(G) <= 0.3, "the grade must lie between -30% and 30%")
    Fr, Fa, Fg = _resist(m, v, C_D, A, rho, f_r, G)
    F = Fr + Fa + Fg
    return {"F_roll": _r(Fr), "F_aero": _r(Fa), "F_grade": _r(Fg), "F_total": _r(F), "P": _r(F * v), "P_kW": _r(F * v / 1e3)}


def _ref_vehicle_resistance(m, v, C_D, A, rho=1.2, f_r=0.015, G=0.0):
    # the weight resolved on the grade by a rotation of the vector (0, -m g) (numpy), the drag as the momentum given to
    # the swept air per unit time (rho A v^2 C_D/2), power as the work over one second of travel (quadrature)
    import numpy as np
    th = math.atan(G)
    W = np.array([0.0, -m * 9.81])
    along = np.array([math.cos(th), math.sin(th)])
    normal = np.array([-math.sin(th), math.cos(th)])
    Fg = -float(W @ along)
    N = -float(W @ normal)
    Fr = f_r * N
    Fa = 0.5 * rho * C_D * A * v * v
    F = Fr + Fa + Fg
    P = _quad(lambda t: F * v, 0.0, 1.0) if v > 0 else 0.0
    return {"F_roll": Fr, "F_aero": Fa, "F_grade": Fg, "F_total": F, "P": P, "P_kW": P / 1e3}


_RES_IN = [_M("m", "vehicle mass", ("mass",)), _N("C_D", "drag coefficient", ("drag coefficient",), kind="drag_coefficient"),
           _in("A", "area", "area", "frontal area", ("frontal area",), "m^2"),
           _in("rho", "density", "air_density", "air density (default 1.2 kg/m^3)", ("air density",), "kg/m^3", required=False),
           _N("f_r", "rolling resistance coefficient (default 0.015)", ("rolling resistance",), required=False), _GR()]

proc("transport_vehicle_resistance", "Vehicle resistance forces and power at a speed", "vehicle performance",
     "Rolling resistance f_r m g cos(theta), aerodynamic drag rho C_D A v^2/2 and grade resistance m g sin(theta) "
     "(tan theta = G) of a road vehicle at a speed, their sum and the power needed to overcome them.",
     [_RES_IN[0], _V("v", "speed", ("speed",))] + _RES_IN[1:],
     [_o("F_roll", "force", "rolling resistance (N)"), _o("F_aero", "force", "aerodynamic drag (N)"), _o("F_grade", "force", "grade resistance (N)"),
      _o("F_total", "force", "total resistance (N)"), _o("P", "power", "power to overcome the resistance (W)"), _o("P_kW", "number", "power (kW)")],
     vehicle_resistance, _ref_vehicle_resistance,
     _guard(lambda m, v, C_D, A, rho=1.2, f_r=0.015, G=0.0: (lambda Fr, Fa, Fg: {"F_roll": _r(Fr), "F_aero": _r(Fa), "F_grade": _r(Fg), "F_total": _r(Fr + Fa + Fg),
                                                                                "P": _r((Fr + Fa + Fg) * v), "P_kW": _r((Fr + Fa + Fg) * v / 1e3)})(
         f_r * m * 9.81, rho * C_D * A * v * v, m * 9.81 * G)),
     lambda rng: {"m": rng.uniform(800, 40000), "v": rng.uniform(5, 40), "C_D": rng.uniform(0.25, 0.8), "A": rng.uniform(1.8, 10),
                  **({"rho": rng.uniform(1.0, 1.3)} if rng.random() < 0.3 else {}), **({"f_r": rng.uniform(0.006, 0.03)} if rng.random() < 0.5 else {}),
                  **({"G": rng.uniform(-0.06, 0.08)} if rng.random() < 0.6 else {})},
     [{"m": 1500, "v": 100, "C_D": 0.3, "A": 2.2}, {"m": 20000, "v": 80, "C_D": 0.6, "A": 9, "f_r": 0.008, "G": 0.03}],
     ["vehicle resistance forces", "resistance forces on a vehicle", "vehicle resistance", "road load"],
     conditions=["m, C_D, A, rho > 0", "v >= 0", "0 <= f_r <= 0.1", "|G| <= 0.3", "still air"],
     invalid=[{"m": 1500, "v": 100, "C_D": -0.3, "A": 2.2}],
     method="F = f_r m g cos(theta) + rho C_D A v^2/2 + m g sin(theta), tan(theta) = G; P = F v",
     latex=r"F = f_r m g\cos\theta + \tfrac12\rho C_D A v^2 + m g\sin\theta,\quad P = F v")


# --- top speed --------------------------------------------------------------------------------------------------------
def top_speed(m, P, C_D, A, rho=1.2, f_r=0.015, G=0.0, eta=1.0):
    _pos(m=m, P=P, C_D=C_D, A=A, rho=rho)
    _finite(f_r=f_r, G=G, eta=eta)
    _need(0 <= f_r <= 0.1 and abs(G) <= 0.3, "need 0 <= f_r <= 0.1 and |G| <= 0.3")
    _need(0 < eta <= 1, "the drivetrain efficiency must lie in (0, 1]")
    th = math.atan(G)
    a = 0.5 * rho * C_D * A
    b = m * 9.81 * (f_r * math.cos(th) + math.sin(th))               # (G here is the grade; gravity is 9.81 m/s^2)
    Pw = eta * P
    f = lambda v: a * v ** 3 + b * v - Pw                              # noqa: E731
    lo = math.sqrt(-b / (3 * a)) if b < 0 else 0.0                    # downhill: f falls until here, so the root lies beyond
    hi = max(1.0, 2 * lo, (Pw / a) ** (1 / 3))
    while f(hi) < 0:
        hi *= 2
    a_, b_ = lo, hi
    for _ in range(300):
        mid = 0.5 * (a_ + b_)
        if mid in (a_, b_):
            break
        if f(mid) < 0:
            a_ = mid
        else:
            b_ = mid
    v = 0.5 * (a_ + b_)
    _need(v > 0.5, "no steady top speed (the power does not overcome the resistance)")
    return {"v_max": _r(v), "v_max_kmh": _r(v * 3.6)}


def _ref_top_speed(m, P, C_D, A, rho=1.2, f_r=0.015, G=0.0, eta=1.0):
    # the power balance a v^3 + b v - eta P = 0 as a cubic polynomial solved by numpy.roots (companion-matrix
    # eigenvalues): the largest real positive root
    import numpy as np
    th = math.atan(G)
    a = 0.5 * rho * C_D * A
    b = m * 9.81 * (f_r * math.cos(th) + math.sin(th))
    r = np.roots([a, 0.0, b, -eta * P])
    v = max(float(z.real) for z in r if abs(z.imag) <= 1e-9 * max(1.0, abs(z.real)) and z.real > 0)
    # polish on the same cubic (one Newton step keeps the root of the polynomial, removes eigen-solver rounding)
    for _ in range(3):
        v -= (a * v ** 3 + b * v - eta * P) / (3 * a * v * v + b)
    return {"v_max": v, "v_max_kmh": v * 3.6}


proc("transport_top_speed", "Top speed of a road vehicle", "vehicle performance",
     "The steady top speed at which the power at the wheels eta P equals the power absorbed by rolling, aerodynamic and "
     "grade resistance: rho C_D A v^3/2 + m g (f_r cos(theta) + sin(theta)) v = eta P.",
     [_RES_IN[0], _P("P", "engine power", ("power",))] + _RES_IN[1:] +
     [_N("eta", "drivetrain efficiency (default 1)", ("efficiency", "drivetrain efficiency"), kind="efficiency", required=False)],
     [_o("v_max", "speed", "top speed (m/s)"), _o("v_max_kmh", "number", "top speed (km/h)")],
     top_speed, _ref_top_speed,
     _guard(lambda m, P, C_D, A, rho=1.2, f_r=0.015, G=0.0, eta=1.0: (lambda v: {"v_max": _r(v), "v_max_kmh": _r(v * 3.6)})((2 * eta * P / (rho * C_D * A)) ** (1 / 3))),
     lambda rng: {"m": rng.uniform(800, 40000), "P": rng.uniform(40, 400) * 1e3, "C_D": rng.uniform(0.25, 0.8), "A": rng.uniform(1.8, 10),
                  **({"rho": rng.uniform(1.0, 1.3)} if rng.random() < 0.3 else {}), **({"f_r": rng.uniform(0.006, 0.03)} if rng.random() < 0.5 else {}),
                  **({"G": rng.uniform(0.0, 0.06)} if rng.random() < 0.5 else {}), **({"eta": rng.uniform(0.8, 0.98)} if rng.random() < 0.5 else {})},
     [{"m": 1500, "P": 100, "C_D": 0.3, "A": 2.2}, {"m": 20000, "P": 300, "C_D": 0.6, "A": 9, "f_r": 0.008, "G": 0.02, "eta": 0.9}],
     ["top speed of a road vehicle", "maximum speed of a vehicle", "vehicle top speed", "top speed"],
     conditions=["m, P, C_D, A, rho > 0", "0 <= f_r <= 0.1", "|G| <= 0.3", "0 < eta <= 1", "power-limited (no gearing or traction limit)"],
     invalid=[{"m": 1500, "P": 100, "C_D": 0.3, "A": 2.2, "eta": 1.5}],
     method="solve rho C_D A v^3/2 + m g (f_r cos(theta) + sin(theta)) v = eta P by bisection",
     latex=r"\tfrac12\rho C_D A v^3 + m g\left(f_r\cos\theta + \sin\theta\right)v = \eta P")


# --- passing sight distance -------------------------------------------------------------------------------------------
def passing_sight(v, m_diff, a, t1, t2, d3):
    _pos(v=v, a=a, t1=t1, t2=t2)
    _finite(m_diff=m_diff, d3=d3)
    _need(0 < m_diff < v, "the speed difference must be positive and below the passing speed")
    _need(d3 >= 0, "the clearance distance must be zero or positive")
    d1 = t1 * (v - m_diff + a * t1 / 2)
    d2 = v * t2
    d4 = 2 * d2 / 3
    return {"PSD": _r(d1 + d2 + d3 + d4), "d1": _r(d1), "d2": _r(d2), "d3": _r(d3), "d4": _r(d4)}


def _ref_passing_sight(v, m_diff, a, t1, t2, d3):
    # the passing manoeuvre simulated (scipy DOP853): initial manoeuvre from the impeded speed v - m accelerating at a
    # for t1, then the occupancy of the left lane at speed v for t2; the opposing vehicle travels 2/3 of d2
    from scipy.integrate import solve_ivp
    s1 = solve_ivp(lambda t, y: [y[1], a], (0, t1), [0.0, v - m_diff], method="DOP853", rtol=1e-13, atol=1e-12).y[0, -1]
    s2 = solve_ivp(lambda t, y: [v], (0, t2), [0.0], method="DOP853", rtol=1e-13, atol=1e-12).y[0, -1]
    d4 = 2 * s2 / 3
    return {"PSD": s1 + s2 + d3 + d4, "d1": s1, "d2": s2, "d4": d4}


proc("transport_passing_sight_distance", "Passing sight distance (four-component method)", "highway geometry",
     "The passing sight distance on a two-lane road as d1 + d2 + d3 + d4: the initial manoeuvre d1 = t1 (v - m + a t1/2), "
     "the time in the left lane d2 = v t2, the clearance d3 and the opposing vehicle's travel d4 = 2 d2/3 (AASHTO 2001).",
     [_V("v", "average speed of the passing vehicle", ("speed", "passing speed")),
      _V("m_diff", "speed difference between passing and passed vehicles", ("speed difference",)),
      _A("a", "average acceleration during the initial manoeuvre", ("acceleration",)),
      _T("t1", "time of the initial manoeuvre", ("initial manoeuvre time", "t1")), _T("t2", "time in the left lane", ("left lane time", "t2")),
      _L("d3", "clearance between the passing and opposing vehicles", ("clearance",))],
     [_o("PSD", "length", "passing sight distance (m)"), _o("d1", "length", "initial manoeuvre distance (m)"),
      _o("d2", "length", "distance in the left lane (m)"), _o("d3", "length", "clearance (m)"), _o("d4", "length", "opposing vehicle distance (m)")],
     passing_sight, _ref_passing_sight,
     _guard(lambda v, m_diff, a, t1, t2, d3: (lambda d1, d2: {"PSD": _r(d1 + d2 + d3 + d2), "d1": _r(d1), "d2": _r(d2), "d3": _r(d3), "d4": _r(d2)})(
         t1 * (v - m_diff + a * t1 / 2), v * t2)),
     lambda rng: (lambda v: {"v": v, "m_diff": rng.uniform(10, 20) * KMH, "a": rng.uniform(0.6, 0.7), "t1": rng.uniform(3.5, 4.5), "t2": rng.uniform(9, 11.5),
                             "d3": rng.uniform(30, 90)})(rng.uniform(50, 110) * KMH),
     [{"v": 90, "m_diff": 15, "a": 0.64, "t1": 4.3, "t2": 10.4, "d3": 60}, {"v": 60, "m_diff": 16, "a": 0.63, "t1": 3.6, "t2": 9.3, "d3": 30}],
     ["passing sight distance", "overtaking sight distance", "four component passing sight distance"],
     conditions=["0 < m < v", "a, t1, t2 > 0", "d3 >= 0", "AASHTO 2001 four-component model"],
     invalid=[{"v": 90, "m_diff": 100, "a": 0.64, "t1": 4.3, "t2": 10.4, "d3": 60}],
     method="d1 = t1 (v - m + a t1/2), d2 = v t2, d4 = 2 d2/3, PSD = d1 + d2 + d3 + d4",
     latex=r"PSD = t_1\left(v - m + \frac{a t_1}{2}\right) + v t_2 + d_3 + \frac23 v t_2")


# --- travel-time reliability ------------------------------------------------------------------------------------------
def _pctl(xs, p):
    """Percentile by linear interpolation between closest ranks (the 'linear' rule, Hyndman-Fan type 7)."""
    s = sorted(xs)
    h = (len(s) - 1) * p / 100.0
    lo = math.floor(h)
    hi = min(lo + 1, len(s) - 1)
    return s[lo] + (h - lo) * (s[hi] - s[lo])


def reliability(times, t_free):
    _need(isinstance(times, (list, tuple)) and len(times) >= 5, "give at least five travel times")
    for t in times:
        _need(isinstance(t, (int, float)) and t > 0, "every travel time must be positive")
    _pos(t_free=t_free)
    mean = sum(times) / len(times)
    t95 = _pctl(times, 95)
    return {"mean": _r(mean), "t95": _r(t95), "buffer_index": _r((t95 - mean) / mean), "planning_time_index": _r(t95 / t_free),
            "travel_time_index": _r(mean / t_free)}


def _ref_reliability(times, t_free):
    import numpy as np
    x = np.asarray(times, float)
    mean = float(np.mean(x))
    t95 = float(np.percentile(x, 95, method="linear"))
    return {"mean": mean, "t95": t95, "buffer_index": (t95 - mean) / mean, "planning_time_index": t95 / t_free, "travel_time_index": mean / t_free}


proc("transport_travel_time_reliability", "Travel-time reliability indices (buffer, planning time, travel time index)", "traffic flow",
     "The buffer index (T95 - mean)/mean, the planning time index T95/T_free and the travel time index mean/T_free of "
     "observed travel times on a route (T95 the 95th percentile by linear interpolation). Times in minutes.",
     [_VEC("times", "observed travel times in min", ("travel times",), min_length=5),
      _in("t_free", "number", "minutes", "free-flow travel time in min", ("free flow time", "free-flow time"), "min")],
     [_o("mean", "number", "mean travel time (min)"), _o("t95", "number", "95th-percentile travel time (min)"),
      _o("buffer_index", "number", "buffer index (T95 - mean)/mean"), _o("planning_time_index", "number", "planning time index T95/T_free"),
      _o("travel_time_index", "number", "travel time index mean/T_free")],
     reliability, _ref_reliability,
     _guard(lambda times, t_free: (lambda o: dict(o, buffer_index=_r((o["t95"] - o["mean"]) / t_free)))(reliability(times, t_free))),
     lambda rng: (lambda tf: {"t_free": round(tf, 2), "times": [round(tf * rng.uniform(1.0, 2.2), 2) for _ in range(rng.randint(8, 40))]})(rng.uniform(5, 60)),
     [{"times": [22, 25, 24, 30, 27, 23, 35, 26, 24, 28], "t_free": 18}, {"times": [12, 14, 13, 15, 20, 12], "t_free": 10}],
     ["travel time reliability indices", "travel time reliability", "buffer index", "planning time index"],
     conditions=["at least five positive travel times", "T95 by linear interpolation (type 7)"], invalid=[{"times": [22, 25, -24, 30, 27], "t_free": 18}],
     method="BI = (T95 - mean)/mean; PTI = T95/T_free; TTI = mean/T_free",
     latex=r"BI = \frac{T_{95} - \bar T}{\bar T},\quad PTI = \frac{T_{95}}{T_{free}},\quad TTI = \frac{\bar T}{T_{free}}")
