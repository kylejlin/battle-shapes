# battle-shapes
A simple 2D battle game written in Rust, using piston_window.

```bash
git clone https://github.com/kylejlin/battle-shapes.git
cargo run
```

## Overview
There are two teams: blue (left) and red (right). You control the blue team, and the computer controls the red team (except in sandbox mode, where you control both).

Press keys to deploy troops where your mouse cursor is.

Troops will move toward the opposing side until they die. Along the way, if they encounter enemy troops they will engage them in combat. If they manage to reach the other side, their team will instantly win. The entire screen will turn the color of their team to reflect the victory.

## Troops
| Troop | Key to Deploy | Price | Health | Attack Damage | Attack Cooldown (seconds) | Movement Speed |
| --- | --- | --- | --- | --- | --- | --- |
| Swordsman | 1 (0) | 30 | 100 | 30 | 0.8 | 35 |
| Archer | 2 (9) | 60 | 100 | 25 | 2.0 | 20 |
| Rider | 3 (8) | 50 | 100 | 25 | 1.0 | 80 |
| Shieldsman | 4 (7) | 40 | 250 | 0 | N/A | 35 |
| Daggerman | 5 (6) | 50 | 50 | 20 | 0.15 | 25 |
| Coward Archer | Q (P) | 60 | 100 | 25 | 2.75 | 15 |
| Giant | W (O) | 100 | 1000 | 100 | 3.5 | 12 |
| Wall | A (Semicolon) | 10 | 300 | 0 | N/A | 0 |

Note: for the "Key to Deploy" column, the key in parentheses is the key to deploy for the red team. Normally, this won't do anything because the computer controls the red team. However, this works in sandbox mode, where you control both teams.

## Game Modes
All modes are off by default, but you can turn them on with command line flags. They are not mutually exclusive (i.e., you can play with several modes on at once).

| Mode | Flag | Description |
| --- | --- | --- |
| Sandbox | `-s` or `--sandbox` | You control both sides. |
| Big Money | `-m` or `--big-money` | Your money regenerates much faster. |
| Freespawn | `-f` or `--freespawn` | The normal deployment boundaries don't apply, so you can deploy troops anywhere on the screen. |

Note: if you are using `cargo run`, you have to add a `--` before passing in the desired flags. This is so Cargo knows the flags are intended for the executable (this program), not Cargo. For example, if you want to activate all the modes, you would type:

```bash
cargo run -- -s -m -f
```
