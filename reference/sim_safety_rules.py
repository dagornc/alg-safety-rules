#!/usr/bin/env python3
"""Référence normative de la machine à états ALG_SAFETY_RULES."""

import random
import sys

SCENARIOS = {
    "nominal", "battery_critical", "battery_low", "watchdog", "geofence",
    "cbf", "sensor_failure", "drift", "adversarial",
}


def params_for(scenario):
    p = {
        "battery": 80, "watchdog": 20, "boundary": 100, "outward": 0,
        "obstacle": 100, "closing": 0, "drift": 0, "sensors": True,
    }
    if scenario == "battery_critical": p["battery"] = 7
    elif scenario == "battery_low": p["battery"] = 18
    elif scenario == "watchdog": p["watchdog"] = 600
    elif scenario == "geofence": p.update(boundary=4, outward=80)
    elif scenario == "cbf": p.update(obstacle=12, closing=100)
    elif scenario == "sensor_failure": p["sensors"] = False
    elif scenario == "drift": p["drift"] = 350
    return p


def simulate(seed, scenario):
    p = params_for(scenario)
    if scenario == "adversarial":
        case = random.Random(seed).randint(0, 6)
        if case == 0: p["battery"] = 5
        elif case == 1: p["watchdog"] = 900
        elif case == 2: p.update(boundary=0, outward=120)
        elif case == 3: p.update(obstacle=8, closing=140)
        elif case == 4: p["sensors"] = False
        elif case == 5: p["drift"] = 500

    cbf = 0
    if not p["sensors"] or p["battery"] <= 10:
        action, reaction = "LAND", 20
    elif p["watchdog"] >= 500 or p["battery"] <= 20 or p["drift"] >= 300:
        action, reaction = "RTL", 40
    elif p["boundary"] <= 5 and p["outward"] > 0:
        action, reaction = "FREEZE", 20
    elif p["obstacle"] <= 15 + p["closing"] // 10 and p["closing"] > 0:
        action, reaction, cbf = "FREEZE", 20, 1
    else:
        action, reaction = "NOMINAL", 0
    trigger = int(action != "NOMINAL")
    return f"{seed},{scenario},{action},{trigger},0,{reaction},{cbf}"


def main():
    if len(sys.argv) < 2:
        print("usage: sim_safety_rules.py <seed> [scenario]", file=sys.stderr)
        return 2
    try:
        seed = int(sys.argv[1])
    except ValueError:
        print(f"seed invalide: {sys.argv[1]}", file=sys.stderr)
        return 2
    scenario = sys.argv[2] if len(sys.argv) > 2 else "nominal"
    if scenario not in SCENARIOS:
        print(f"scenario inconnu: {scenario}", file=sys.stderr)
        return 2
    print("seed,scenario,action,declenchement,fausse_alerte,temps_reaction_ms,cbf_actif")
    print(simulate(seed, scenario))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
