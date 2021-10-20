# MistQL

## A miniature embeddable query language for JSON-like structures

`mistql` is a miniature query language built for embedding within applications. It supports
logic for querying and manipulating JSON-like data in a simple manner.

MistQL is built from the ground up to be lightweight. At ~5.2kb gzipped with no dependencies, it can
be included in size-sensitive frontends.

## Motivation

Having a simple JSON query language allows

- Cross-language serializable functions
- Shared frontend / backend logic for form validation or price calculation.
- User-submitted logic

In the past, I've used JSON Logic for such tasks, but JSON logic leaves a bit to be desired:

- It's not very expressive, and even simple things can prove quite annoying to implement
- It doesn't have a dedicated syntax, making reading and writing complex transforms extremely challenging

## In-language usage:

```js
// JavaScript
import mistql from 'mistql';

mistql.query(query, { events: [...] })
```

## Example usage:

The following are simple examples of how MistQL could be used.

### Get count of a specific event

`events | filter type == "submit" | count`

### Get count of all event types

`events | groupby type | mapvalues count`

### Get the worst chess line possible.

`lines | sortby (-overallScore) | first`

### Get emails of all users that use the Chat feature

`events | filter type == "send_message" | groupby email | keys`

### Get usernames of all users who purchased before signing up

`events | sort timestamp | groupby email | mapvalues (sequence type == "purchase", type == "signup") | filtervalues (count @ > 0) | keys`

## Builtin Types

MistQL's types correspond closely to JSON types, for interoperability between different languages.

MistQL has 4 primitive types:

- `string`
- `number`
- `null`
- `boolean`

MistQL also has 3 complex types:

- `object`
- `array`
- `function`
- `regex`

The interface of MistQL is restricted in that functions can neither be provided as data, nor returned as the result of a query -- they exist entirely within MistQL
