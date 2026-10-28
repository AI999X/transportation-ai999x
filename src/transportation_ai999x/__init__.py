"""transportation-ai999x — offline transportation answers with their formula, steps and verification.

    import transportation_ai999x as m
    m.ask("Stopping sight distance with speed = 100 km/h, reaction time = 2.5 s and deceleration = 3.4 m/s^2.").show()
    # SSD = 182.916 m, d_r = 69.4444 m, d_b = 113.471 m
    m.solve("transport_greenshields_capacity", vf="100 km/h", kj="120 1/km").show()
    # q_max = 3000 1/h, k_m = 60 1/km, v_m = 50 km/h
    m.capabilities()                   # every certified capability (70)

What it covers, from school to graduate level:
- braking and stopping: braking distance, stopping sight distance with grade, skid-mark speed, slowing distance,
  impact speed after reaction and braking, the yellow change interval, braking with aerodynamic and rolling
  resistance;
- vehicle performance: acceleration distance, overtaking, coast-down, acceleration under constant power (and a truck
  slowing on an upgrade), friction-limited tractive force, the safe following gap;
- vehicle dynamics: braking and lateral load transfer, rollover threshold, understeer gradient with characteristic and
  critical speed, steady-state cornering, steer angle for a radius, yaw natural frequency, quarter-car natural
  frequencies and transmissibility;
- highway geometry: safe speed, minimum radius, side friction and superelevation on curves, crest vertical curve
  length and sight distance, horizontal sightline offset, clothoid transition coordinates;
- traffic flow and signals: Greenshields, Greenberg and Underwood capacities and fits, states at a flow, shockwaves,
  queues at signals (D/D/1 uniform delay and shockwave analysis), bottleneck and interval queues, green splits,
  Poisson arrivals, headways, gap-acceptance capacity, time- and space-mean and percentile spot speeds;
- queues: M/M/1, M/M/c (Erlang C), M/D/1, Erlang B blocking, first-come-first-served simulation of given arrivals;
- railway operations: running time between stations, station headway and line capacity, train braking and
  acceleration with Davis resistance;
- networks and planning: fastest route and travel-time skims, all-or-nothing assignment, doubly constrained gravity
  model, Furness balancing, trip-generation regression, multinomial and nested logit, elasticities, logsum benefits.

Questions take one documented form: the capability's name, then "with name = value unit, ..." (lists in brackets,
road networks as edge lists such as [A-B 4, B-C 2.5]); the reader also takes km/h, mph, veh/h, veh/km, %, per mille.
solve() takes SI numbers or unit strings: flows in veh/s or '600 1/h', densities in veh/m or '25 1/km', angles in
rad or '2 deg'. g = 9.81 m/s^2. Built by the AI999X model machine from the same engine as every domain; a question
that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.3"
_P = Product("transportation", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
