# transportation-ai999x — offline, verified transportation solver with step-by-step answers

[![tests](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/transportation-ai999x)](https://pypi.org/project/transportation-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/transportation-ai999x)

**[Install](https://pypi.org/project/transportation-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/transportation-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/transportation-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.2

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

transportation-ai999x is an offline, deterministic Python solver that answers transportation questions with verified results from 77 certified capabilities, gives 536 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**transportation-ai999x** is a compact, deterministic transportation engine for Python. It reads a question in plain English (or a typed call), picks a certified transportation capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified transportation capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 536 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** vehicle dynamics, braking and stopping distances, traffic flow, highway geometry, railway operations, route planning and transit capacity.

## Model overview

| Property | Value |
|---|---|
| Package | `transportation-ai999x` 1.0.2 (import `transportation_ai999x`) |
| Type | Deterministic neuro-symbolic transportation engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 77 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 536 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

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

Command line: `transportation-ai999x "<question>"`, `transportation-ai999x --capabilities`. The outputs above were produced by running this exact code on transportation-ai999x 1.0.2.

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

## Worked examples

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
@software{transportation_ai999x_1_0_2,
  title  = {transportation-ai999x: an offline, verified transportation solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.2},
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
