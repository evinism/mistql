---
sidebar_position: 2
---

# The Context Variable

Every expression is executed with a certain context. The variable `@` refers to the current context of an
expression. The context of a raw expression refers to the entire data object passed into MistQL.

Example:
```js
import mistql from 'mistql';
mistql.query('@', {i: {am: "data"}}); // returns {i: {am: "data"}}
```

If `@` refers to an object, then the keys of the object are populated as variables to the expression context.
```js
import mistql from 'mistql';
mistql.query('(@.foo == foo) && (foo == 42)', {foo: 42}) // returns true
```

Certain functions populate the context variable when executing an expression.
```js
import mistql from 'mistql';
// In the map function below, map computes the expression "@ + 1" multiple times, 
// each time under a different context.
mistql.query('pages | map @ + 1', {pages: [1, 2, 3]}) // returns true
```

Since the keys of the context variable `@` are populated as part of the expression, we can use them directly
for certain functions such as map and filter.
```js
import mistql from 'mistql';

const query = 'events | filter type == "purchase"';
const data = {
  events: [
    {user: 1, type: "view"}
    {user: 2, type: "purchase"}
    {user: 3, type: "view"}
    {user: 4, type: "purchase"}
  ]
}

// Returns [{user: 2, type: "purchase"}, {user: 4, type: "purchase"}]
mistql.query(query, data);
```
