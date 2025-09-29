import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";
import { castToString, getProperties } from "../runtimeValues";

const mapkeys: BuiltinFunction = arity(2, (args, stack, exec) => {
  const evaluated = validateType("object", exec(args[1], stack));
  const results = {};
  getProperties(evaluated).forEach((key) => {
    const newKey = castToString(
      exec(args[0], pushRuntimeValueToStack(key, stack))
    );
    results[newKey] = evaluated[key];
  });
  return results;
});

export default mapkeys;
