---
sidebar_position: 6
---

# Writing Custom Functions

WARNING: Custom functions are currently only available in the JS implementation of MistQL, and are relatively unstable. Use with caution, and/or ping the Discord if you need help.

There might be instances where MistQL's builtin functions don't provide enough flexibility, but you still want to maintain the MistQL query language interface. The JS implementation provides a mechanism for adding custom functions to MistQL.

## Basic Example

Below is a basic example of writing a custom function available inside a MistQL query:

```js
import {
  MistQLInstance
} from 'mistql';

const mq = new MistQLInstance({
  extras: {
    sumthree: (a, b, c) => a + b + c;
  }
});

console.log(mq.query("sumthree 1 2 3", null)); // Prints 6
```

The above eagerly evaluates the 3 arguments to the `threesum` function and passes them to the JS function. The `extras` key in the options parameter to `MistQLInstance` contains a mapping from lexical name to a function or function definition.

## A level deeper

Since MistQL lazily evaluates subexpressions due to [contextualized expressions](../tutorial/contextualized-expressions.md), simple JS functions don't capture all the semantics necesary to define MistQL functions in all cases. Instead, function semantics are captured in their entirety via FunctionValues:

The type of `FunctionValue` in MistQL is as follows:

```ts
type FunctionValue = (
  args: ASTExpression[],
  stack: Closure[], 
  exec: (args: ASTExpression, stack: Closure[]) => RuntimeValue
) => RuntimeValue
```

The arguments to this function are as follows

| Argument | Meaning |
|---|---|
| `args` | The arguments passed to the function in MistQL. Since these arguments haven't yet been evaluated, they are passed in AST format |
| `stack` | The current lexical scope of the function. This should not be modified. |
| `exec` | A callback method to execute subexpressions under  |

This is the same interface that the MistQL JS implementation uses internally for builtin functions, and as such, you can look at the [builtin implementations](https://github.com/evinism/mistql/tree/main/js/src/builtins) for clean examples on how to write custom functions.

You can pass a function value the instance `extras` keys by wrapping it in a `{definition: fn}` object. For example:

```ts
import {
  MistQLInstance
} from 'mistql';

const functionValue = (args, stack, exec) => (
  args
    .map((arg) => exec(arg, stack))
    .reduce((a, b) => a + b, 0)
);

const mq = new MistQLInstance({
  extras: {
    addall: { 
      definition: (args, stack, exec) => (
        args
          .map((arg) => exec(arg, stack))
          .reduce((a, b) => a + b, 0)
      ),
    }
  }
});

console.log(mq.query("addall 1 2 3", null)); // Prints 6
```
