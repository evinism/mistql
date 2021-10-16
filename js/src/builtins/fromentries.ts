import { castToString } from "../runtimeValues";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const fromentries: BuiltinFunction = arity(1, (args, stack, exec) => {
  const source = validateType("array", exec(args[0], stack));
  const retval: { [key: string]: RuntimeValue } = {};
  source.forEach((item) => {
    const [key = null, value = null] = validateType("array", item);
    retval[castToString(key)] = value;
  });
  return retval;
});

export default fromentries;
