import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const flatten: BuiltinFunction = arity(1, (args, stack, exec) => {
  const target = validateType("array", exec(args[0], stack));
  const newValue = target.reduce(
    (acc: unknown[], cur: unknown) => 
      acc.concat(validateType("array", cur)), 
    []
  );
  return newValue;
});

export default flatten;