import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const map: BuiltinFunction = arity(2, (args, stack, exec) => {
  const target = exec(args[1], stack);
  if (!Array.isArray(target)) {
    throw new Error("Expected array");
  }
  const newValue = target.map((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    return exec(args[0], newStack);
  });
  return newValue;
});

export default map;