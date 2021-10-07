import { BuiltinFunction } from "../types";
import { arity } from "../util";


const keys: BuiltinFunction = arity(1, (args, stack, exec) => {
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(i);
    }
  }
  return results;
});

export default keys;