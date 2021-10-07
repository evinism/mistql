import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const mapvalues: BuiltinFunction = arity(2, (args, stack, exec) => {
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results[i] = exec(args[0], pushRuntimeValueToStack(evaluated[i], stack));
    }
  }
  return results;
});

export default mapvalues;