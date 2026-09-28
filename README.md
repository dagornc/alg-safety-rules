# ALG_SAFETY_RULES — portage Rust

Implémentation Rust déterministe de règles de sûreté embarquées, utilisable sans lien de communication. Le crate et le binaire se nomment `safety_rules_rs`.

## Ce qui est implémenté

Une machine à états locale applique l'ordre de priorité suivant :

1. `LAND` si les capteurs essentiels sont invalides ou si la batterie est à 10 % ou moins ;
2. `RTL` si le watchdog atteint 500 ms, si la batterie est à 20 % ou moins, ou si la dérive atteint 300 cm ;
3. `FREEZE` à l'approche sortante de la frontière géographique ;
4. `FREEZE` par garde CBF simplifiée si un obstacle franchit la marge dynamique ;
5. `NOMINAL` sinon.

Les seuils sont exprimés en entiers pour obtenir des décisions reproductibles. Le scénario `adversarial` injecte de façon déterministe, via le MT19937 compatible CPython fourni par le socle RUST-13, un cas nominal ou une anomalie locale.

Interface commune Rust/Python :

    safety_rules_rs <seed> [scenario]
    python3 reference/sim_safety_rules.py <seed> [scenario]

Scénarios : `nominal`, `battery_critical`, `battery_low`, `watchdog`, `geofence`, `cbf`, `sensor_failure`, `drift`, `adversarial`.

La sortie contient exactement deux lignes CSV (en-tête puis données) avec l'action, le déclenchement, la fausse alerte, le temps de réaction et l'activation CBF.

## Ce qui est vérifié

Résultats mesurés lors de la publication initiale :

- `cargo build --release` : succès ;
- `cargo test` : 13 tests passés, 0 échec (4 tests unitaires RNG + 9 tests d'intégration) ;
- `python3 verify_parite_rust.py` : 900 comparaisons (100 graines × 9 scénarios), 0 écart ;
- statut : **parité bit-à-bit vérifiée** sur la ligne CSV complète.

Commandes de re-jeu :

    cargo build --release
    cargo test
    python3 verify_parite_rust.py

## Ce qui ne l'est pas

Cette implémentation est un simulateur de référence simplifié, pas un contrôleur certifié :

- la validation Monte-Carlo adversariale à N ≥ 1000 demandée par le modèle LikeC4 n'a pas été faite ;
- aucun essai matériel en vol, HIL/SIL, ni test de latence temps réel n'a été effectué ;
- les seuils opérationnels doivent être calibrés et approuvés pour chaque plateforme ;
- la garde CBF est une règle scalaire conservatrice, pas une résolution complète d'optimisation CBF ;
- la colonne `fausse_alerte` reste à zéro car les scénarios injectés utilisent la même vérité terrain explicite que les seuils approuvés ; elle ne constitue pas une estimation terrain du taux de fausses alertes.

Le modèle LikeC4 classe l'algorithme comme `conceptual` / `idea` et mentionne l'hypothèse que les capteurs essentiels restent valides. Ce portage ne change pas ce statut de validation.

## Licence

MIT — Christophe Dagorn, 2026.
