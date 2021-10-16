import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity } from "../util";
import { getProperties } from "../runtimeValues";

const mapvalues: BuiltinFunction = arity(2, (args, stack, exec) => {
  const evaluated = exec(args[1], stack);
  const results = {};
  getProperties(evaluated).forEach((key) => {
    results[key] = exec(
      args[0],
      pushRuntimeValueToStack(evaluated[key], stack)
    );
  });
  return results;
});

export default mapvalues;
