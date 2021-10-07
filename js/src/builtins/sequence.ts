import { truthy } from "../runtimeValues";
import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction, RuntimeValue } from "../types";
import { seqHelper, validateType } from "../util";


const sequence: BuiltinFunction = (args, stack, exec) => {
  if (args.length < 3) {
    throw new Error("Expected at least 3 arguments, got " + args.length);
  }
  const target: RuntimeValue[] = validateType("array", exec(args[args.length - 1], stack));
  const fns = args.slice(0, args.length - 1);
  const booleanMap = fns.map((fn) =>
    target.map(value => truthy(exec(fn, pushRuntimeValueToStack(value, stack))))
  );
  const seq = seqHelper(booleanMap);
  return seq.map((inner) => inner.map(idx => target[idx]));
}

export default sequence;