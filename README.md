# Code Ratter

A [Mastermind](https://en.wikipedia.org/wiki/Mastermind_(board_game)) code-breaking game for your terminal, built with [Ratatui](https://ratatui.rs/).

```
  ┌─────────┬────────┐
  │ ? ? ? ? │  N  X  │
  ├─────────┼────────┤
  │ - - - - │  -  -  │
  │ - - - - │  -  -  │
  │ - - - - │  -  -  │
  │ - - - - │  -  -  │
  │ - - - - │  -  -  │
  │ 0 0 0 0 │  -  -  │
  │ 2 3 4 5 │  2  0  │
  │ 1 2 3 4 │  1  0  │
  └─────────┴────────┘
```

---

## What is Code Ratter?

Code Ratter is a terminal-native clone of the classic **Mastermind** board game. You have a limited number of attempts to crack a hidden secret code. After each guess, you receive feedback telling you how many are correct but in the wrong position (**N**) and how many digits are correct and in the exact position (**X**).

No mouse. No GUI. Just you, your terminal, and your brain.

---

## Gameplay

The game generates a secret sequence of digits from 1 to 6. Your goal is to guess it within the allowed number of attempts.

After each guess, you'll see two columns of feedback:

**N** Number of digits that are correct but in the **wrong** position
**X** Number of digits that are correct **and** in the correct position

Use the clues to narrow down the secret code, one guess at a time.

**1 - 6** to put number on current guess  
**Backspace** to remove previous number

### Example Round

```
 ? ? ? ?    N  X
─────────┬───────
 - - - - │  -  -
 0 0 0 0 │  -  -   <- current turn
 1 2 3 4 │  1  1
 2 4 5 6 │  2  1
```

In this example:
- Your first guess `2 4 5 6` had **2** digits in the right place and **1** in the right digit, wrong place.
- Your second guess `1 2 3 4` had **1** correct position and **1** misplaced digit.
