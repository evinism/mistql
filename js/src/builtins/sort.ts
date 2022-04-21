import { comparable, compare } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const sort: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg = validateType("array", exec(args[0], stack));

  if (arg.some(value => !comparable(value))) {
    throw new Error("Cannot sort non-comparable values");
  }
  // default to ascending
  return arg.slice().sort((b, a) => compare(a, b));
});

export default sort;
