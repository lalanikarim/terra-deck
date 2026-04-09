# Bevy Knowledge Base

## Version: 0.18.1

## Core Concepts

### ECS (Entity Component System)

**Entities**: Unique IDs representing game objects  
**Components**: Plain Rust structs attached to entities  
**Components**: Data-only, derive `Component` trait  
**Systems**: Functions processing entities with specific components  
**Resources**: Global data accessible to all systems  

### App Structure

```rust
App::new()
    .add_plugins(DefaultPlugins)  // Includes window, input, rendering
    .add_systems(Startup, setup)   // Runs once at app start
    .add_systems(Update, logic)    // Runs every frame during gameplay
    .run();
```

### Key Patterns

**Entity Spawning**:
```rust
commands.spawn((
    ComponentA { field: value },
    ComponentB { value: other_value },
    Transform::default(),
));
```

**Querying**:
```rust
fn my_system(
    query: Query<&my_component>,
) {
    for entity in &query {
        // process entity
    }
}
```

**Commands**:
```rust
struct Commands<'w, 's>
fn spawn() -> EntityBuilder
fn insert_resource() -> Self
fn add_systems() -> Self
```

---

## 2D Rendering Patterns

### Camera2d

**Minimal Setup**:
```rust
commands.spawn(Camera2d);
```

**Camera2dBundle** (for more control):
```rust
commands.spawn(Camera2dBundle {
    transform: Transform::default(),
    camera: Camera {
        order: 0,
        ..default()
    },
    ..default()
});
```

### Sprite Rendering

**Option 1: Sprite with Image (Recommended when using textures)**:
```rust
commands.spawn(Sprite::from_image(
    asset_server.load("path/to/image.png"),
));
```

**Option 2: Solid Color Rectangle**:
```rust
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(width, height))),
    MeshMaterial2d(materials.add(Color::srgb(0.9, 0.2, 0.2))),
    Transform::from_xyz(x, y, 0.0),
));
```

**Option 3: Sprite Custom Size**:
```rust
commands.spawn(Sprite {
    color: Color::srgb(0.5, 0.5, 0.5),
    custom_size: Some(Vec2::new(2.0, 1.0)),
    ..default()
});
```

### Transform Patterns

**Basic Position**:
```rust
Transform::from_xyz(10.0, 5.0, 0.0)
```

**Using Translations**:
```rust
Transform::from_translation(Vec3::new(10.0, 5.0, 0.0))
```

**Position + Scale**:
```rust
Transform::from_xyz(10.0, 5.0, 0.0)
    .with_scale(Vec3::splat(2.0))
```

**Rotation**:
```rust
Transform::default()
    .with_rotation(Quat::from_rotation_z(f32::from_degrees(45.0)))
```

---

## 3D Rendering Patterns

### Camera3d

**Basic Setup**:
```rust
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(-2.0, 3.0, 5.0)
        .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
));
```

### StandardMaterial

The go-to material for 3D rendering:
```rust
commands.spawn((
    Mesh3d(meshes.add(Sphere::new(radius))),
    MeshMaterial3d(materials.add(Color::srgb(r, g, b))),
    Transform::from_xyz(x, y, z),
));
```

### Common Mesh Shapes

```rust
Mesh3d(meshes.add(Sphere::new(1.0)))
Mesh3d(meshes.add(Cuboid::default()))
Mesh3d(meshes.add(Torus::default()))
Mesh3d(meshes.add(Capsule3d::default()))
Mesh3d(meshes.add(Tetrahedron::default()))

// 2D shapes extruded to 3D:
Mesh3d(meshes.add(Extrusion::new(Circle::default(), 1.0)))
Mesh3d(meshes.add(Extrusion::new(Rectangle::default(), height)))
```

### Plane Ground

```rust
commands.spawn((
    Mesh3d(meshes.add(Plane3d::default()
        .mesh()
        .size(50.0, 50.0)
        .subdivisions(10))),
    MeshMaterial3d(materials.add(Color::srgb(0.3, 0.15, 0.3))),
    Transform::from_xyz(0.0, -2.5, 0.0),
));
```

### Lights

