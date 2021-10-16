import { getType } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const split: BuiltinFunction = arity(2, (args, stack, exec) => {
  const splitter = exec(args[0], stack);
  const source = validateType("string", exec(args[1], stack));

  if (["string", "regex"].indexOf(getType(splitter)) === -1) {
    throw new Error("Expected string or regex as second argument to split");
  }
  return source.split(splitter);
});

export default split;
