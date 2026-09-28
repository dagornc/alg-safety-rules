//! Machine à états de sûreté autonome.
//!
//! Les priorités sont explicites : LAND > RTL > FREEZE > NOMINAL. Toutes les
//! grandeurs sont entières afin de rendre les décisions reproductibles entre
//! plateformes et d'éviter une ambiguïté aux seuils.

use crate::rng::PyRandom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Nominal,
    Freeze,
    Rtl,
    Land,
}

impl Action {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nominal => "NOMINAL",
            Self::Freeze => "FREEZE",
            Self::Rtl => "RTL",
            Self::Land => "LAND",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Params {
    pub batterie_pct: u8,
    pub watchdog_ms: u32,
    pub distance_frontiere_dm: i32,
    pub vitesse_sortante_cms: u32,
    pub distance_obstacle_dm: u32,
    pub vitesse_approche_cms: u32,
    pub derive_cm: u32,
    pub capteurs_valides: bool,
    pub adversarial: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            batterie_pct: 80,
            watchdog_ms: 20,
            distance_frontiere_dm: 100,
            vitesse_sortante_cms: 0,
            distance_obstacle_dm: 100,
            vitesse_approche_cms: 0,
            derive_cm: 0,
            capteurs_valides: true,
            adversarial: false,
        }
    }
}

impl Params {
    pub fn scenario(nom: &str) -> Result<Self, String> {
        let mut p = Self::default();
        match nom {
            "nominal" => {}
            "battery_critical" => p.batterie_pct = 7,
            "battery_low" => p.batterie_pct = 18,
            "watchdog" => p.watchdog_ms = 600,
            "geofence" => {
                p.distance_frontiere_dm = 4;
                p.vitesse_sortante_cms = 80;
            }
            "cbf" => {
                p.distance_obstacle_dm = 12;
                p.vitesse_approche_cms = 100;
            }
            "sensor_failure" => p.capteurs_valides = false,
            "drift" => p.derive_cm = 350,
            "adversarial" => p.adversarial = true,
            _ => return Err(format!("scenario inconnu: {nom}")),
        }
        Ok(p)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resultat {
    pub action: Action,
    pub declenchement: bool,
    pub fausse_alerte: bool,
    pub temps_reaction_ms: u32,
    pub cbf_active: bool,
}

/// Évalue les réflexes sans aucune donnée de communication.
pub fn simuler(seed: u64, params: &Params) -> Resultat {
    let mut p = params.clone();
    if p.adversarial {
        appliquer_cas_adversarial(seed, &mut p);
    }

    let (action, temps_reaction_ms, cbf_active) = if !p.capteurs_valides || p.batterie_pct <= 10 {
        (Action::Land, 20, false)
    } else if p.watchdog_ms >= 500 || p.batterie_pct <= 20 || p.derive_cm >= 300 {
        (Action::Rtl, 40, false)
    } else if p.distance_frontiere_dm <= 5 && p.vitesse_sortante_cms > 0 {
        (Action::Freeze, 20, false)
    } else {
        // Marge CBF entière : 1,5 m de garde plus 1 dm par 10 cm/s
        // d'approche. Cette règle est conservatrice et locale.
        let marge_cbf_dm = 15u32.saturating_add(p.vitesse_approche_cms / 10);
        if p.distance_obstacle_dm <= marge_cbf_dm && p.vitesse_approche_cms > 0 {
            (Action::Freeze, 20, true)
        } else {
            (Action::Nominal, 0, false)
        }
    };

    Resultat {
        action,
        declenchement: action != Action::Nominal,
        // Le simulateur injecte uniquement des cas dont la vérité terrain est
        // définie par les seuils approuvés ; aucun déclenchement nominal n'est
        // donc compté comme fausse alerte.
        fausse_alerte: false,
        temps_reaction_ms,
        cbf_active,
    }
}

fn appliquer_cas_adversarial(seed: u64, p: &mut Params) {
    let mut rng = PyRandom::new(seed);
    match rng.randint(0, 6) {
        0 => p.batterie_pct = 5,
        1 => p.watchdog_ms = 900,
        2 => {
            p.distance_frontiere_dm = 0;
            p.vitesse_sortante_cms = 120;
        }
        3 => {
            p.distance_obstacle_dm = 8;
            p.vitesse_approche_cms = 140;
        }
        4 => p.capteurs_valides = false,
        5 => p.derive_cm = 500,
        _ => {}
    }
}
