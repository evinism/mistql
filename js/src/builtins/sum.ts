import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const sum: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg: RuntimeValue = validateType("array", exec(args[0], stack));
  return arg.reduce((a, b) => a + b, 0);
});

export default sum;