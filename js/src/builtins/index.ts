import { RuntimeError } from "../errors";
import { compare, truthy } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";
import and from "./and";
import apply from "./apply";
import count from "./count";
import entries from "./entries";
import equal from "./equal";
import filter from "./filter";
import filterkeys from "./filterkeys";
import filtervalues from "./filtervalues";
import find from "./find";
import flatten from "./flatten";
import float from "./float";
import fromentries from "./fromentries";
import groupby from "./groupby";
import indexFn, {indexInner} from "./indexFn";
import join from "./join";
import keys from "./keys";
import log from "./log";
import map from "./map";
import mapkeys from "./mapkeys";
import mapvalues from "./mapvalues";
import match from "./match";
import not from "./not";
import notequal from "./notequal";
import or from "./or";
import plus from "./plus";
import reduce from "./reduce";
import regex from "./regex";
import replace from "./replace";
import reverse from "./reverse";
import sequence from "./sequence";
import sort from "./sort";
import sortby from "./sortby";
import split from "./split";
import string from "./string";
import sum from "./sum";
import summarize from "./summarize";
import unaryMinus from "./unaryMinus";
import values from "./values";
import withindices from "./withindices";

const numericBinaryOperator = (
  op: (a: number, b: number) => number
): BuiltinFunction =>
  arity(2, (args, stack, exec) => {
    const a = validateType("number", exec(args[0], stack));
    const b = validateType("number", exec(args[1], stack));
    return op(a, b);
  });

const binaryCompareFunction = (
  truthtable: [boolean, boolean, boolean]
): BuiltinFunction =>
  arity(2, (args, stack, exec) => {
    const a = exec(args[0], stack);
    const b = exec(args[1], stack);
    const comparison = compare(a, b);
    if (comparison < 0) {
      return truthtable[0];
    }
    if (comparison === 0) {
      return truthtable[1];
    }
    if (comparison > 0) {
      return truthtable[2];
    }
  });

const dotAccessor: BuiltinFunction = arity(2, (args, stack, exec) => {
  const former = exec(args[0], stack);
  const ref = args[1];
  if (ref.type !== "reference") {
    throw new Error("Only references are allowed as rhs to dot access");
  }
  return indexInner(former, ref.ref, undefined);
});

const ifFunction: BuiltinFunction = arity(3, (args, stack, exec) => {
  return truthy(exec(args[0], stack))
    ? exec(args[1], stack)
    : exec(args[2], stack);
});

const matchBinaryOp: BuiltinFunction = arity(2, (args, stack, exec) =>
  match(args.slice().reverse(), stack, exec)
);

const divide = numericBinaryOperator((a, b) => {
  if (b === 0) {
    throw new RuntimeError("Division by zero");
  }
  return (a / b);
});

export default {
  apply,
  count,
  entries,
  filter,
  filterkeys,
  filtervalues,
  find,
  flatten,
  float,
  fromentries,
  groupby,
  if: ifFunction,
  index: indexFn,
  join,
  stringjoin: join,
  keys,
  log,
  match,
  map,
  mapkeys,
  mapvalues,
  reduce,
  regex,
  replace,
  reverse,
  sequence,
  sort,
  sortby,
  split,
  string,
  sum,
  summarize,
  values,
  withindices,
  "!/unary": not,
  "-/unary": unaryMinus,
  ".": dotAccessor,
  "+": plus,
  "-": numericBinaryOperator((a, b) => a - b),
  "*": numericBinaryOperator((a, b) => a * b),
  "/": divide,
  "%": numericBinaryOperator((a, b) => a % b),
  "||": or,
  "&&": and,
  "==": equal,
  "!=": notequal,
  ">": binaryCompareFunction([true, false, false]),
  "<": binaryCompareFunction([false, false, true]),
  ">=": binaryCompareFunction([true, true, false]),
  "<=": binaryCompareFunction([false, true, true]),
  "=~": matchBinaryOp,
};
