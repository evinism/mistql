import { castToString } from "../runtimeValues";
import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const groupby: BuiltinFunction = arity(2, (args, stack, exec) => {
  const target = validateType("array", exec(args[1], stack));
  const groupings: { [key: string]: RuntimeValue } = {};
  target.forEach((innerValue: RuntimeValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    const group = castToString(exec(args[0], newStack));
    groupings[group] = (groupings[group] || []).concat([innerValue]);
  });
  return groupings;
});

export default groupby;
