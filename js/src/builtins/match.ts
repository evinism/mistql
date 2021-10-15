import { getType } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const match: BuiltinFunction = arity(2, (args, stack, exec) => {
  const matcher = exec(args[0], stack);
  const target = validateType("string", exec(args[1], stack));
  if (getType(matcher) === 'regex') {
    return matcher.test(target);
  } else if (getType(matcher) === 'string') {
    return matcher === target;
  } else {
    throw new Error("Matching only works with strings or regexes")
  }
});

export default match;
