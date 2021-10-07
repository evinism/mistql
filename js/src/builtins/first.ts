import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const first: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arr = validateType("array", exec(args[0], stack));
  return arr[0] ?? null;
});

export default first;