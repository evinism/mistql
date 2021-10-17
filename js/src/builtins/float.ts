import { castToFloat } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const float: BuiltinFunction = arity(1, (args, stack, exec) =>
  castToFloat(exec(args[0], stack))
);

export default float;
