import { pushRuntimeValueToStack } from "./stackManip";
import { BuiltinFunction } from "./types";

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

export default {
  map,
  "+": numericBinaryOperator((a, b) => a + b),
  "-": numericBinaryOperator((a, b) => a - b),
  "*": numericBinaryOperator((a, b) => a * b),
  "/": numericBinaryOperator((a, b) => a / b),
};