import { castToString } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const split: BuiltinFunction = arity(2, (args, stack, exec) => {
  const joiner = validateType("string", exec(args[0], stack));
  const source = validateType("array", exec(args[1], stack));
  return source.map(castToString).join(joiner);
});

export default split;
