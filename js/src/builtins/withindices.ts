import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const withindices: BuiltinFunction = arity(1, (args, stack, exec) => {
  return validateType("array", exec(args[0], stack)).map((innerValue: RuntimeValue, index: number) => {
    return [index, innerValue]
  });
});

export default withindices;