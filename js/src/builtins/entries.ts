import { BuiltinFunction } from "../types";
import { arity } from "../util";
import { getProperties } from "../runtimeValues";

const entries: BuiltinFunction = arity(1, (args, stack, exec) => {
  const source = exec(args[0], stack);
  return getProperties(source).map((key) => [key, source[key]]);
});

export default entries;
