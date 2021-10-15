import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const validFlags = /^[gims]*$/

const match: BuiltinFunction = arity([2, 3], (args, stack, exec) => {
  const regexStr = validateType("string", exec(args[0], stack));
  let target: RuntimeValue;
  let flags: string;
  if (args.length === 2) {
    flags = "";
    target = validateType("string", exec(args[1], stack));
  } else {
    flags = validateType("string", exec(args[1], stack));
    if (!validFlags.test(flags)) {
      throw new Error("Invalid flags passed to replace: " + flags);
    }
    target = validateType("string", exec(args[2], stack));
  }
  return (new RegExp(regexStr, flags)).test(target);
});

export default match;