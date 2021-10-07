import { truthy } from "../runtimeValues";
import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity } from "../util";


const filterkeys: BuiltinFunction = arity(2, (args, stack, exec) => {
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (
      evaluated.hasOwnProperty(i) &&
      truthy(exec(args[0], pushRuntimeValueToStack(i, stack)))
    ) {
      results[i] = evaluated[i];
    }
  }
  return results;
});

export default filterkeys;