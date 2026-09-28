use std::{env, process::ExitCode};

use safety_rules_rs::{Params, simuler};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || matches!(args[0].as_str(), "-h" | "--help" | "--aide") {
        eprintln!(
            "safety_rules_rs <seed> [scenario]\nscenarios: nominal battery_critical battery_low watchdog geofence cbf sensor_failure drift adversarial"
        );
        return ExitCode::SUCCESS;
    }

    let seed = match args[0].parse::<u64>() {
        Ok(seed) => seed,
        Err(_) => {
            eprintln!("seed invalide: {}", args[0]);
            return ExitCode::from(2);
        }
    };
    let scenario = args.get(1).map(String::as_str).unwrap_or("nominal");
    let params = match Params::scenario(scenario) {
        Ok(params) => params,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(2);
        }
    };
    let r = simuler(seed, &params);
    println!("seed,scenario,action,declenchement,fausse_alerte,temps_reaction_ms,cbf_actif");
    println!(
        "{seed},{scenario},{},{},{},{},{}",
        r.action.as_str(),
        u8::from(r.declenchement),
        u8::from(r.fausse_alerte),
        r.temps_reaction_ms,
        u8::from(r.cbf_active)
    );
    ExitCode::SUCCESS
}
