---
sidebar_position: 2
---

# Contextualized Expressions
Contextualized expressions form the core of what differentiate MistQL from many other languages, and so should be given special attention. Contextualized Expressions are MistQL's standin for what would be lambdas in other languages.

Every expression is executed with a certain _context_. The variable @ refers to the current context of an expression _when it is executed_. Context operates like a _stack_: functions that modify the current context do so by pushing a context to what is reerred to as the _context stack_.

When calculating the result of a function, MistQL often executes expressions multiple times under different contexts. For example, in the following code, the expression `@ % 2 == 0` is run multiple times: For each element in the array, filter pushes the @ variable (along with any properties of that variable) to the stack, and executes the first expression.

`filter (@%2 == 0) [1, 2, 3, 4, 5, 6, 7]` results in `[2, 4, 6]`

The function calling the expression dictates how context is supplied to that expression, and how to handle the result of the expression. Functions that push to the context stack in this way are notated as `@: SomeContextType -> SomeReturnType` in the documentation.

As always, variables defined on the `@` variable are populated into scope. In the below, since @ is set to each element of the array in turn, the variable `foo` evaluates to the `foo` key of each array element in turn.

```
(map (foo + 1) [{"num": 10}, {"num": 20}])) == [11, 21]
```

Having contextualized expressions in this manner allows for some very clean-looking syntax. For example, filtering for all events of a given type and mapping to their email can be done with the following:

`events | filter type == "page_shown" | map email`

In the above query, we first filter for all events where `event.type`  is "page_shown", then map to the email field on each event in turn.
