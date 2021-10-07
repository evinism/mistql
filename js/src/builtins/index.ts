import { compare, truthy } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";
import count from './count';
import equal from './equal';
import filter from './filter';
import filterkeys from './filterkeys';
import filtervalues from './filtervalues';
import find from './find';
import first from './first';
import groupby from './groupby';
import head from './head';
import keys from './keys';
import last from './last';
import log from './log';
import map from './map';
import mapkeys from './mapkeys';
import mapvalues from './mapvalues';
import not from './not';
import notequal from './notequal';
import plus from './plus';
import reduce from './reduce';
import reverse from './reverse';
import sequence from './sequence';
import sort from './sort';
import sortby from './sortby';
import sum from './sum';
import summarize from './summarize';
import tail from './tail';
import unaryMinus from './unaryMinus';
import values from './values';

const numericBinaryOperator = (
  op: (a: number, b: number) => number
): BuiltinFunction =>
  arity(2, (args, stack, exec) => {
    const a = validateType("number", exec(args[0], stack));
    const b = validateType("number", exec(args[1], stack));
    return op(a, b);
  });

const booleanBinaryOperator = (
  op: (a: boolean, b: boolean) => boolean
): BuiltinFunction =>
  arity(2, (args, stack, exec) => {
    const a = validateType("boolean", exec(args[0], stack));
    const b = validateType("boolean", exec(args[1], stack));
    return op(a, b);
  });

const index: BuiltinFunction = arity(2, (args, stack, exec) => {
  const idx = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr[idx] ?? null;
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
  // Arrays and strings have ownProperties that shouldn't be accessible.
  // TODO: Abstract this logic out.
  if (Array.isArray(former) || typeof former === 'string' || former === null) {
    return null;
  }
  if (former.hasOwnProperty(ref.ref)) {
    return former[ref.ref];
  }
  return null;
});

const ifFunction: BuiltinFunction = arity(3, (args, stack, exec) => {
  return truthy(exec(args[0], stack))
    ? exec(args[1], stack)
    : exec(args[2], stack);
});

export default {
  count,
  filter,
  filterkeys,
  filtervalues,
  find,
  first,
  groupby,
  head,
  if: ifFunction,
  index,
  keys,
  last,
  log,
  map,
  mapkeys,
  mapvalues,
  reduce,
  reverse,
  sequence,
  sort,
  sortby,
  sum,
  summarize,
  tail,
  values,
  "!/unary": not,
  "-/unary": unaryMinus,
  ".": dotAccessor,
  "+": plus,
  "-": numericBinaryOperator((a, b) => a - b),
  "*": numericBinaryOperator((a, b) => a * b),
  "/": numericBinaryOperator((a, b) => a / b),
  "%": numericBinaryOperator((a, b) => a % b),
  "||": booleanBinaryOperator((a, b) => a || b),
  "&&": booleanBinaryOperator((a, b) => a && b),
  "==": equal,
  "!=": notequal,
  ">": binaryCompareFunction([true, false, false]),
  "<": binaryCompareFunction([false, false, true]),
  ">=": binaryCompareFunction([true, true, false]),
  "<=": binaryCompareFunction([false, true, true]),
};
