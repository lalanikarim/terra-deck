# Bevy Knowledge Base
## Version: 0.18.1

## Core Concepts

### Entity Component System (ECS)
- **Entities**: Unique IDs that represent objects in the game world
- **Components**: Data attached to entities (plain Rust structs)
- **Systems**: Functions that process entities with specific components
- **Resources**: Global data accessible to systems

### App Structure
- `App::new()` creates the application
- `.add_plugins()` adds functionality (DefaultPlugins includes window, input, etc.)
- `.add_systems()` registers systems to run
- `.run()` starts the main loop

### Systems
- Systems are functions that take parameters from the world
- Systems run in parallel when they don't conflict
- System parameters include:
  - `Query<&Component>`: Read component data
  - `Query<&mut Component>`: Mutably access component data
  - `Res<Resource>`: Read resource
  - `ResMut<Resource>`: Mutably access resource
  - `Commands`: Spawn/despawn entities, add/remove components

### 2D Rendering
- SpriteBundle for 2D sprites
- TextureAtlas for sprite sheets
- Camera2dBundle for 2D camera
- Transform component for position/rotation/scale
- Visibility component to show/hide entities

### Input Handling
- ButtonInput resource for keyboard/mouse input
- ActionState for gamepad input
- Systems typically run in FixedUpdate or Update schedule

### Asset Management
- AssetServer for loading assets
- Handle<T> for referencing assets
- Assets<T> resource for managing asset lifecycle

### Schedules
- Default schedules: Startup, PreUpdate, Update, PostUpdate
- FixedUpdate for physics/timestep-based logic
- Systems can be chained with `.before()`, `.after()`, `.at_start()`, `.at_end()`

### Best Practices for Our Project
1. Keep components small and focused
2. Use resources for game state (turn manager, deck, etc.)
3. Separate concerns: one system per responsibility
4. Use queries efficiently (avoid querying everything every frame)
5. Consider using events for decoupled communication (damage dealt, turn ended, etc.)
