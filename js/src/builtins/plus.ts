import { getType } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const plus: BuiltinFunction = arity(2, (args, stack, exec) => {
  const a = exec(args[0], stack);
  const b = exec(args[1], stack);
  const type = getType(a);
  if (type !== getType(b)) {
    throw new Error("Cannot add values of different types");
  }
  if (type === "array") {
    return [].concat(a, b)
  }
  if (type !== 'string' && type !== 'number') {
    throw new Error("Cannot add values of type " + type)
  }
  return a + b;
});

export default plus;
