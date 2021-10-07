import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const count: BuiltinFunction = arity(1, (args, stack, exec) => {
  const result = validateType("array", exec(args[0], stack));
  return result.length;
});

export default count;