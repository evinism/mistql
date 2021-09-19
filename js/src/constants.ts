// These binary expressions have definitions that span over
// n consecutive items, e.g. function application. Could absolutely be extended
// to all commutative operators, e.g. +, *, &&, and ||. For simplicity, though, pls no.
export const amalgamatingBinaryOperators = [" ", "|"];

export const simpleBinaryOperators = [
  ".",
  "*",
  "/",
  "%",
  "+",
  "-",
  "<",
  ">",
  "<=",
  ">=",
  "==",
  "!=",
  "&&",
  "||",
];

export const binaryExpressionStrings = [].concat(
  simpleBinaryOperators,
  amalgamatingBinaryOperators
);

export const specials = binaryExpressionStrings.concat([
  "(",
  ")",
  "[",
  "]",
  "{",
  "}",
  ":",
  ",",
]);

export const unaryExpressions = ["-", "!"];

export const builtinValues = {
  true: true,
  false: false,
  null: null,
};
