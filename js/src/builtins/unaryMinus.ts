import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";


const unaryMinus: BuiltinFunction = arity(1, (args, stack, exec) => {
  return -validateType("number", exec(args[0], stack));
});

export default unaryMinus;