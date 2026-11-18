# Transportation: the desktop app, user guide

**Transportation**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Transportation covers vehicle dynamics, braking and stopping distances, traffic flow, highway geometry, railway operations, route planning and transit capacity. Its desktop app holds all 81 calculations in 16 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-transportation-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-transportation-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-transportation-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-transportation.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-transportation-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-transportation-1.6.5-linux-x86_64.tar.gz
./ai999x-transportation-1.6.5-linux-x86_64/ai999x-transportation
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Transportation app, opened on Stopping sight distance (reaction plus braking, with grade) with its worked example.](https://www.ai999x.com/?ax_shot=transportation-ai999x/hero.webp)

*The Transportation app, opened on Stopping sight distance (reaction plus braking, with grade) with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Braking and stopping › Stopping sight distance (reaction plus braking, with grade) (or type part of its name in the search box). It opens with a worked example filled in: “Stopping sight distance with speed = 100 km/h, reaction time = 2.5 s and deceleration = 3.4 m/s^2.” The answer: SSD = 182.9157589 m, d_r = 69.44444444 m, d_b = 113.4713145 m.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Stopping sight distance (reaction plus braking, with grade): the live scene of the worked example.](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-1.webp)

*Stopping sight distance (reaction plus braking, with grade): the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Stopping sight distance with speed = 100 km/h, reaction time = 2.5 s and deceleration = 3.4 m/s^2.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Stopping sight distance (reaction plus braking, with grade)
SSD = 182.9157589 m
d_r = 69.44444444 m
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Queue and uniform delay at a signal (deterministic D/D/1)](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-2.webp)

*Queue and uniform delay at a signal (deterministic D/D/1)*

![Fastest route on a road network (link lengths and speeds)](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-3.webp)

*Fastest route on a road network (link lengths and speeds)*

![Maximum safe speed on a superelevated horizontal curve](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-4.webp)

*Maximum safe speed on a superelevated horizontal curve*

![Understeer gradient and characteristic or critical speed (bicycle model)](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-5.webp)

*Understeer gradient and characteristic or critical speed (bicycle model)*

![Freeway level of service from density (basic segment)](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-6.webp)

*Freeway level of service from density (basic segment)*

![Direct and cross elasticities of logit choice probabilities](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-7.webp)

*Direct and cross elasticities of logit choice probabilities*

