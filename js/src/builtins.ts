import { compare, truthy } from "./runtimeValues";
import { pushRuntimeValueToStack } from "./stackManip";
import { BuiltinFunction, RuntimeValue } from "./types";

const map: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const fnExp = args[0];
  const targetExp = args[1];
  const target = exec(targetExp, stack);
  if (!Array.isArray(target)) {
    throw new Error("Expected array");
  }
  const newValue = target.map((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    return exec(fnExp, newStack);
  });
  return newValue;
};

// TODO: Make this work with non-string values
const groupby: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const fnExp = args[0];
  const targetExp = args[1];
  const target = exec(targetExp, stack);
  if (!Array.isArray(target)) {
    throw new Error("Expected array");
  }
  const groupings: { [key: string]: RuntimeValue } = {};
  target.forEach((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    const group = exec(fnExp, newStack);
    if (typeof group !== "string") {
      throw new Error("Expected string for groupBy return value");
    }
    groupings[group] = (groupings[group] || []).concat([innerValue]);
  });
  return groupings;
};

const filter: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const fnExp = args[0];
  const targetExp = args[1];
  const target = exec(targetExp, stack);
  if (!Array.isArray(target)) {
    throw new Error("Expected array");
  }
  const newValue = target.filter((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    return truthy(exec(fnExp, newStack));
  });
  return newValue;
};

const find: BuiltinFunction = (args, stack, exec) => {
  return filter(args, stack, exec)[0] || null;
};

const numericBinaryOperator =
  (op: (a: number, b: number) => number): BuiltinFunction =>
  (args, stack, exec) => {
    if (args.length !== 2) {
      throw new Error("Expected 2 arguments");
    }
    const a = exec(args[0], stack);
    const b = exec(args[1], stack);
    if (typeof a !== "number" || typeof b !== "number") {
      throw new Error("+ does not work with non-numbers");
    }
    return op(a, b);
  };

const booleanBinaryOperator =
  (op: (a: boolean, b: boolean) => boolean): BuiltinFunction =>
  (args, stack, exec) => {
    if (args.length !== 2) {
      throw new Error("Expected 2 arguments");
    }
    const a = exec(args[0], stack);
    const b = exec(args[1], stack);
    if (typeof a !== "boolean" || typeof b !== "boolean") {
      throw new Error("+ does not work with non-numbers");
    }
    return op(a, b);
  };

const equal: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const a = exec(args[0], stack);
  const b = exec(args[1], stack);
  if (typeof a !== typeof b) {
    return false;
  }
  // TODO: Make equality work for arrays.
  return a === b;
};

const notequal: BuiltinFunction = (args, stack, exec) => {
  return !equal(args, stack, exec);
};

const count: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const result = exec(args[0], stack);
  if (!Array.isArray(result)) {
    throw new Error("Expected array");
  }
  return result.length;
};

const keys: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(i);
    }
  }
  return results;
};

const values: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(evaluated[i]);
    }
  }
  return results;
};

const index: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const idx = exec(args[0], stack);
  const arr = exec(args[1], stack);
  if (typeof idx !== "number") {
    throw new Error("Expected number");
  }
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[idx] || null;
};

const first: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const arr = exec(args[0], stack);
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[0] || null;
};

const last: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const arr = exec(args[0], stack);
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[arr.length - 1] || null;
};

const binaryCompareFunction =
  (truthtable: [boolean, boolean, boolean]): BuiltinFunction =>
  (args, stack, exec) => {
    if (args.length !== 2) {
      throw new Error("Expected 2 argument");
    }
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
  };

const sort: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const arg = exec(args[0], stack);
  if (!Array.isArray(arg)) {
    throw new Error("Expected array");
  }
  // default to ascending -- should really figure out something here.
  return arg.slice().sort((a, b) => compare(b, a));
};

const reverse: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const arg = exec(args[0], stack);
  if (!Array.isArray(arg)) {
    throw new Error("Expected array");
  }
  return arg.reverse();
};

const dotAccessor: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const former = exec(args[0], stack);
  if (args[1].type !== "reference") {
    throw new Error("Only references are allowed as rhs to dot access");
  }
  const latter = exec(args[1], pushRuntimeValueToStack(former, []));
  return latter;
};

export default {
  map,
  filter,
  find,
  count,
  keys,
  values,
  groupby,
  index,
  first,
  last,
  sort,
  reverse,
  ".": dotAccessor,
  "+": numericBinaryOperator((a, b) => a + b),
  "-": numericBinaryOperator((a, b) => a - b),
  "*": numericBinaryOperator((a, b) => a * b),
  "/": numericBinaryOperator((a, b) => a / b),
  "%": numericBinaryOperator((a, b) => a % b),
  "||": booleanBinaryOperator((a, b) => a || b),
  "&&": booleanBinaryOperator((a, b) => a && b),
  "==": equal,
  "!=": equal,
  ">": binaryCompareFunction([true, false, false]),
  "<": binaryCompareFunction([false, false, true]),
  ">=": binaryCompareFunction([true, true, false]),
  "<=": binaryCompareFunction([false, true, true]),
};
