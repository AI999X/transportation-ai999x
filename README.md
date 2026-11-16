# Transportation (transportation-ai999x)

[![tests](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/transportation-ai999x)](https://pypi.org/project/transportation-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/transportation-ai999x)

Transportation covers vehicle dynamics, braking and stopping distances, traffic flow, highway geometry, railway operations, route planning and transit capacity: 81 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Transportation desktop app: Side friction demanded on a superelevated curve, with its worked example and live scene](https://www.ai999x.com/?ax_shot=transportation-ai999x/hero.webp)](https://www.ai999x.com/models/transportation-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `transportation` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/transportation-ai999x/#try) |
| **Python** 3.10+ | `pip install transportation-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Transportation desktop app: Side friction demanded on a superelevated curve, with its worked example and live scene](https://www.ai999x.com/?ax_shot=transportation-ai999x/hero-dark.webp)](https://www.ai999x.com/models/transportation-ai999x/)

[![Fit the Greenshields model to speed-density observations: the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/see-1.webp)](https://www.ai999x.com/models/transportation-ai999x/)

*Fit the Greenshields model to speed-density observations*

[![Queue from arrival counts and capacity per interval: the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/see-2.webp)](https://www.ai999x.com/models/transportation-ai999x/)

*Queue from arrival counts and capacity per interval*

[![Shockwave speed in a Greenshields traffic stream: the live scene in the Transportation desktop app](https://www.ai999x.com/?ax_shot=transportation-ai999x/see-3.webp)](https://www.ai999x.com/models/transportation-ai999x/)

*Shockwave speed in a Greenshields traffic stream*

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