![Equivalent single-axle loads by the fourth-power law](https://www.ai999x.com/?ax_shot=transportation-ai999x/scene-8.webp)

*Equivalent single-axle loads by the fourth-power law*

![Stopping sight distance (reaction plus braking, with grade): a chart, enlarged.](https://www.ai999x.com/?ax_shot=transportation-ai999x/chart-1.webp)

*Stopping sight distance (reaction plus braking, with grade): a chart, enlarged.*

![Queue and uniform delay at a signal (deterministic D/D/1): a chart, enlarged.](https://www.ai999x.com/?ax_shot=transportation-ai999x/chart-2.webp)

*Queue and uniform delay at a signal (deterministic D/D/1): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Transportation app in the dark theme.](https://www.ai999x.com/?ax_shot=transportation-ai999x/hero-dark.webp)

*The Transportation app in the dark theme.*

## 7. Every calculation

81 calculations in 16 topics; each has its own entry on the model's page (https://www.ai999x.com/models/transportation-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Braking and stopping | Braking distance from speed and deceleration, Distance to slow from one speed to another (friction and grade), Impact speed with an obstacle after reaction and braking, Speed estimate from skid-mark length, Stopping sight distance (reaction plus braking, with grade), Theoretical stopping distance with aerodynamic and rolling resistance |
| Capacity and level of service | Freeway level of service from density (basic segment) |
| Highway geometry | Clothoid (spiral) transition curve coordinates, Curve radius or speed by the simplified e + f formula, Horizontal sightline offset on a circular curve, Maximum safe speed on a superelevated horizontal curve, Minimum length of a crest vertical curve for a sight distance, Minimum radius of a horizontal curve, Passing sight distance (four-component method), Sag vertical curve length for headlight sight distance, Side friction demanded on a superelevated curve, Sight distance available on a curve with a roadside obstruction, Sight distance available over a crest vertical curve, Superelevation required on a horizontal curve |
| Mode choice | Direct and cross elasticities of logit choice probabilities, Logit mode choice from attributes and coefficients, Multinomial logit mode shares from utilities, Nested logit choice probabilities, User benefit of a change from the logit logsum |
| Pavement design | Equivalent single-axle loads by the fourth-power law |
| Queueing | Blocking probability with no waiting room (Erlang B), e.g. parking spaces, First-come-first-served queue from arrival and service times, Multi-server random queue (M/M/c, Erlang C), e.g. a toll plaza, Single server with constant service time (M/D/1), Single-server random queue (M/M/1), e.g. a toll booth |
| Railway operations | Minimum headway of trains at a station stop (moving block), Running time between stations (accelerate, cruise, brake), Train acceleration time and distance (tractive effort curve, Davis resistance), Train braking distance with Davis resistance and gradient |
| Route planning | Fastest route on a road network (link lengths and speeds), Zone-to-zone travel time matrix (skim) of a road network |
| Traffic characteristics | Design hourly volume from AADT, K and D factors, Peak-hour factor from 15-minute counts |
| Traffic flow | Fit the Greenberg (logarithmic) model to speed-density observations, Fit the Greenshields model to speed-density observations, Fit the Underwood (exponential) model to speed-density observations, Greenberg model: capacity and density at capacity, Greenshields model: capacity, optimum speed and density, Greenshields model: densities and speeds at a given flow, Probability of a headway longer than h (negative exponential), Probability of n vehicle arrivals in an interval (Poisson), Queue and delay at a temporary bottleneck (incident, lane closure), Queue from arrival counts and capacity per interval, Shockwave speed between two traffic states, Shockwave speed in a Greenshields traffic stream, Travel-time reliability indices (buffer, planning time, travel time index), Underwood model: capacity and speed at capacity |
| Traffic signals | Green splits by equal degree of saturation (flow-ratio method), Optimum cycle length by Webster's formula, Queue and uniform delay at a signal (deterministic D/D/1), Queue at a signal by shockwave analysis, Yellow change interval (kinematic, ITE) |
| Traffic studies | 85th, 50th and 15th percentile spot speeds, Time-mean and space-mean speed of spot speeds |
| Travel demand | All-or-nothing traffic assignment, Doubly constrained gravity model of trip distribution, Growth-factor (Furness) balancing of a trip matrix, Trip generation by multiple linear regression |
| Unsignalized intersections | Capacity of a minor stream by gap acceptance |
| Vehicle dynamics | Lateral load transfer in a steady turn, Longitudinal weight transfer under braking and ideal brake proportioning, Quarter-car model: body and wheel-hop natural frequencies, Quarter-car model: road-to-body transmissibility at a frequency, Rollover threshold: static stability factor and rollover speed on a curve, Steady-state cornering: yaw rate, lateral acceleration, radius, sideslip, Steer angle needed for a turn of given radius (bicycle model), Understeer gradient and characteristic or critical speed (bicycle model), Yaw natural frequency and damping of the bicycle model |
| Vehicle performance | Acceleration time and distance under constant engine power, Coast-down time and distance (rolling and aerodynamic resistance), Distance and time to accelerate (acceleration lane length), Maximum tractive force and acceleration limited by tyre friction, Minimum safe following gap when the leader brakes, Overtaking manoeuvre: time and distance with constant acceleration, Top speed of a road vehicle, Vehicle resistance forces and power at a speed |

## 8. Other ways to use it

The same engine runs on the command line (`transportation`, its own guide), in Python (`pip install transportation-ai999x`, `import transportation_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/transportation-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/transportation-ai999x/