**Point Light**:
```rust
commands.spawn((
    PointLight {
        intensity: 10_000_000.,
        range: 100.0,
        shadows_enabled: true,
        ..default()
    },
    Transform::from_xyz(4.0, 8.0, 4.0),
));
```

---

## Input Handling

### Keyboard Input

```rust
use bevy::input::common_conditions::input_just_pressed;

fn my_system(
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::KeyW) {
        // move forward
    }
    if keyboard.just_pressed(KeyCode::Space) {
        // jump
    }
}
```

### Common Conditions

```rust
use bevy::input::common_conditions::*;

.add_systems(Update,
    my_system.run_if(input_just_pressed(KeyCode::Space)),
    my_other_system.run_if(input_toggle_active(true, KeyCode::KeyR)),
)
```

### Mouse Input

```rust
fn mouse_system(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // click handled
    }
}
```

---

## Resources vs Components

**Resources** - Global singleton data, one per type:
```rust
#[derive(Resource)]
pub struct GameSession {
    // global state
}
```

**Query for Resources**:
```rust
fn my_system(
    game: Res<GameSession>,
    mut game_mut: ResMut<GameSession>,
) {}
```

**Components** - Per-entity data, many per type:
```rust
#[derive(Component)]
pub struct Player {
    // per-player state
}
```

**Query for Components**:
```rust
fn my_system(
    players: Query<&Player>,
    mut players: Query<&mut Position, With<Player>>,
) {}
```

---

## Schedules and Update Phases

**Startup** - Runs once at app initialization:
```rust
.add_systems(Startup, setup_assets)
```

**Update** - Runs every frame:
```rust
.add_systems(Update, player_input)
```

**PreUpdate** - Before Update:
```rust
.add_systems(PreUpdate, pre_computation)
```

**PostUpdate** - After Update (for rendering):
```rust
.add_systems(PostUpdate, post_render_computation)
```

### System Ordering

```rust
.add_systems(Update,
    first_system.before(second_system),
    second_system.after(first_system),
)
```

---

## Asset Loading

### Loading Textures/Sprites

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("path/to/image.png");
    commands.spawn(Sprite::from_image(texture));
}
```

### Loading Meshes

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(Rectangle::new(width, height));
    commands.spawn(Mesh2d(mesh));
}
```

---

## Mesh Primitives

From `bevy::render::mesh::`:
- `Rectangle::new(width, height)` - 2D rectangle
- `Cuboid::default()` - 3D rectangular prism
- `Sphere::new(radius)` - Sphere
- `Torus::default()` - Donut shape
- `Capsule3d::default()` - Capsule
- `Cone::default()` - Cone
- `Cylinder::default()` - Cylinder
- `Plane3d::default()` - Infinite plane
- `Extrusion::new(shape, depth)` - Extrude 2D shape to 3D

---

## Common Patterns

### Spawn Multiple Entities

```rust
for i in 0..10 {
    commands.spawn((
        MyComponent { value: i },
        Transform::from_xyz(i as f32, 0.0, 0.0),
    ));
}
```

### Conditional Spawning

```rust
fn setup_system(
    mut commands: Commands,
    should_spawn: Res<ShouldSpawn>,
) {
    if should_spawn.0 {
        commands.spawn(Sprite::from_image(texture));
    }
}
```

### Cleanup/Despawn

```rust
fn cleanup(
    mut commands: Commands,
    to_despawn: Query<Entity, With<ToDespawn>>,
) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn();
    }
}
```

---

## Debugging Tips

### Logging

```rust
println!("System running, entities: {}", count);
```

### Queries

```rust
fn debug_system(
    query: Query<(Entity, &Position, &Name)>,
) {
    for (entity, pos, name) in &query {
        println!("Entity {:?} at {:?} named {:?}", 
                 entity.index(), pos, name);
    }
}
```

### Checking Component Existence

```rust
fn check_system(
    query_with: Query<Entity, With<MyComponent>>,
    query_without: Query<Entity, Without<MyComponent>>,
) {
    println!("With: {}, Without: {}", 
             query_with.iter().count(),
             query_without.iter().count());
}
```

---

## Performance Considerations

### Avoid System Bottlenecks

