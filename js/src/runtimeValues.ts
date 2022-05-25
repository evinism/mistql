import { OpenAnIssueIfThisOccursError } from "./errors";
import { RuntimeValue, RuntimeValueType } from "./types";

export const truthy = (runtimeValue: RuntimeValue): boolean => {
  const type = getType(runtimeValue);
  if (type === "array") {
    return !!runtimeValue.length;
  } else if (type === "object") {
    return !!getProperties(runtimeValue).length;
  }
  return !!runtimeValue;
};

export const getProperties = (value: RuntimeValue) => {
  if (getType(value) !== "object") {
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

// Differs from parser in 3 ways:
// 1: has (-) sign
// 2: asserts empty character
// 3: allows empty string
// 4. Doesn't require a digit after the . sign
const validNumberFormat =
  /^\s*-?(0|([1-9][0-9]*))(\.[0-9]*)?([eE][+-]?[0-9]+)?\s*$/;

export const castToFloat = (value: RuntimeValue): RuntimeValue => {
  const type = getType(value);
  if (type === "string") {
    if (value.match(validNumberFormat)) {
      return parseFloat(value);
    } else {
      throw new Error("Cannot cast string to float: " + value);
    }
  } else if (
    type === "regex" ||
    typeof value === "function" ||
    type == "object" ||
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
    return "object";
  } else {
    return typeof a as RuntimeValueType;
  }
};

export const comparable = (a: RuntimeValue) => {
  const type = getType(a);
  if (type === "string" || type === "number" || type === "boolean") {
    return true;
  } else {
    return false;
  }
};

export const compare = (a: RuntimeValue, b: RuntimeValue): number => {
  const varType = getType(a);
  if (varType !== getType(b)) {
    throw new Error("Comparison ill-defined between different variable types");
  }
  if (varType === "array") {
    throw new Error("Comparison between arrays not permitted");
  } else if (varType === "object") {
    throw new Error("Comparison between objects not permitted");
  } else if (varType === "regex") {
    throw new Error("Comparison between regexes not permitted");
  } else if (varType === "null") {
    throw new Error("Comparison between nulls not permitted");
  } else if (varType === "number") {
    return b - a;
  } else if (varType === "boolean") {
    return +b - +a;
  } else if (varType === "string") {
    return a === b ? 0 : b > a ? 1 : -1;
  }
  throw new OpenAnIssueIfThisOccursError("Unknown type " + varType);
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
  } else if (type === "object") {
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
