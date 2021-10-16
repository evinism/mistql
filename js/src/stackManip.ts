import { getProperties } from "./runtimeValues";
import { RuntimeValue, Stack } from "./types";

const refRegex = /^[a-zA-Z_][a-zA-Z_0-9]*$/;

export const pushRuntimeValueToStack = (
  runtimeValue: RuntimeValue,
  stack: Stack
): Stack => {
  let nextEntry = {
    "@": runtimeValue,
  };
  getProperties(runtimeValue).forEach((key) => {
    if (refRegex.test(key)) {
      nextEntry[key] = runtimeValue[key];
    }
  });
  const nextStack = stack.slice();
  nextStack.push(nextEntry);
  return nextStack;
};
