import { RuntimeValue, Stack } from "./types";

export const pushRuntimeValueToStack = (runtimeValue: RuntimeValue, stack: Stack): Stack => {
  let nextEntry = {
    "@": runtimeValue,
  }
  // Only structs have self properties that are added to the stack
  if (!Array.isArray(runtimeValue)) {
    for (let i in runtimeValue) {
      if (runtimeValue.hasOwnProperty(i)) {
        nextEntry[i] = runtimeValue[i];
      }
    }
  }
  const nextStack = stack.slice();
  nextStack.push(nextEntry);
  return nextStack;
}