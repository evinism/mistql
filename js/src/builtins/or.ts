import { truthy } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const or: BuiltinFunction =
  arity(2, (args, stack, exec) => {
    const a = exec(args[0], stack);
    const b = exec(args[1], stack);
    return truthy(a) ? a : b;
  });

export default or;
