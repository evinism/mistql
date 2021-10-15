import { getType } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";


const replace: BuiltinFunction = arity(3, (args, stack, exec) => {
  const matcher = exec(args[0], stack);
  const replacer = validateType("string", exec(args[1], stack));
  const target = validateType("string", exec(args[2], stack));
  if (getType(matcher) === 'regex' || getType(matcher) === 'string') {
    return target.replace(matcher, replacer);
  } else {
    throw new Error("Matching only works with strings or regexes")
  }
});

export default replace;