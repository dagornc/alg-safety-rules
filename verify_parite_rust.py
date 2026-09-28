#!/usr/bin/env python3
"""Compare exactement les sorties CSV Rust et Python de ALG_SAFETY_RULES."""

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
REFERENCE = ROOT / "reference" / "sim_safety_rules.py"
SCENARIOS = [
    "nominal", "battery_critical", "battery_low", "watchdog", "geofence",
    "cbf", "sensor_failure", "drift", "adversarial",
]


def data_line(command):
    output = subprocess.run(command, capture_output=True, text=True, check=True).stdout
    lines = output.strip().splitlines()
    if len(lines) != 2:
        raise RuntimeError(f"sortie CSV inattendue ({len(lines)} lignes): {output!r}")
    return lines[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binaire", default=str(ROOT / "target" / "release" / "safety_rules_rs"))
    parser.add_argument("--graines", nargs=2, type=int, default=[1000, 1100])
    parser.add_argument("--scenarios", nargs="*", default=SCENARIOS)
    args = parser.parse_args()
    binary = Path(args.binaire)
    if not binary.is_file():
        print(f"ERREUR : binaire introuvable : {binary}", file=sys.stderr)
        return 2

    total = differences = 0
    for scenario in args.scenarios:
        if scenario not in SCENARIOS:
            print(f"ERREUR : scenario inconnu : {scenario}", file=sys.stderr)
            return 2
        identical = 0
        for seed in range(*args.graines):
            python_line = data_line([sys.executable, str(REFERENCE), str(seed), scenario])
            rust_line = data_line([str(binary), str(seed), scenario])
            total += 1
            if python_line == rust_line:
                identical += 1
            else:
                differences += 1
                if differences <= 10:
                    print(f"DIVERGENCE [{scenario}] seed={seed}")
                    print(f"  python: {python_line}")
                    print(f"  rust:   {rust_line}")
        print(f"scenario {scenario:16s} : {identical}/{args.graines[1] - args.graines[0]} identiques")

    print(f"TOTAL : {total - differences}/{total} identiques")
    print(f"ECARTS : {differences}")
    if differences == 0:
        print("PARITÉ BIT-À-BIT VÉRIFIÉE ✓")
        return 0
    print("PARITÉ NON VÉRIFIÉE")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
