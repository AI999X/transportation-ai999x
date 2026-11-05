# transportation-ai999x — offline, verified transportation solver with step-by-step answers

[![tests](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/transportation-ai999x)](https://pypi.org/project/transportation-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/transportation-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/transportation-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/transportation-ai999x/)** · **[Guides](https://huggingface.co/AI999X/transportation-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/transportation-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

transportation-ai999x is an offline, deterministic Python solver that answers transportation questions with verified results from 77 certified capabilities, gives 536 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**transportation-ai999x** is a compact, deterministic transportation engine for Python. It reads a question in plain English (or a typed call), picks a certified transportation capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** transportation-ai999x comes as the AI999X desktop app and as the command-line program `transportation` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Transportation desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-transportation-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-transportation-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`transportation`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/transportation-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install transportation-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Transportation** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Transportation among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-transportation-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-transportation-1.4.0-linux-x86_64.tar.gz
./ai999x-transportation-1.4.0-linux-x86_64/ai999x-transportation
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
transportation --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
transportation --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ transportation ask "Stopping sight distance with speed = 100 km/h, reaction time = 2.5 s and deceleration = 3.4 m/s^2."
Read as: Stopping sight distance (reaction plus braking, with grade)
SSD = 182.9157589 m
d_r = 69.44444444 m
d_b = 113.4713145 m
  by: Stopping sight distance (reaction plus braking, with grade)  (SSD = v t_r + v^2 / (2 g (f + G)), or with a deceleration rate v t_r + v^2 / (2 (a + g G)))
  a = 3.4 m/s^2
  t_r = 2.5 s
  v = 27.77777778 m/s
  assumes: v > 0; t_r >= 0; exactly one of f, a; f + G > 0 (or a + g G > 0)
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ transportation info transport_signal_queue_uniform_delay
Queue and uniform delay at a signal (deterministic D/D/1)
  Q = lam r, t_c = lam r/(s - lam), D = lam r^2 / (2(1 - lam/s)), d = r^2 / (2C (1 - lam/s))
  in   lam      arrival flow (veh/h in questions; veh/s in solve(), or a string such as '600 1/h') [1/s]
  in   s        saturation flow (veh/h in questions; veh/s in solve(), or a string such as '600 1/h') [1/s]
  in   r        effective red [s]
  in   g        effective green [s]
  out  Q_max    largest queue (veh) []
  out  t_clear  time after the start of green to clear the queue (s) [s]
  out  D_total  total delay per cycle (veh s) [s]
  out  d_uniform average delay per vehicle (s) [s]
  out  P_stopped share of vehicles that stop []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ transportation solve transport_signal_queue_uniform_delay "lam=600 1/h" "s=1800 1/h" "r=40 s" "g=30 s"
Q_max = 6.666666667
t_clear = 20 s
D_total = 200 s
d_uniform = 17.14285714 s
P_stopped = 0.8571428571
  by: Queue and uniform delay at a signal (deterministic D/D/1)  (Q = lam r, t_c = lam r/(s - lam), D = lam r^2 / (2(1 - lam/s)), d = r^2 / (2C (1 - lam/s)))
  lam = 0.1666666667 1/s
  s = 0.5 1/s
  r = 40 s
  g = 30 s
  assumes: lam < s; the queue clears within the green (lam C <= s g)
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | transportation batch

`cases.jsonl`:

```json
{"key": "transport_signal_queue_uniform_delay", "inputs": {"lam": "600 1/h", "s": "1800 1/h", "r": "40 s", "g": "30 s"}}
{"key": "transport_curve_safe_speed", "inputs": {"R": "300 m", "e": "0.06 1", "f": "0.12 1"}}
```

```console
$ transportation batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":6.66666666667,"unit":"","solved_for":"Q_max","si":[["lam",0.16666666666666666],["s",0.5],["r",40.0],["g",30.0]],"values":[["Q_max",6.66666666667,""],["t_clear",20.0,"s"],["D_total",200.0,"s"],["d_uniform",17.1428571429,"s"],["P_stopped",0.857142857143,""]],"target":"Q_max","phrase":"largest queue (veh)","assumptions":["lam < s","the queue clears within the green (lam C <= s g)"],"name":"Queue and uniform delay at a signal (deterministic D/D/1)","equation":"Q = lam r, t_c = lam r/(s - lam), D = lam r^2 / (2(1 - lam/s)), d = r^2 / (2C (1 - lam/s))"}
{"outcome":"SOLVED","message":"","value":23.0993893616,"unit":"m/s","solved_for":"v","si":[["R",300.0],["e",0.06],["f",0.12]],"values":[["v",23.0993893616,"m/s"],["v_kmh",83.1578017018,"km/h"]],"target":"v","phrase":"maximum safe speed (m/s)","assumptions":["R > 0","f > 0","e f < 1"],"name":"Maximum safe speed on a superelevated horizontal curve","equation":"v = sqrt(g R (e + f) / (1 - e f)) (point-mass equilibrium)"}
```

### In your browser

Open [Transportation on ai999x.com](https://www.ai999x.com/models/transportation-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `transportation_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install transportation-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Braking distance from speed and deceleration

Distance and time for a vehicle to stop from speed v at a constant deceleration a (no reaction time).

Command line:

```console
$ transportation solve transport_braking_distance "v=27.77777778 m/s" "a=3.4 m/s^2"
d = 113.4713145 m
t = 8.169934641 s
  by: Braking distance from speed and deceleration  (d = v^2 / (2a), t = v / a)
  v = 27.77777778 m/s
  a = 3.4 m/s^2
  assumes: v > 0; a > 0
```

Python:

```python
import transportation_ai999x as m

a = m.solve("transport_braking_distance", v="27.77777778 m/s", a="3.4 m/s^2")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_braking_distance) and press **Compute in your browser**.

_What this means:_ This calculation determines the distance a vehicle travels to stop completely from an initial speed of 27.7778 m/s under a constant deceleration of 3.4 m/s^2. The resulting braking distance is 113.471 m, and the time required to reach a full stop is 8.16993 s. These values assume no driver reaction time and level ground.

### Distance to slow from one speed to another (friction and grade)

Braking distance and time to slow from v1 to v2 with friction coefficient f on a grade G (deceleration g (f + G)).

Command line:

```console
$ transportation solve transport_speed_reduction_distance "v1=25 m/s" "v2=11.11111111 m/s" "f=0.4 1" "G=0.03 1"
d = 59.44849938 m
t = 3.292532274 s
  by: Distance to slow from one speed to another (friction and grade)  (d = (v1^2 - v2^2) / (2 g (f + G)))
  v1 = 25 m/s
  v2 = 11.11111111 m/s
  f = 0.4
  G = 0.03
  assumes: v1 > v2 >= 0; f + G > 0
```

Python:

```python
import transportation_ai999x as m

a = m.solve("transport_speed_reduction_distance", v1="25 m/s", v2="11.11111111 m/s", f="0.4 1", G="0.03 1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_speed_reduction_distance) and press **Compute in your browser**.

_What this means:_ This calculation determines the distance and time required for a vehicle to decelerate from an initial speed of 25 m/s to a final speed of 11.1111 m/s. The result assumes a constant deceleration governed by the friction coefficient 0.4 and the road grade 0.03. The computed stopping distance is 59.4485 m, which represents the length of road needed to reduce speed under these specific conditions, while the duration of this maneuver is 3.29253 s.

### Impact speed with an obstacle after reaction and braking

A driver at speed v sees an obstacle at distance D, reacts after t_r and brakes with friction f on grade G: the speed at the obstacle (0 if the vehicle stops in time) and the full stopping distance.

Command line:

```console
$ transportation solve transport_impact_speed "v=25 m/s" "D=60 m" "t_r=1 s" "f=0.6 1"
v_impact = 14.59383431 m/s
v_impact_kmh = 52.53780353 km/h
d_stop = 78.09208291 m
  by: Impact speed with an obstacle after reaction and braking  (v_i = sqrt(v^2 - 2 g (f + G) (D - v t_r)) when the obstacle lies within the braking phase)
  v = 25 m/s
  D = 60 m
  t_r = 1 s
  f = 0.6
  assumes: v > 0; D > 0; t_r >= 0; f + G > 0
```

Python:

```python
import transportation_ai999x as m

a = m.solve("transport_impact_speed", v="25 m/s", D="60 m", t_r="1 s", f="0.6 1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_impact_speed) and press **Compute in your browser**.

_What this means:_ The impact speed is 14.5938 m/s (52.5378 km/h), with a total stopping distance of 78.0921 m.

### Speed estimate from skid-mark length

Initial speed of a vehicle that skidded to rest over a length d with friction coefficient f on a grade G.

Command line:

```console
$ transportation solve transport_skid_speed "d=30 m" "f=0.6 1"
v = 18.79255172 m/s
v_kmh = 67.65318618 km/h
  by: Speed estimate from skid-mark length  (v = sqrt(2 g (f + G) d))
  d = 30 m
  f = 0.6
  assumes: d > 0; f > 0; f + G > 0
```

Python:

```python
import transportation_ai999x as m

a = m.solve("transport_skid_speed", d="30 m", f="0.6 1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/transportation-ai999x/#cap-transport_skid_speed) and press **Compute in your browser**.

_What this means:_ The estimated initial speed is 18.7926 m/s which converts to 67.6532 km/h.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/transportation-ai999x/blob/main/docs/transportation_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/transportation-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified transportation capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 536 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `transportation` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** vehicle dynamics, braking and stopping distances, traffic flow, highway geometry, railway operations, route planning and transit capacity.

## Model overview

| Property | Value |
|---|---|
| Package | `transportation-ai999x` 1.1.0 (import `transportation_ai999x`) |
| Type | Deterministic neuro-symbolic transportation engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 77 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 536 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

```bash
pip install transportation-ai999x
```

```python
import transportation_ai999x as transportation

transportation.ask("Stopping sight distance with speed = 100 km/h, reaction time = 2.5 s and deceleration = 3.4 m/s^2.").show()
# -> SOLVED — SSD = 182.916, d_r = 69.4444, d_b = 113.471  (transport_stopping_sight_distance)
transportation.ask("Signal queue and uniform delay with arrival flow = 600 veh/h, saturation flow = 1800 veh/h, red = 40 s and green = 30 s.").show()
# -> SOLVED — Q_max = 6.66667, t_clear = 20, D_total = 200, d_uniform = 17.1429  (transport_signal_queue_uniform_delay)
transportation.ask("Fastest route with roads = [A-B 4, A-C 2.5, B-C 1.2, B-D 5, C-D 8, C-E 10, D-E 2], speeds = [60, 50, 40, 80, 100, 90, 50], origin = A and destination = E.").show()
# -> SOLVED — route = A-C-E, time = 9.66667, distance = 12.5  (transport_fastest_route)
transportation.ask("Maximum safe speed on a curve with radius = 300 m, superelevation = 6 % and side friction = 0.12.").show()
# -> SOLVED — v = 23.0994, v_kmh = 83.1578  (transport_curve_safe_speed)
transportation.solve("transport_understeer_gradient", m="1500 kg", L="2.7 m", l_f="1.2 m", C_f="90000 N/rad", C_r="85000 N/rad").show()
# -> SOLVED — K = 0.00141612, K_deg_per_g = 0.795962, behaviour = understeer, v_c = 43.6648  (transport_understeer_gradient)
transportation.ask("Who created you?").message
# -> transportation-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs a Python command, `transportation-ai999x "<question>"`; for the native program use `transportation` (above). The outputs above were produced by running this exact code on transportation-ai999x 1.1.0.

## Find the right capability, and draw it

`transportation.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "transportation-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import transportation_ai999x as transportation

transportation.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `transportation-ai999x`; the answer is the one it returned, with the steps it took.

**Stopping sight distance (reaction plus braking, with grade)** · _Braking and stopping_

> Stopping sight distance with speed = 100 km/h, reaction time = 2.5 s and deceleration = 3.4 m/s^2.

→ **SSD = 182.9157589 m, d_r = 69.44444444 m, d_b = 113.4713145 m**

<details><summary>Steps</summary>

- Stopping sight distance (reaction plus braking, with grade): v = 27.77777778 m/s, t_r = 2.5 s, a = 3.4 m/s^2 → SSD = 182.9157589 m, d_r = 69.44444444 m, d_b = 113.4713145 m

</details>

**Queue and uniform delay at a signal (deterministic D/D/1)** · _Traffic signals_

> Signal queue and uniform delay with arrival flow = 600 veh/h, saturation flow = 1800 veh/h, red = 40 s and green = 30 s.

→ **Q_max = 6.666666667, t_clear = 20 s, D_total = 200 s, d_uniform = 17.14285714 s, P_stopped = 0.8571428571**

<details><summary>Steps</summary>

- Queue and uniform delay at a signal (deterministic D/D/1): lam = 600 1/h, s = 1800 1/h, r = 40 s, g = 30 s → Q_max = 6.666666667, t_clear = 20 s, D_total = 200 s, d_uniform = 17.14285714 s, P_stopped = 0.8571428571

</details>

**Fastest route on a road network (link lengths and speeds)** · _Route planning_

> Fastest route with roads = [A-B 4, A-C 2.5, B-C 1.2, B-D 5, C-D 8, C-E 10, D-E 2], speeds = [60, 50, 40, 80, 100, 90, 50], origin = A and destination = E.

→ **route = A-C-E, time = 9.666666667 min, distance = 12.5 km**

<details><summary>Steps</summary>

- Fastest route on a road network (link lengths and speeds): roads = A-B (4), A-C (2.5), B-C (1.2), B-D (5), C-D (8), C-E (10), D-E (2), speeds = [60, 50, 40, 80, 100, 90, 50], origin = A, destination = E → route = A-C-E, time = 9.666666667 min, distance = 12.5 km

</details>

**Maximum safe speed on a superelevated horizontal curve** · _Highway geometry_

> Maximum safe speed on a curve with radius = 300 m, superelevation = 6 % and side friction = 0.12.

→ **v = 23.09938936 m/s, v_kmh = 83.1578017 km/h**

<details><summary>Steps</summary>

- Maximum safe speed on a superelevated horizontal curve: R = 300 m, e = 0.06 1, f = 0.12 1 → v = 23.09938936 m/s, v_kmh = 83.1578017 km/h

</details>

**Understeer gradient and characteristic or critical speed (bicycle model)** · _Vehicle dynamics_

`m.solve("transport_understeer_gradient", m="1500 kg", L="2.7 m", l_f="1.2 m", C_f="90000 N/rad", C_r="85000 N/rad")`

→ **K = 0.001416122004 rad*s^2/m, K_deg_per_g = 0.7959619566 deg, behaviour = understeer, v_c = 43.66480716 m/s, v_c_kmh = 157.1933058 km/h**

<details><summary>Steps</summary>

- Understeer gradient and characteristic or critical speed (bicycle model): m = 1500 kg, L = 2.7 m, l_f = 1.2 m, C_f = 90000 N/rad, C_r = 85000 N/rad → K = 0.001416122004 rad*s^2/m, K_deg_per_g = 0.7959619566 deg, behaviour = understeer rad*s^2/m, v_c = 43.66480716 m/s, v_c_kmh = 157.1933058 km/h

</details>

**Direct and cross elasticities of logit choice probabilities** · _Mode choice_

> Logit elasticities with utilities = [-1.2, -0.8, -2.1], attribute = [30, 45, 20] and beta = -0.04.

→ **P = [0.3450186126, 0.5147072874, 0.1402740999], direct = [-0.7859776649, -0.8735268826, -0.6877807201], cross = [0.4140223351, 0.9264731174, 0.1122192799]**

<details><summary>Steps</summary>

- Direct and cross elasticities of logit choice probabilities: V = [-1.2, -0.8, -2.1], x = [30, 45, 20], beta = -0.04 → P = [0.3450186126, 0.5147072874, 0.1402740999], direct = [-0.7859776649, -0.8735268826, -0.6877807201], cross = [0.4140223351, 0.9264731174, 0.1122192799]

</details>

## Figures

Every point was computed by `transportation-ai999x`.

**Speed–density data and the fitted traffic-flow model**

![Speed–density data and the fitted traffic-flow model — computed by transportation-ai999x](https://huggingface.co/AI999X/transportation-ai999x/resolve/main/figures/transport-greenshields-fit-fit.svg)

_Six observed speed–density pairs and the linear speed–density model fitted to them by the package: free-flow speed vf = 95.3537 km/h, jam density kj = 120.141 veh/km, R² = 0.99869; the line is traced by the package's speeds at 80 flows up to the capacity 2863.96 veh/h. Marked: k = 0, v = vf._

**The flow–density fundamental diagram and capacity**

![The flow–density fundamental diagram and capacity — computed by transportation-ai999x](https://huggingface.co/AI999X/transportation-ai999x/resolve/main/figures/transport-greenshields-capacity-diagram.svg)

_Fundamental diagram for free-flow speed 100 km/h and jam density 120 veh/km: for each of 40 flows the package returns the two densities that carry it (uncongested left, congested right). Marked: the capacity it returned, q = 3000 veh/h at the optimum density k = 60 veh/km and speed 50 km/h._

**Stopping sight distance: reaction plus braking**

![Stopping sight distance: reaction plus braking — computed by transportation-ai999x](https://huggingface.co/AI999X/transportation-ai999x/resolve/main/figures/transport-stopping-sight-distance-diagram.svg)

_Stopping sight distance against design speed (reaction time 2.5 s, deceleration 3.4 m/s², level road), 25 speeds computed by the package: the lower band is the distance travelled during the reaction time, the upper band the braking distance. Marked: v = 100 km/h, SSD = 182.916 m (reaction 69.4444 m + braking 113.471 m)._

**Queue at a traffic signal over one cycle**

![Queue at a traffic signal over one cycle — computed by transportation-ai999x](https://huggingface.co/AI999X/transportation-ai999x/resolve/main/figures/transport-signal-queue-uniform-delay-queue.svg)

_Queue at a signal over one cycle (arrivals 600 veh/h, saturation flow 1800 veh/h, red 40 s, green 30 s): it grows during red to the maximum the package returned, 6.66667 vehicles, and clears 20 s into green; the shaded area is the total delay 200 veh·s (17.1429 s per vehicle, 85.7143 % stopped). Marked: t = 40 s, queue = 6.66667 vehicles._

**Time–space diagram of a signal queue**

![Time–space diagram of a signal queue — computed by transportation-ai999x](https://huggingface.co/AI999X/transportation-ai999x/resolve/main/figures/transport-signal-queue-shockwave-timespace.svg)

_Time–space diagram of the queue at a signal (arrivals 1000 veh/h at 20 veh/km, jam density 140 veh/km, discharge 1800 veh/h at 50 veh/km, red 40 s): the queue tail moves upstream at 8.33333 km/h, the discharge wave from the start of green at 20 km/h (both returned by the package); where they meet the queue is longest. Marked: t = 68.5714 s, maximum queue 158.73 m._

## Capabilities

77 certified procedures. A few of them:

- **Procedures:** 85th, 50th and 15th percentile spot speeds; Acceleration time and distance under constant engine power; All-or-nothing traffic assignment; Blocking probability with no waiting room (Erlang B), e.g. parking spaces; Braking distance from speed and deceleration; Capacity of a minor stream by gap acceptance; Clothoid (spiral) transition curve coordinates; Coast-down time and distance (rolling and aerodynamic resistance); Curve radius or speed by the simplified e + f formula; Direct and cross elasticities of logit choice probabilities; Distance and time to accelerate (acceleration lane length); Distance to slow from one speed to another (friction and grade); Doubly constrained gravity model of trip distribution; Fastest route on a road network (link lengths and speeds); First-come-first-served queue from arrival and service times; Fit the Greenberg (logarithmic) model to speed-density observations; Fit the Greenshields model to speed-density observations; Fit the Underwood (exponential) model to speed-density observations; Green splits by equal degree of saturation (flow-ratio method); Greenberg model: capacity and density at capacity; Greenshields model: capacity, optimum speed and density; Greenshields model: densities and speeds at a given flow; Growth-factor (Furness) balancing of a trip matrix; Horizontal sightline offset on a circular curve …

`transportation_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified transportation capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (vehicle dynamics, braking and stopping distances, traffic flow, highway geometry, railway operations, route planning and transit capacity); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{transportation_ai999x_1_1_0,
  title  = {transportation-ai999x: an offline, verified transportation solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
  url    = {https://huggingface.co/AI999X/transportation-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/transportation-ai999x/discussions or visit https://ai999x.com.

## Support the project

transportation-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/transportation-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/transportation-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/transportation-ai999x/tree/main/onnx).
