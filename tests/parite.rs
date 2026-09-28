use safety_rules_rs::sim::{Action, Params, simuler};

#[test]
fn batterie_critique_impose_atterrissage() {
    let r = simuler(
        1001,
        &Params {
            batterie_pct: 7,
            ..Params::default()
        },
    );
    assert_eq!(r.action, Action::Land);
    assert_eq!(r.temps_reaction_ms, 20);
}

#[test]
fn watchdog_impose_rtl() {
    let r = simuler(
        1001,
        &Params {
            watchdog_ms: 600,
            ..Params::default()
        },
    );
    assert_eq!(r.action, Action::Rtl);
}

#[test]
fn frontiere_impose_gel() {
    let r = simuler(
        1001,
        &Params {
            distance_frontiere_dm: 4,
            vitesse_sortante_cms: 80,
            ..Params::default()
        },
    );
    assert_eq!(r.action, Action::Freeze);
}

#[test]
fn cbf_renforce_la_distance_minimale() {
    let r = simuler(
        1001,
        &Params {
            distance_obstacle_dm: 12,
            vitesse_approche_cms: 100,
            ..Params::default()
        },
    );
    assert_eq!(r.action, Action::Freeze);
    assert!(r.cbf_active);
}

#[test]
fn priorite_atterrissage_sur_rtl_et_gel() {
    let r = simuler(
        1001,
        &Params {
            batterie_pct: 5,
            watchdog_ms: 900,
            distance_frontiere_dm: 0,
            vitesse_sortante_cms: 100,
            ..Params::default()
        },
    );
    assert_eq!(r.action, Action::Land);
}

#[test]
fn nominal_reste_nominal() {
    for seed in 1000..1100 {
        let r = simuler(seed, &Params::default());
        assert_eq!(r.action, Action::Nominal, "seed={seed}");
        assert!(!r.cbf_active);
        assert_eq!(r.temps_reaction_ms, 0);
    }
}

#[test]
fn panne_capteurs_essentiels_impose_atterrissage() {
    let r = simuler(
        42,
        &Params {
            capteurs_valides: false,
            ..Params::default()
        },
    );
    assert_eq!(r.action, Action::Land);
}

#[test]
fn simulation_est_deterministe() {
    let p = Params::scenario("adversarial").expect("scenario");
    assert_eq!(simuler(777, &p), simuler(777, &p));
}

#[test]
fn scenarios_inconnus_sont_rejetes() {
    assert!(Params::scenario("inconnu").is_err());
}
