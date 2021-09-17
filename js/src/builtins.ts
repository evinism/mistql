import { truthy } from "./runtimeValues";
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
}

// TODO: Make this work with non-string values
const groupby = (args, stack, exec) => {
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
      throw new Error("Expected string for groupBy return value")
    }
    groupings[group] = (groupings[group] || []).concat([innerValue]);
  });
  return groupings;
}

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
}

const find: BuiltinFunction = (args, stack, exec) => {
  return filter(args, stack, exec)[0] || null;
}

const numericBinaryOperator = (op: (a: number, b: number) => number): BuiltinFunction => (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const a = exec(args[0], stack);
  const b = exec(args[1], stack);
  if (typeof a !== "number" || typeof b !== "number") {
    throw new Error("+ does not work with non-numbers");
  }
  return op(a, b);
}

const equal: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 2) {
    throw new Error("Expected 2 arguments");
  }
  const a = exec(args[0], stack);
  const b = exec(args[1], stack);
  if (typeof a !== typeof b) {
    return false
  }
  // TODO: Make equality work for arrays.
  return a === b;
}

const count: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const result = exec(args[0], stack);
  if (!Array.isArray(result)) {
    throw new Error("Expected array");
  }
  return result.length;
}

const keys: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(i)
    }
  }
  return results;
}

const values: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(evaluated[i])
    }
  }
  return results;
}

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
}

const first: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const arr = exec(args[0], stack);
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[0] || null;
}

const last: BuiltinFunction = (args, stack, exec) => {
  if (args.length !== 1) {
    throw new Error("Expected 1 argument");
  }
  const arr = exec(args[0], stack);
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[arr.length - 1] || null;
}

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
  "+": numericBinaryOperator((a, b) => a + b),
  "-": numericBinaryOperator((a, b) => a - b),
  "*": numericBinaryOperator((a, b) => a * b),
  "/": numericBinaryOperator((a, b) => a / b),
  "%": numericBinaryOperator((a, b) => a % b),
  "==": equal,
};