import { RuntimeValue } from "./types";

export const truthy = (runtimeValue: RuntimeValue): boolean => {
  if (Array.isArray(runtimeValue)) {
    return !!runtimeValue.length;
  }
  return !!runtimeValue
}