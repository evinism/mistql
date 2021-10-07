import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const head: BuiltinFunction = arity(2, (args, stack, exec) => {
  const count = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr.slice(0, count);
});

export default head;