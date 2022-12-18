import { pushRuntimeValueToStack } from "../stackManip";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

const validateIsInteger = (value: RuntimeValue) => {
  const res = validateType("number", value);
  if (!Number.isInteger(value)) {
    throw new Error("Arguments to range must be integers");
  }
  return res;
};

const range: BuiltinFunction = arity([1, 2, 3], (args, stack, exec) => {
  let start = 0;
  let end;
  const target = [];
  if (args.length > 1) {
    start = validateIsInteger(exec(args[0], stack));
    end = validateIsInteger(exec(args[1], stack));
  } else {
    end = validateIsInteger(exec(args[0], stack));
  }
  // Step size
  let step = 1;
  if (args.length > 2) {
    step = validateIsInteger(exec(args[2], stack));
  }
  // Iteration Methods
  if (step === 0) {
    throw new Error("Range: Step size cannot be 0");
  } else if (step > 0 && start < end) {
    for (let i = start; i < end; i += step) {
      target.push(i);
    }
  } else if (step < 0 && start > end) {
    for (let i = start; i > end; i += step) {
      target.push(i);
    }
  } else {
    return [];
  }
  return target;
});

export default range;
