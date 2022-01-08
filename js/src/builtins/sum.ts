import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const sum: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg: RuntimeValue = validateType("array", exec(args[0], stack));
  return arg.reduce((acc, cur) => acc + validateType("number", cur), 0);
});

export default sum;