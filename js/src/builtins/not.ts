import { truthy } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const not: BuiltinFunction = arity(1, (args, stack, exec) => {
  return !truthy(validateType("boolean", exec(args[0], stack)));
});

export default not;