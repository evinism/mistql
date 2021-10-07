import { truthy } from "../runtimeValues";
import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const filter: BuiltinFunction = arity(2, (args, stack, exec) => {
  const target = validateType("array", exec(args[1], stack));
  const newValue = target.filter((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    return truthy(exec(args[0], newStack));
  });
  return newValue;
});

export default filter;