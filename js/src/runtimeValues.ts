import { RuntimeValue } from "./types";

type RuntimeValueTypes = 'array' | 'struct' | 'number' | 'boolean' | 'string' | 'null'

export const truthy = (runtimeValue: RuntimeValue): boolean => {
  if (Array.isArray(runtimeValue)) {
    return !!runtimeValue.length;
  }
  return !!runtimeValue
}

const getType = (a: RuntimeValue): string => {
  if (Array.isArray(a)){
    return 'array';
  } else if (a === null) {
    return 'null';
  } else if (typeof a === 'object') {
    return 'struct';
  } else {
    return typeof a;
  }
}

export const compare = (a: RuntimeValue, b: RuntimeValue): number => {
  const varType = getType(a);
  if (varType !== getType(b)) {
    throw new Error("Type comparison ill defined between different variable types");
  }
  if (varType === 'array') {
    throw new Error("Type comparison betwen arrays not yet implemented");
  } else if(varType === 'struct') {
    throw new Error("Type comparison betwen structs not permitted");
  } else if(varType === 'number') {
    return b - a;
  } else if (varType === 'boolean') {
    return (+b) - (+a);
  } else if (varType === 'string') {
    return a === b ? 0 : (b > a ? 1 : -1);
  } else {
    return 0;
  }
}