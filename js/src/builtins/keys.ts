import { BuiltinFunction } from "../types";
import { arity } from "../util";
import { getProperties } from "../runtimeValues";

const keys: BuiltinFunction = arity(1, (args, stack, exec) => {
  return getProperties(exec(args[0], stack));
});

export default keys;
