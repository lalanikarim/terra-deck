# Ratatui Knowledge Base
## Version: 0.30.0

## Core Concepts

### Terminal User Interface (TUI)
- Ratatui is a library for building terminal user interfaces using Rust
- Uses immediate mode rendering (similar to egui but for terminals)
- Zero-cost abstractions, pure Rust

### Main Components
- **Terminal**: Main interface to the terminal (manages raw mode, etc.)
- **Frame**: Represents a drawable area where widgets are rendered
- **Widget**: Trait for things that can be drawn (similar to React components)
- **Layout**: Handles splitting and positioning areas (Flexbox-like)
- **Block**: Borders and titles for containers
- **Paragraph**: Text display with alignment and styling
- **Table**: For displaying tabular data
- **List**: Rendering lists of items
- **BarChart**: For data visualization
- **Gauge**: Progress indicators
- **Sparkline**: Small inline charts
- **Canvas**: Custom drawing area

### Styling
- **Style**: Foreground/background colors, modifiers (bold, italic, etc.)
- **Color**: Supports ANSI colors (8, 256, truecolor)
- **Modifier**: Bold, italic, underline, etc.
- **Stylize Trait**: Extension trait for easy styling (e.g., `"text".red().bold()`)

### Layout System
- **Constraint**: Defines how space is split (Percentage, Length, Min/Max, Ratio, Fill)
- **Layout**: Splits a rect into multiple rects based on constraints
- **Rect**: Rectangle area (x, y, width, height)
- **Alignment**: Horizontal/Vertical positioning within a rect

### Event Handling
- Uses crossterm for event polling (keyboard, mouse, resize)
- Event types: Key, Mouse, Resize
- Polling typically done in a loop with timeout

### TUI Application Structure
1. Initialize terminal (enter raw mode)
2. Main loop:
   - Draw UI to frame
   - Handle input events
   - Update application state
3. Restore terminal on exit

### Best Practices for Our Project
1. Separate UI logic from game state (keep game state in Bevy ECS)
2. Use minimal redraws - only update changed parts
3. Use Layout for responsive design (works in different terminal sizes)
4. Style consistently using a theme/palette
5. Handle terminal resize events gracefully
6. Use blocking or timeout-based event polling to control CPU usage
7. Consider using a state machine for different screens (menu, gameplay, game over)

### Common Widgets We'll Need
- **Block**: For card display with borders/titles
- **Paragraph**: For card values, suit symbols, HP display
- **Table/List**: For showing hand of cards
- **Gauge**: For health bars if needed
- **Canvas**: For custom card rendering if desired
- **Text**: For logs and messages
