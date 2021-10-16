import { getProperties } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const values: BuiltinFunction = arity(1, (args, stack, exec) => {
  const evaluated = exec(args[0], stack);
  return getProperties(evaluated).map((key) => evaluated[key]);
});

export default values;
