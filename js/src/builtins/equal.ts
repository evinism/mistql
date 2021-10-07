import { getType } from "../runtimeValues";
import { BuiltinFunction } from "../types";
import { arity } from "../util";


const equal: BuiltinFunction = arity(2, (args, stack, exec) => {
  const a = exec(args[0], stack);
  const b = exec(args[1], stack);
  if (getType(a) !== getType(b)) {
    return false;
  }
  // TODO: Make equality work for arrays.
  return a === b;
});

export default equal;
