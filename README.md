# transportation-ai999x — offline, verified transportation solver with step-by-step answers

[![tests](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/transportation-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/transportation-ai999x)](https://pypi.org/project/transportation-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/transportation-ai999x)

**[Install](https://pypi.org/project/transportation-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/transportation-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/transportation-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.1

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
| Package | `transportation-ai999x` 1.0.1 (import `transportation_ai999x`) |
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

Command line: `transportation-ai999x "<question>"`, `transportation-ai999x --capabilities`. The outputs above were produced by running this exact code on transportation-ai999x 1.0.1.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

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
@software{transportation_ai999x_1_0_1,
  title  = {transportation-ai999x: an offline, verified transportation solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.1},
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
