import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const map: BuiltinFunction = arity(2, (args, stack, exec) => {
  const target = exec(args[1], stack);
  const newStack = pushRuntimeValueToStack(target, stack);
  return exec(args[0], newStack);
});

export default map;
