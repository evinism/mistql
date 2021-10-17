import { castToString } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const string: BuiltinFunction = arity(1, (args, stack, exec) =>
  castToString(exec(args[0], stack))
);

export default string;
