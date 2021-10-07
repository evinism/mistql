import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity } from "../util";

const mapkeys: BuiltinFunction = arity(2, (args, stack, exec) => {
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      const newKey = exec(args[0], pushRuntimeValueToStack(i, stack));
      results[newKey] = evaluated[i];
    }
  }
  return results;
});

export default mapkeys;