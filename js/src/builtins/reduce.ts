import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";


const reduce: BuiltinFunction = arity(3, (args, stack, exec) => {
  const target = validateType("array", exec(args[2], stack));
  const newValue = target.reduce((acc: RuntimeValue, cur: RuntimeValue) => {
    const accCurPair: RuntimeValue = [acc, cur];
    const newStack = pushRuntimeValueToStack(accCurPair, stack);
    return exec(args[0], newStack);
  }, exec(args[1], stack));
  return newValue;
});

export default reduce;