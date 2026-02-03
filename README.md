# Rust Cell Simulation

Un simulateur de vie cellulaire (type Jeu de la Vie de Conway) haute performance, écrit en Rust.

## 🏗️ Architecture du Projet

Le projet est divisé en deux modules principaux (Workspace) pour séparer la logique métier de l'interface graphique.

```text
rust-cell-simulation/
├── core_sim/ (LE DOMAINE MÉTIER)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Point d'entrée, expose les modules
│       │
│       ├── domain/             # Les structures de données pures (Anémique)
│       │   ├── mod.rs
│       │   ├── cell.rs         # Struct Cell, Enum State
│       │   └── grid.rs         # Struct Grid (Gestion mémoire 1D)
│       │
│       ├── engine/             # La mécanique interne (Le moteur)
│       │   ├── mod.rs
│       │   ├── simulator.rs    # Le "Service" principal (Double Buffer)
│       │   └── config.rs       # Struct SimulationConfig (Builder Pattern)
│       │
│       └── rules/              # La logique métier interchangeable (Strategy Pattern)
│           ├── mod.rs
│           ├── traits.rs       # Trait SimulationRule
│           └── conway.rs       # Implémentation classique
│
├── app_gui/ (L'ADAPTATEUR GRAPHIQUE)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # Bootstrap de l'app
│       │
│       ├── state/              # Gestion de l'état de l'application
│       │   ├── mod.rs
│       │   └── app_state.rs    # Global State (Paused, Speed, CurrentTool)
│       │
│       ├── view/               # Tout ce qui concerne le rendu visuel
│       │   ├── mod.rs
│       │   ├── renderer.rs     # Dessine la grille (Macroquad)
│       │   └── camera.rs       # Gestion du Zoom/Pan (Maths de transformation)
│       │
│       ├── ui/                 # L'interface utilisateur (Widgets)
│       │   ├── mod.rs
│       │   ├── hud.rs          # Overlay (Compteur FPS, Status)
│       │   └── panels.rs       # Fenêtres de contrôle (Egui side panel)
│       │
│       └── input/              # Gestion des interactions
│           ├── mod.rs
│           └── mouse.rs        # Raycasting (Écran -> Grille)