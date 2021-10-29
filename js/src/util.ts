import { RuntimeError } from "./errors";
import { getType } from "./runtimeValues";
import { BuiltinFunction, RuntimeValue, RuntimeValueType } from "./types";

export function escapeRegExp(string: string): string {
  return string.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"); // $& means the whole matched string
}

export const seqHelper = (arr: boolean[][], start = 0): number[][] => {
  const firstArray = arr[0];
  const result: number[][] = [];
  for (let idx = start; idx < firstArray.length; idx++) {
    if (firstArray[idx]) {
      if (arr.length === 1) {
        result.push([idx]);
      } else {
        const subResult = seqHelper(arr.slice(1), idx + 1);
        for (let i = 0; i < subResult.length; i++) {
          result.push([idx].concat(subResult[i]));
        }
      }
    }
  }
  return result;
};

// Builtin Helpers
export const arity =
  (arityCount: number | number[], fn: BuiltinFunction): BuiltinFunction =>
    (args, stack, exec) => {
      const validArity =
        typeof arityCount === "number"
          ? arityCount === args.length
          : arityCount.indexOf(args.length) !== -1;
      if (!validArity) {
        throw new RuntimeError(
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
    throw new RuntimeError("Expected type " + type + ", got " + getType(value));
  }
  return value;
};
