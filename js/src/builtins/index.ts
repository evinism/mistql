import { compare, truthy } from "../runtimeValues";
import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, seqHelper, validateType } from "../util";
import count from './count';
import equal from './equal';
import filter from './filter';
import filterkeys from './filterkeys';
import filtervalues from './filtervalues';
import find from './find';
import groupby from './groupby';
import keys from './keys';
import map from './map';
import mapvalues from './mapvalues';
import not from './not';
import notequal from './notequal';
import plus from './plus';
import reduce from './reduce';
import summarize from './summarize';
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

const mapkeys: BuiltinFunction = arity(2, (args, stack, exec) => {
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      const newKey = exec(args[0], pushRuntimeValueToStack(i, stack));
      results[newKey] = evaluated[i];
    }
  }
  return results;
});

const index: BuiltinFunction = arity(2, (args, stack, exec) => {
  const idx = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr[idx] ?? null;
});

const head: BuiltinFunction = arity(2, (args, stack, exec) => {
  const count = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr.slice(0, count);
});

const tail: BuiltinFunction = arity(2, (args, stack, exec) => {
  const count = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr.slice(arr.length - count, arr.length);
});

const first: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arr = validateType("array", exec(args[0], stack));
  return arr[0] ?? null;
});

const last: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arr = exec(args[0], stack);
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[arr.length - 1] ?? null;
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

const sort: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg = validateType("array", exec(args[0], stack));
  // default to ascending
  return arg.slice().sort((b, a) => compare(a, b));
});

const sortby: BuiltinFunction = arity(2, (args, stack, exec) => {
  const arg = validateType("array", exec(args[1], stack));
  return arg.slice().map((item) => {
    const sortValue = exec(args[0], pushRuntimeValueToStack(item, stack))
    return ({ sortValue, item })
  }).sort(({ sortValue: a }, { sortValue: b }) => compare(b, a)).map(({ item }) => item);
});

const reverse: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg = validateType("array", exec(args[0], stack));
  return arg.reverse();
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

const sum: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg: RuntimeValue = validateType("array", exec(args[0], stack));
  return arg.reduce((a, b) => a + b, 0);
});

const ifFunction: BuiltinFunction = arity(3, (args, stack, exec) => {
  return truthy(exec(args[0], stack))
    ? exec(args[1], stack)
    : exec(args[2], stack);
});

const log: BuiltinFunction = arity(1, (args, stack, exec) => {
  const res = exec(args[0], stack);
  console.log("MistQL: " + JSON.stringify(res, null, 2));
  return res;
});

const sequence: BuiltinFunction = (args, stack, exec) => {
  if (args.length < 3) {
    throw new Error("Expected at least 3 arguments, got " + args.length);
  }
  const target: RuntimeValue[] = validateType("array", exec(args[args.length - 1], stack));
  const fns = args.slice(0, args.length - 1);
  const booleanMap = fns.map((fn) =>
    target.map(value => truthy(exec(fn, pushRuntimeValueToStack(value, stack))))
  );
  const seq = seqHelper(booleanMap);
  return seq.map((inner) => inner.map(idx => target[idx]));
}

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
