import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const validFlags = /^[gims]*$/

const replace: BuiltinFunction = arity([3, 4], (args, stack, exec) => {
  const regexStr = validateType("string", exec(args[0], stack));
  const replacement = validateType("string", exec(args[1], stack));
  let target: RuntimeValue;
  let flags: string;
  if (args.length === 3) {
    flags = "";
    target = validateType("string", exec(args[2], stack));
  } else {
    flags = validateType("string", exec(args[2], stack));
    if (!validFlags.test(flags)) {
      throw new Error("Invalid flags passed to replace: " + flags);
    }
    target = validateType("string", exec(args[3], stack));
  }
  return target.replace(new RegExp(regexStr, flags), replacement);
});

export default replace;