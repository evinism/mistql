# beakerql

## A miniature embeddable query language for JSON-like structures

`beakerql` is a miniature query language built for embedding within applications. It supports
logic for querying and manipulating JSON-like data in a simple manner.

Beaker is built from the ground up to be extremely lightweight. At ~3.5kb gzipped, it can
be included in even extremely size-sensitive frontends.

## Motivation

Having a simple query language allows

- Shared frontend / backend logic for form validation or price calculation.
- User-submitted logic
- Cross-language serializable functions

In the past, I've used JSON Logic for such tasks, but JSON logic leaves a bit to be desired:

- It's not very expressive, and even simple things can prove quite annoying to implement
- It doesn't have a dedicated syntax, making reading and writing complex transforms extremely challenging

## In-language usage:

```js
// JavaScript
import beakerql from 'beakerql';

beakerql.query(query, { events: [...] })
```

## Example usage:

The following are simple examples of how Beaker could be used.

### Get count of a specific event

`events | filter type=="submit" | count`

### Get count of all event types

`events | groupby type | mapvalues count`

### Get the worst chess line possible.

`lines | sort -overallScore | first`

### Get emails of all users that use the Chat feature

`events | filter type="send_message" | groupby email | keys`

### Get usernames of all users who purchased before signing up

`events | sort timestamp | groupby email | mapvalues (sequence type == "purchase", type == "signup") | filtervalues count @ > 0 | keys`

## Builtin Types

Beaker's types correspond closely to JSON types, for interoperability between different languages.

Beaker has 4 primitive types:

- `string`
- `number`
- `null`
- `boolean`

Beaker also has 3 complex types:

- `struct`
- `array`
- `function`

The interface of beaker is restricted in that functions can neither be provided as data, nor returned as the result of a query -- they exist entirely within Beaker

# Reference

The following is a reference of the builtin functions of beakerql

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
sequence
summarize
count
```
