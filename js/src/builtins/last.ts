import { BuiltinFunction } from "../types";
import { arity } from "../util";

const last: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arr = exec(args[0], stack);
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[arr.length - 1] ?? null;
});

export default last;