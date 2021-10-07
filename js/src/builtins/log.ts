import { BuiltinFunction } from "../types";
import { arity } from "../util";

const log: BuiltinFunction = arity(1, (args, stack, exec) => {
  const res = exec(args[0], stack);
  console.log("MistQL: " + JSON.stringify(res, null, 2));
  return res;
});

export default log;