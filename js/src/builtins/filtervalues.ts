import { truthy } from "../runtimeValues";
import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const filtervalues: BuiltinFunction = arity(2, (args, stack, exec) => {
  const evaluated = validateType("object", exec(args[1], stack));
  const results = {};
  for (let i in evaluated) {
    if (
      evaluated.hasOwnProperty(i) &&
      truthy(exec(args[0], pushRuntimeValueToStack(evaluated[i], stack)))
    ) {
      results[i] = evaluated[i];
    }
  }
  return results;
});

export default filtervalues;
