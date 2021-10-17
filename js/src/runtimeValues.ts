import { RuntimeValue } from "./types";

// TODO: We also have functions here!!
export type RuntimeValueType =
  | "array"
  | "struct"
  | "regex"
  | "number"
  | "boolean"
  | "string"
  | "function"
  | "null";

export const truthy = (runtimeValue: RuntimeValue): boolean => {
  if (Array.isArray(runtimeValue)) {
    return !!runtimeValue.length;
  }
  return !!runtimeValue;
};

export const getProperties = (value: RuntimeValue) => {
  if (getType(value) !== "struct") {
    return [];
  } else {
    const retval: string[] = [];
    for (let i in value) {
      retval.push(i);
    }
    return retval.sort();
  }
};

export const castToString = (value: RuntimeValue): RuntimeValue => {
  const type = getType(value);
  if (type === "string") {
    return value;
  } else if (type === "regex" || typeof value === "function") {
    throw new Error("Cannot cast type " + type + " to string");
  } else {
    return JSON.stringify(value);
  }
};

export const castToFloat = (value: RuntimeValue): RuntimeValue => {
  const type = getType(value);
  if (type === "string") {
    return parseFloat(value);
  } else if (
    type === "regex" ||
    typeof value === "function" ||
    type == "struct" ||
    type === "array"
  ) {
    throw new Error("Cannot cast type " + type + " to float");
  } else {
    return +value;
  }
};

export const getType = (a: RuntimeValue): RuntimeValueType => {
  if (Array.isArray(a)) {
    return "array";
  } else if (a === null) {
    return "null";
  } else if (a instanceof RegExp) {
    return "regex";
  } else if (typeof a === "object") {
    return "struct";
  } else {
    return typeof a as RuntimeValueType;
  }
};

export const compare = (a: RuntimeValue, b: RuntimeValue): number => {
  const varType = getType(a);
  if (varType !== getType(b)) {
    throw new Error("Comparison ill-defined between different variable types");
  }
  if (varType === "array") {
    throw new Error("Comparison between arrays not permitted");
  } else if (varType === "struct") {
    throw new Error("Comparison between structs not permitted");
  } else if (varType === "regex") {
    throw new Error("Comparison between regexes not permitted");
  } else if (varType === "number") {
    return b - a;
  } else if (varType === "boolean") {
    return +b - +a;
  } else if (varType === "string") {
    return a === b ? 0 : b > a ? 1 : -1;
  } else {
    return 0;
  }
};

export const equal = (a: RuntimeValue, b: RuntimeValue): boolean => {
  const aType = getType(a);
  const bType = getType(b);
  if (aType !== bType) {
    return false;
  }
  const type = aType;
  if (type === "array") {
    if (a.length !== b.length) {
      return false;
    }
    for (let i = 0; i < a.length; i++) {
      if (!equal(a[i], b[i])) {
        return false;
      }
    }
    return true;
  } else if (type === "struct") {
    const aProps = getProperties(a);
    const bProps = getProperties(b);
    if (aProps.length !== bProps.length) {
      return false;
    }
    for (let i = 0; i < aProps.length; i++) {
      if (aProps[i] !== bProps[i]) {
        return false;
      }
      const key = aProps[i];
      if (!equal(a[key], b[key])) {
        return false;
      }
    }
    return true;
  } else if (type === "regex") {
    return a.flags === b.flags && a.source === b.source;
  } else {
    return a === b;
  }
};
