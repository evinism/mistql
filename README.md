# MilliEQL
## A miniature embeddable query language

MilliEQL is a miniature query language built for embedding within applications. It supports
logic for querying and manipulating data in a simple, typesafe manner.

## Motivation
Having a simple query language allows 
* Shared frontend / backend logic for form validation or price calculation. 
* User-submitted logic
* Cross-language serializable functions

In the past, I've used JSON Logic for such tasks, but JSON logic leaves a bit to be desired:
* It's not very expressive, and even simple things can prove quite annoying to implement
* It doesn't have a dedicated syntax, making reading and writing complex transforms extremely challenging

## In-language usage:

```js
// JavaScript
import millieql from 'millieql';

millieql.query(query, { events: [...] })
```

```py
# Python
import millieql
millieql.query(query, {"events": [...]})
```



## Example usage:
The following are simple examples of how MilliEQL could be used.

### Get count of a specific event
`events | filter type="submit" | count`

### Get count of all event types
`events | groupby type | mapvalues count`

### Get the worst chess line possible.
`lines | sort -overallScore | first`

### Get emails of all users that use the Chat feature
`events | filter type="send_message" | groupby email | keys`

### Get usernames of all users who purchased before signing up
`events | sort timestamp | groupby email | mapvalues (sequence type == "purchase", type == "signup") | filtervalues count > 0 | keys`

## Builtin Types
MilliEQL's types correspond closely to JSON types, for interoperability between different languages.

MilliEQL has 4 primitive types:
- `string`
- `number`
- `null`
- `boolean`

MilliEQL also has 3 complex types:
- `Struct`
- `Array`
- `Function<T, K>`

The interface of MilliEQL is restricted in that functions can neither be provided as data, nor returned as the result of a query -- they exist entirely within MilliEQL


# Reference
The following is a reference of the builtin functions of MilliEQL

```
keys
values
map
filter
mapvalues
filtervalues
mapkeys
filterkeys
groupby
find
index
first
last
summarize
count
```