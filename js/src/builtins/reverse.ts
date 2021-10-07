import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const reverse: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg = validateType("array", exec(args[0], stack));
  return arg.reverse();
});

export default reverse;