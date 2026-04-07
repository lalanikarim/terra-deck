# 🎮 Terra-Deck Game Loop Documentation

## Overview

The game loop is a **Finite State Machine (FKS)** that manages the complete flow of gameplay. It ensures players interact with the game in a structured, three-step process for each attack.

---

## 1️⃣ The 10 Game States

### Player-Controlled States (Blue)

| State | Description | Player Actions |
|-------|-------------|----------------|
| 🟦 `SelectPlayerCard` | Choose which card to attack with | ← → navigate, Enter to continue |
| 🟦 `SelectOpponentTarget` | Choose which opponent card to target | ← → navigate, Enter to preview |
| 🟦 `ConfirmAttack` | Review and confirm the attack | Y=confirm, N=cancel, Esc=back |

### Opponent/AI States (Yellow)

| State | Description | Controlled By |
|-------|-------------|---------------|
| 🟨 `WaitingForOpponent` | Transition to AI turn | System |
| 🟨 `OpponentSelectingTarget` | AI chooses card | AI Logic |
| 🟨 `OpponentAttackResolving` | AI attack being calculated | System |

### System States (Gray)

| State | Description | Triggered By |
|-------|-------------|--------------|
| ⬜ `Start` | Initial game state | Game startup |
| ⬜ `ResolvingCombat` | Damage calculation | After attack confirmed |
ly ⬜ `GameOver` | Win/Loss detected | When a hand is empty |
| ⬜ `Quit` | User quits game | Q key pressed |

---

## 2️⃣ The Three-Step Combat Flow

```
┌─────────────────────────────────────────────────────────────┐
│                  PLAYER TURN                             │
├─────────────────────────────────────────────────────────┤
│                                                            │
│  ┌──────────────────┐     ┌──────────────────────┐           │
│  │ SelectPlayerCard │────▶│ SelectOpponentTarget │           │
│  │     (← →)        │     │      (← →)           │           │
│  └──────────────────┘     └─────────────────────┘           │
│         │                       │                             │
│         │ Enter                 │ Enter                       │
│         ▼                       ▼                             │
│  ┌──────────────────────────────────────────────────────┐     │
│  │              ConfirmAttack                            │     │
│  │      (Y=Confirm, N=Cancel, Esc=Back)                  │     │
│  └────────────────────────────────────────────────────┘     │
│                          │                                 │
│                          │ Y key                           │
│                          ▼                                 │
│  ┌───────────────────────────────────────────────────┐       │
│  │              ResolvingCombat                       │       │
│  │  (damage, dead card removal, log update)           │       │
│  └───────────────────────────────────────────────────┘       │
│                          │                                 │
│                          │ if game over → GameOver         │
│                          │ else → reset to SelectPlayer    │
│                          ▼                                 │
│  ┌────────────────────────────────────────────────────────┐  │
│  │            Back to Start of Loop                        │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

---


