import { getProperties } from "./runtimeValues";
import { RuntimeValue, Stack } from "./types";

export const pushRuntimeValueToStack = (
  runtimeValue: RuntimeValue,
  stack: Stack
): Stack => {
  let nextEntry = {
    "@": runtimeValue,
  };
  getProperties(runtimeValue).forEach((key) => {
    nextEntry[key] = runtimeValue[key];
  });
  const nextStack = stack.slice();
  nextStack.push(nextEntry);
  return nextStack;
};
