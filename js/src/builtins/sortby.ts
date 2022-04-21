import { comparable, compare } from "../runtimeValues";
import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const sortby: BuiltinFunction = arity(2, (args, stack, exec) => {
  const arg = validateType("array", exec(args[1], stack));
  return arg.slice().map((item) => {
    const sortValue = exec(args[0], pushRuntimeValueToStack(item, stack))

    if (!comparable(sortValue)) {
      throw new Error("Cannot sort non-comparable values");
    }

    return ({ sortValue, item })
  }).sort(({ sortValue: a }, { sortValue: b }) => compare(b, a)).map(({ item }) => item);
});

export default sortby;