```rust
// BAD - Query everything every frame
fn update_all(mut all: Query<&mut Transform>) {
    for mut transform in &mut all {
        // too many transforms
    }
}

// GOOD - Query only what you need
fn update_players(mut players: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut players {
        // only player transforms
    }
}
```

### Use Local State for One-Time Setup

```rust
fn spawn_once(
    mut spawned: Local<bool>,
    mut commands: Commands,
) {
    if *spawned {
        return;
    }
    *spawned = true;
    // spawn once
}
```

---

## Common Errors and Fixes

### Sprite Not Visible
**Cause**: Missing `Mesh2d` or `MeshMaterial2d` components  
**Fix**: Use proper bundle:
```rust
commands.spawn((
    Mesh2d(mesh),
    MeshMaterial2d(material),
    Transform::from_xyz(...),
));
```

### Camera Not Showing Objects
**Cause**: Objects behind camera or at Z=0 with no depth  
**Fix**: Move objects to positive Z:
```rust
Transform::from_xyz(0.0, 0.0, 1.0)
```

### Material Color Not Appearing
**Cause**: `MeshMaterial2d` expects `StandardMaterial` handle  
**Fix**: Create and insert material:
```rust
commands.spawn((
    Mesh2d(mesh),
    MeshMaterial2d(materials.add(Color::srgb(...))),
    ...
));
```

---

## Integration with game_core

### Wrapping game_core State as Bevy Resource

```rust
#[derive(Resource)]
pub struct GameSessionResource {
    pub session: game_core::GameSession,
}

impl Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSessionResource::new());
    }
}
```

### System Signature Pattern

```rust
fn render_cards(
    game_session: Res<GameSessionResource>,
) {
    let hand = &game_session.session.player_hand;
    for card in &hand.cards {
        // render card
    }
}
```

---

## Bevy 0.18 Specifics

### Changed Components

1. **`Camera2dBundle` removed** - Use `Camera2d` component directly:
```rust
commands.spawn(Camera2d);
```

2. **`Sprite` requires image** - Or use Mesh2d for colors:
```rust
// With texture:
Sprite::from_image(image_handle)

// Solid color:
Mesh2d(mesh) + MeshMaterial2d(material)
```

3. **Event System** - Use `#[derive(Event)]`:
```rust
#[derive(Event, Debug, Clone)]
pub struct GameAction {
    // action data
}
```

### Camera2d vs OrthographicProjection

In Bevy 0.18, you must use `Camera2d` component directly:
```rust
commands.spawn(Camera2d);
```

---

---

## Bevy 0.18 Key Corrections for game_bevy

### Critical Discoveries

1. **Camera2d must be spawned explicitly**:
   ```rust
   commands.spawn(Camera2d);
   ```
   The default plugins don't automatically handle this.

2. **For colored rectangles**, use `Mesh2d` + `MeshMaterial2d`:
   ```rust
   commands.spawn((
       Mesh2d(meshes.add(Rectangle::new(2.0, 1.2))),
       MeshMaterial2d(materials.add(Color::srgb(r,g,b))),
       Transform::from_xyz(x, y, 0.0),
   ));
   ```

3. **`Sprite::from_image()` requires actual image assets** - for solid colors, use the Mesh approach above.

4. **Z-position matters**: Objects at Z=0 with Camera at Z=0 may not render - use positive Z values.

5. **Resources vs Components**:
   - `Resource` - Global singleton (query with `Res<T>` or `ResMut<T>`)
   - `Component` - Per-entity data (query with `Query<&T>`)

### Final game_bevy Structure

```
game_bevy/src/
├── main.rs                      # Minimal app bootstrap
└── plugins/
    ├── mod.rs                   # Module re-export
    ├── game_state.rs            # GameSessionResource
    └── rendering.rs             # Mesh2d-based card rendering
```

### Next Steps for Implementation

1. **Task 2.2** - Fan layout positioning
2. **Task 2.3** - Map card suits to actual Kenney sprite textures
3. **Task 2.4** - Opponent card backs (card_back.png)
4. **Task 3** - UI overlay and animations
5. **Task 4** - Polish and interactions

---

