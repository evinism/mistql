---
sidebar_position: 3
---

# Gotchas

These are the nasty parts of MistQL, documented here for posterity's sake.

## Gotcha 1: Unary Minus:
You may be tempted to write:

```
items | map -cost
```

Unfortunately, the expression is parsed as `map - cost` rather than `map (-cost)`. This is due to the ambiguity
between the minus sign acting as a unary operator vs. a binary operator.

To get around this, simply surround the unary minus operator and operand with parentheses:

```
items | map (-cost)
```

This only affects the minus sign because it's the only token that serves as both a unary and a binary operator.
