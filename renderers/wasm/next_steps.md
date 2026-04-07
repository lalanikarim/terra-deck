# Next Steps - WASM Renderer Enhancement

## Option A: Integrate game_core (Recommended)
Bring in the actual Terra-Deck game session logic.

### Steps:
1. Add `game_core` as dev-dependency in `renderers/wasm/Cargo.toml`
2. Create GameState wrapper around `game_core::GameSession`
3. Render game board, cards, players
4. Connect WASM event handlers to game actions

### Why First?
- Tests existing game logic in browser
- Real gameplay instead of demo particles
- Foundation for all other visual features

---

## Option B: Add Terrain Rendering
Visualize the game board with tiles/terrain.

### Steps:
1. Define terrain tile types (grass, water, mountain, etc.)
2. Create tile rendering function
3. Add tile map data structure
4. Render grid-based terrain

### Why?
- Visual feedback for game board
- Prepares for unit placement
- Nice visual improvement

---

## Option C: Add Card Display
Render poker cards on the canvas.

### Steps:
1. Define card symbols (hearts, diamonds, clubs, spades)
2. Draw card rectangles with suit/rank
3. Add card animations
4. Display player hand/deck

### Why?
- Core to Terra-Deck gameplay
- Visual feedback for deck building
- Engaging UI

---

## Option D: Add Save/Load
Serialize game state for persistence.

### Steps:
1. Implement Serialize/Deserialize for GameState
2. Use `serde-wasm-bindgen` for JS interop
3. Add save/load methods to CanvasApplication
4. Store in localStorage

### Why?
- Progress preservation
- Enables complex game states
- Good for testing

---

## Quick Commands

### Rebuild WASM:
```bash
cd /Users/karim/Projects/ocproject/bevygame/renderers/wasm
wasm-pack build --target web
```

### Test Server:
```bash
# Server should already be running on port 8000
# If not:
npx http-server -p 8000 --cors
```

### View Results:
```
http://localhost:8000/web/index.html
```

---

## Recommendation
**Start with Option A (game_core integration)** since you already have 110 tests passing in game_core - this will give you immediate value and a real playable game in the browser.

Tell me which option you want to tackle and I'll help you implement it! 🎮
