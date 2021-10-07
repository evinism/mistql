import { BuiltinFunction } from "../types";
import { arity } from "../util";

const values: BuiltinFunction = arity(1, (args, stack, exec) => {
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(evaluated[i]);
    }
  }
  return results;
});

export default values;