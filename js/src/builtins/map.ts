import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const map: BuiltinFunction = arity(2, (args, stack, exec) => {
  const target = validateType("array", exec(args[1], stack));
  const newValue = target.map((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    return exec(args[0], newStack);
  });
  return newValue;
});

export default map;