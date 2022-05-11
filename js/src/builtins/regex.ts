import { RuntimeError } from "../errors";
import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";

const validFlags = /^[gims]*$/

const regex: BuiltinFunction = arity([1, 2], (args, stack, exec) => {
  const regexStr = validateType("string", exec(args[0], stack));
  let flags: string;
  if (args.length === 1) {
    flags = "";
  } else {
    flags = validateType("string", exec(args[1], stack));
    if (!validFlags.test(flags)) {
      throw new RuntimeError("Invalid flags passed to replace: " + flags);
    }
  }
  // In MistQL, all strings should operate in unicode fashion.
  return new RegExp(regexStr, flags + "u");
});

export default regex;