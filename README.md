# Simple Tower Defense

A minimalist tower defense game written in Rust using the Piston game engine.

## Current Features

### Core Gameplay
- Central tower that automatically targets and shoots nearest enemy
- Enemies spawn from screen edges and move towards the tower
- Bullet collision system with enemy health tracking
- Visual tower range indicator
- Enemy health bars

### Economy System
- Score tracking (10 points per kill)
- Satoshi currency system (5 sats per kill)
- Persistent save system (automatically saves progress)

### Tower Upgrades
- Damage upgrade (D key)
  - Increases tower damage by 5 per level
  - Cost scales with level (20 * current_level)
- Fire rate upgrade (F key)
  - Reduces cooldown by 20% per level
  - Cost scales with level (25 * current_level)

### UI Elements
- Score display (blue bar)
- Satoshi balance (gold bar)
- Upgrade costs with key indicators
- Current upgrade level indicators
- Visual feedback for tower range and bullet hits

### Technical Features
- Automatic game state saving
- Efficient collision detection
- Smooth animations and movement
- JSON-based save system

## Planned Features

### Gameplay Improvements
- Different enemy types:
  - Fast but weak enemies
  - Slow but tanky enemies
  - Special abilities (healing, splitting, etc.)
- Wave-based spawning system
- Boss enemies at milestone waves

### Visual and Audio
- Sound effects for:
  - Tower shooting
  - Enemy hits
  - Enemy deaths
  - Upgrades
- Visual effects:
  - Death animations
  - Hit particles
  - Upgrade animations

### New Mechanics
- Additional tower upgrades:
  - Range increase
  - Multi-shot capability
  - Special ammunition types
- Power-up system:
  - Random drops from enemies
  - Temporary boosts
  - Special abilities

### UI Enhancements
- Wave counter and timer
- High score leaderboard
- More detailed upgrade information
- Tutorial tooltips

## Controls
- D: Purchase damage upgrade
- F: Purchase fire rate upgrade
- ESC: Save and exit game

## Building and Running
1. Ensure you have Rust installed
2. Clone the repository
3. Run `cargo build --release`
4. Run `cargo run --release`

## Dependencies
- piston_window: 0.132.0
- serde: 1.0.215
- serde_json: 1.0.133
- find_folder: 0.3.0
- rand: 0.8.5
