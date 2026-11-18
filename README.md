# Transportation (transportation-ai999x)

[![tests](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/transportation-ai999x)](https://pypi.org/project/transportation-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/transportation-ai999x)

Transportation covers vehicle dynamics, braking and stopping distances, traffic flow, highway geometry, railway operations, route planning and transit capacity: 81 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Transportation desktop app: Stopping sight distance (reaction plus braking, with grade), with its worked example and live scene](https://www.ai999x.com/?ax_shot=transportation-ai999x/hero.webp)](https://www.ai999x.com/models/transportation-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-transportation-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-transportation-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `transportation` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/transportation-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install transportation-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Transportation desktop app: every scene and chart is drawn from the computed answer.

[![Transportation desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=transportation-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=transportation-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=transportation-ai999x/clip.mp4)*

| | |
|---|---|
| [![Stopping sight distance (reaction plus braking, with grade): the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-1.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_stopping_sight_distance)<br>Stopping sight distance (reaction plus braking, with grade) | [![Queue and uniform delay at a signal (deterministic D/D/1): the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-2.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_signal_queue_uniform_delay)<br>Queue and uniform delay at a signal (deterministic D/D/1) |
| [![Fastest route on a road network (link lengths and speeds): the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-3.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_fastest_route)<br>Fastest route on a road network (link lengths and speeds) | [![Maximum safe speed on a superelevated horizontal curve: the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-4.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_curve_safe_speed)<br>Maximum safe speed on a superelevated horizontal curve |
| [![Understeer gradient and characteristic or critical speed (bicycle model): the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-5.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_understeer_gradient)<br>Understeer gradient and characteristic or critical speed (bicycle model) | [![Freeway level of service from density (basic segment): the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-6.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_freeway_level_of_service)<br>Freeway level of service from density (basic segment) |
| [![Direct and cross elasticities of logit choice probabilities: the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-7.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_logit_elasticities)<br>Direct and cross elasticities of logit choice probabilities | [![Equivalent single-axle loads by the fourth-power law: the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-8.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_equivalent_single_axle_loads)<br>Equivalent single-axle loads by the fourth-power law |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Stopping sight distance (reaction plus braking, with grade): a chart in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/chart-1.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_stopping_sight_distance)<br>Stopping sight distance (reaction plus braking, with grade) | [![Queue and uniform delay at a signal (deterministic D/D/1): a chart in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/chart-2.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_signal_queue_uniform_delay)<br>Queue and uniform delay at a signal (deterministic D/D/1) | [![Maximum safe speed on a superelevated horizontal curve: a chart in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/chart-3.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_curve_safe_speed)<br>Maximum safe speed on a superelevated horizontal curve | [![Direct and cross elasticities of logit choice probabilities: a chart in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/chart-4.webp)](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_logit_elasticities)<br>Direct and cross elasticities of logit choice probabilities |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import transportation_ai999x as transportation

a = transportation.ask(
    "Braking distance with speed = 100 km/h and deceleration = 3.4 "
    "m/s^2."
)
print(a.outcome, a.values)
# SOLVED {'d': 113.471314452, 't': 8.16993464052}

a = transportation.ask(
    "Clothoid coordinates with radius = 300 m and spiral length = 80 "
    "m."
)
print(a.outcome, a.values)
# SOLVED {'x': 79.8578947851, 'y': 3.55104311752, 'theta': 7.63943726841}

a = transportation.ask(
    "Station headway with line speed = 80 km/h, acceleration = 1 "
    "m/s^2, braking rate = 1.1 m/s^2, train length = 150 m, dwell time "
    "= 30 s and safety margin = 50 m."
)
print(a.outcome, a.values)
# SOLVED {'h': 70.202020202, 'trains_per_h': 51.2805755396}

a = transportation.ask(
    "Fastest route with roads = [A-B 4, A-C 2.5, B-C 1.2, B-D 5, C-D "
    "8, C-E 10, D-E 2], speeds = [60, 50, 40, 80, 100, 90, 50], origin "
    "= A and destination = E."
)
print(a.outcome, a.values)
# SOLVED {'route': 'A-C-E', 'time': 9.66666666667, 'distance': 12.5}

a = transportation.ask(
    "Design hourly volume with AADT = 40000 veh/day, K = 10 % and D = "
    "60 %."
)
print(a.outcome, a.values)
# SOLVED {'DHV': 4000.0, 'DDHV': 2400.0}

a = transportation.ask(
    "Spot speed percentiles with speeds = [52, 61, 48, 70, 55, 63, 58, "
    "66, 49, 72]."
)
print(a.outcome, a.values)
# SOLVED {'p85': 68.6, 'p50': 59.5, 'p15': 50.05}

a = transportation.ask(
    "Gap acceptance capacity with major flow = 800 veh/h, critical gap "
    "= 6.5 s and follow up time = 3.3 s."
)
print(a.outcome, a.values)
# SOLVED {'c': 363.101002924}

a = transportation.ask(
    "Acceleration under constant power with mass = 1500 kg, power = 75 "
    "kW, initial speed = 20 km/h, final speed = 100 km/h, drag "
    "coefficient = 0.32 and frontal area = 2.2 m^2."
)
print(a.outcome, a.values)
# SOLVED {'t': 8.32025289502, 'd': 162.053710134}

a = transportation.ask(
    "Speed reduction braking distance with initial speed = 90 km/h, "
    "final speed = 40 km/h, friction = 0.4 and grade = 3 %."
)
print(a.outcome, a.values)
# SOLVED {'d': 59.4484993809, 't': 3.2925322734}

a = transportation.ask(
    "Simplified curve formula with superelevation = 0.06 %, side "
    "friction = 0.12 and speed = 100 km/h."
)
print(a.outcome, a.values)
# SOLVED {'R': 652.196829539, 'v': 27.7777777778, 'v_kmh': 100.0}

a = transportation.ask(
    "Time mean and space mean speed with speeds = [52, 61, 48, 70, 55, "
    "63]."
)
print(a.outcome, a.values)
# SOLVED {'TMS': 58.1666666667, 'SMS': 57.2529499157, 's': 8.03533861556}

a = transportation.ask(
    "Coast down time with mass = 1400 kg, initial speed = 100 km/h, "
    "final speed = 60 km/h, drag coefficient = 0.32 and frontal area = "
    "2.2 m^2."
)
print(a.outcome, a.values)
# SOLVED {'t': 42.2501297979, 'd': 917.248498508}

a = transportation.ask(
    "Horizontal sightline offset with radius = 300 m and sight "
    "distance = 160 m."
)
print(a.outcome, a.values)
# SOLVED {'M': 10.6036064308}

a = transportation.ask(
    "Train acceleration with mass = 400 t, initial speed = 0 km/h, "
    "final speed = 100 km/h, tractive effort = 150 kN, power = 4000 "
    "kW, davis a = 6 kN, davis b = 0.05 kN/(km/h) and davis c = 0.0008 "
    "kN/(km/h)^2."
)
print(a.outcome, a.values)
# SOLVED {'t': 86.5875384958, 'd': 1222.69194342}

a = transportation.ask(
    "Quarter car natural frequencies with sprung mass = 400 kg, "
    "unsprung mass = 45 kg, spring stiffness = 25 kN/m and tyre "
    "stiffness = 200 kN/m."
)
print(a.outcome, a.values)
# SOLVED {'f_body': 1.18543873987, 'f_wheel': 11.2618541121}
```

## Command line: 15 examples

Each command below was run with `transportation` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ transportation solve transport_braking_distance "v=27.77777778 m/s" "a=3.4 m/s^2"
d = 113.4713145 m
t = 8.169934641 s

$ transportation solve transport_freeway_level_of_service "V=4000 1/h" PHF=0.92 N=3 P_T=0.1 E_T=2 "S=27.77777778 m/s"
f_HV = 0.9090909091
v_p = 1594.202899 1/h
D = 15.94202898 1/km
D_mi = 25.65620869
LOS = C

$ transportation solve transport_clothoid_coordinates "R=300 m" "L_s=80 m"
x = 79.85789479 m
y = 3.551043118 m
theta = 7.639437268 deg

$ transportation solve transport_logit_elasticities "V=[-1.2, -0.8, -2.1]" "x=[30, 45, 20]" beta=-0.04
P = [0.345018612617,0.514707287448,0.140274099936]
direct = [-0.78597766486,-0.873526882594,-0.687780720051]
cross = [0.41402233514,0.926473117406,0.112219279949]

$ transportation solve transport_equivalent_single_axle_loads "loads=[80, 100, 120]" "counts=[500, 200, 100]" Y=20
LEF = [1.0,2.44140625,5.0625]
ESAL_day = 1494.53125
GF = 20
ESAL = 10910078.12

$ transportation solve transport_erlang_b_blocking "lam=60 1/h" "mu=0.5 1/h" c=120
P_block = 0.06941876906
carried = 111.6697477
lost_per_h = 4.165126144 1/h

$ transportation solve transport_rail_station_headway "v=22.22222222 m/s" "a=1 m/s^2" "b=1.1 m/s^2" "L_t=150 m" "t_d=30 s" "s_m=50 m"
h = 70.2020202 s
trains_per_h = 51.28057554 1/h

$ transportation solve transport_fastest_route "roads=A-B 4, A-C 2.5, B-C 1.2, B-D 5, C-D 8, C-E 10, D-E 2" "speeds=60, 50, 40, 80, 100, 90, 50" origin=A destination=E
route = A-C-E
time = 9.666666667 min
distance = 12.5 km

$ transportation solve transport_design_hourly_volume AADT=40000 K=0.1 D=0.6
DHV = 4000 1/h
DDHV = 2400 1/h

$ transportation solve transport_greenberg_fit "speeds=[82, 70, 58, 47, 33, 21]" "densities=[18, 32, 46, 60, 78, 95]"
v0 = 36.43837575 km/h
kj = 197.8362218 1/km
q_max = 2651.980568 1/h
r2 = 0.9573293444

$ transportation solve transport_signal_green_splits "C=90 s" "L=10 s" "y=[0.3, 0.25, 0.15]"
g = [34.2857142857,28.5714285714,17.1428571429]
X_c = 0.7875

$ transportation solve transport_spot_speed_percentiles "speeds=[52, 61, 48, 70, 55, 63, 58, 66, 49, 72]"
p85 = 68.6 km/h
p50 = 59.5 km/h
p15 = 50.05 km/h

$ transportation solve transport_all_or_nothing_assignment "network=A-B 5, B-A 5, A-C 3, C-A 3, B-D 4, D-B 4, C-D 7, D-C 7, B-C 1, C-B 1" "od=A-D 800, C-B 200, D-A 300"
flows = [0.0,0.0,800.0,300.0,800.0,300.0,0.0,0.0,300.0,1000.0]
VHT = 150 h

$ transportation solve transport_gap_acceptance_capacity "q=800 1/h" "t_c=6.5 s" "t_f=3.3 s"
c = 363.1010029 1/h

$ transportation solve transport_lateral_load_transfer "m=1500 kg" "a_y=0.6 m/s^2" "h=0.55 m" "t_w=1.55 m"
W_out = 7676.854839 N
W_in = 7038.145161 N
dW = 319.3548387 N
```

## Rust library: 15 examples

The crate `transportation-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `transportation_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/transportation/examples/readme.rs`: `cd rust && cargo run --release --example readme -p transportation-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&transportation_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "transport_braking_distance", "inputs": {"v": "27.77777778 m/s", "a": "3.4 m/s^2"}}));
// SOLVED
// d = 113.47131447 m
// t = 8.16993464118 s

show(json!({"key": "transport_freeway_level_of_service", "inputs": {"V": "4000 1/h", "PHF": "0.92", "N": "3", "P_T": "0.1", "E_T": "2", "S": "27.77777778 m/s"}}));
// SOLVED
// f_HV = 0.909090909091
// v_p = 1594.20289855 1/h
// D = 15.9420289842 1/km
// D_mi = 25.6562086936
// LOS = "C"

show(json!({"key": "transport_clothoid_coordinates", "inputs": {"R": "300 m", "L_s": "80 m"}}));
// SOLVED
// x = 79.8578947851 m
// y = 3.55104311752 m
// theta = 7.63943726841 deg

show(json!({"key": "transport_logit_elasticities", "inputs": {"V": [-1.2, -0.8, -2.1], "x": [30, 45, 20], "beta": "-0.04"}}));
// SOLVED
// P = [0.345018612617,0.514707287448,0.140274099936]
// direct = [-0.78597766486,-0.873526882594,-0.687780720051]
// cross = [0.41402233514,0.926473117406,0.112219279949]

show(json!({"key": "transport_equivalent_single_axle_loads", "inputs": {"loads": [80, 100, 120], "counts": [500, 200, 100], "Y": "20"}}));
// SOLVED
// LEF = [1.0,2.44140625,5.0625]
// ESAL_day = 1494.53125
// GF = 20.0
// ESAL = 10910078.125

show(json!({"key": "transport_erlang_b_blocking", "inputs": {"lam": "60 1/h", "mu": "0.5 1/h", "c": "120"}}));
// SOLVED
// P_block = 0.0694187690644
// carried = 111.669747712
// lost_per_h = 4.16512614387 1/h

show(json!({"key": "transport_rail_station_headway", "inputs": {"v": "22.22222222 m/s", "a": "1 m/s^2", "b": "1.1 m/s^2", "L_t": "150 m", "t_d": "30 s", "s_m": "50 m"}}));
// SOLVED
// h = 70.2020202 s
// trains_per_h = 51.280575541 1/h

show(json!({"key": "transport_fastest_route", "inputs": {"roads": "A-B 4, A-C 2.5, B-C 1.2, B-D 5, C-D 8, C-E 10, D-E 2", "speeds": "60, 50, 40, 80, 100, 90, 50", "origin": "A", "destination": "E"}}));
// SOLVED
// route = "A-C-E"
// time = 9.66666666667 min
// distance = 12.5 km

show(json!({"key": "transport_design_hourly_volume", "inputs": {"AADT": "40000", "K": "0.1", "D": "0.6"}}));
// SOLVED
// DHV = 4000.0 1/h
// DDHV = 2400.0 1/h

show(json!({"key": "transport_greenberg_fit", "inputs": {"speeds": [82, 70, 58, 47, 33, 21], "densities": [18, 32, 46, 60, 78, 95]}}));
// SOLVED
// v0 = 36.43837575 km/h
// kj = 197.836221849 1/km
// q_max = 2651.98056847 1/h
// r2 = 0.957329344359

show(json!({"key": "transport_signal_green_splits", "inputs": {"C": "90 s", "L": "10 s", "y": [0.3, 0.25, 0.15]}}));
// SOLVED
// g = [34.2857142857,28.5714285714,17.1428571429]
// X_c = 0.7875

show(json!({"key": "transport_spot_speed_percentiles", "inputs": {"speeds": [52, 61, 48, 70, 55, 63, 58, 66, 49, 72]}}));
// SOLVED
// p85 = 68.6 km/h
// p50 = 59.5 km/h
// p15 = 50.05 km/h

show(json!({"key": "transport_all_or_nothing_assignment", "inputs": {"network": "A-B 5, B-A 5, A-C 3, C-A 3, B-D 4, D-B 4, C-D 7, D-C 7, B-C 1, C-B 1", "od": "A-D 800, C-B 200, D-A 300"}}));
// SOLVED
// flows = [0.0,0.0,800.0,300.0,800.0,300.0,0.0,0.0,300.0,1000.0]
// VHT = 150.0 h

show(json!({"key": "transport_gap_acceptance_capacity", "inputs": {"q": "800 1/h", "t_c": "6.5 s", "t_f": "3.3 s"}}));
// SOLVED
// c = 363.101002924 1/h

show(json!({"key": "transport_lateral_load_transfer", "inputs": {"m": "1500 kg", "a_y": "0.6 m/s^2", "h": "0.55 m", "t_w": "1.55 m"}}));
// SOLVED
// W_out = 7676.85483871 N
// W_in = 7038.14516129 N
// dW = 319.35483871 N
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Transportation on ai999x.com](https://www.ai999x.com/models/transportation-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/transportation-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/transportation-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `transportation` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/transportation`) and its data (`rust/models/transportation/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p transportation-ai999x
./target/release/transportation --help
cargo test -p transportation-ai999x
```
