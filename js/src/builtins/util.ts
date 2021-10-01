import { getType, RuntimeValueType } from "../runtimeValues";
import { BuiltinFunction, RuntimeValue } from "../types";

export const arity =
  (arityCount: number, fn: BuiltinFunction): BuiltinFunction =>
    (args, stack, exec) => {
      if (args.length !== arityCount) {
        throw new Error(
          "Expected " + arityCount + " arguments, got " + args.length
        );
      }
      return fn(args, stack, exec);
    };

export const validateType = (
  type: RuntimeValueType,
  value: RuntimeValue
): RuntimeValue => {
  if (getType(value) !== type) {
    throw new Error("Expected type " + type + ", got " + getType(value));
  }
  return value;
};