import { RuntimeError } from "../errors";
import { getType } from "../runtimeValues";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

type Indexer = (source: RuntimeValue, key: RuntimeValue, keyEnd: RuntimeValue) => RuntimeValue;


const hasSurrogatePairs = (str: string) => {
  for (let i = 0; i < str.length; i++) {
    if (str.codePointAt(i) > 0xffff) {
      return true;
    }
  }
  return false;
};

const objectIndexer: Indexer = (source, key, keyEnd) => {
  if (keyEnd !== undefined) {
    throw new RuntimeError("Index ranges not supported for objects");
  }
  validateType("string", key);
  return source[key] ?? null;
};

const arrayOrStringSubscript = (data: any[] | string, key: number) => {
  if (typeof data === "string" && hasSurrogatePairs(data)) {
    let i = 0;
    for (let codepoint of data) {
      if (i === key) {
        return codepoint;
      }
      i++;
    }
    return null;
  }
  return data[key];
};

const arrayOrStringSlice = (
  data: any[] | string,
  start: number,
  end?: number
) => {
  if (typeof data === "string" && hasSurrogatePairs(data)) {
    let i = 0;
    let result = "";
    for (let codepoint of data) {
      if (i >= end) {
        break;
      } else if (i >= start) {
        result += codepoint;
      }
      i++;
    }
    return result;
  }
  return data.slice(start, end);
};

const arrayOrStringIndexer: Indexer = (source: string | any[], key, keyEnd) => {
  // negative indices!
  if (key < 0) {
    key = key + source.length;
  }
  if (keyEnd < 0) {
    keyEnd = keyEnd + source.length;
  }
  if ((key && key % 1 !== 0) || (keyEnd && keyEnd % 1 !== 0)) {
    throw new RuntimeError("Index must be an integer");
  }
  if (keyEnd === undefined) {
    validateType("number", key);
    return arrayOrStringSubscript(source, key) ?? null;
  }
  if (keyEnd === null) {
    validateType("number", key);
    return arrayOrStringSlice(source, key);
  }
  if (key === null) {
    validateType("number", keyEnd);
    return arrayOrStringSlice(source, 0, keyEnd);
  }
  validateType("number", key);
  validateType("number", keyEnd);
  return arrayOrStringSlice(source, key, keyEnd);
};

const nullIndexer: Indexer = (_, key, keyEnd) => {
  if (keyEnd !== undefined) {
    throw new RuntimeError("Index ranges not supported for null")
  }
  if (getType(key) !== "number" && getType(key) !== "string") {
    throw new RuntimeError("Index must be a number or string");
  };
  return null;
}


const indexers: { [key: string]: Indexer } = {
  'object': objectIndexer,
  'array': arrayOrStringIndexer,
  'string': arrayOrStringIndexer,
  'null': nullIndexer,
}

export const indexInner = (
  source: RuntimeValue, 
  key: RuntimeValue, 
  end: RuntimeValue
): RuntimeValue => {
  const sourceType = getType(source);
  const indexer = indexers[sourceType];
  if (!indexer) {
    throw new RuntimeError("Cannot get index of type " + sourceType);
  }
  return indexer(source, key, end);
}


const index: BuiltinFunction = arity([2, 3], (args, stack, exec) => {
  let key = exec(args[0], stack);
  let keyEnd: RuntimeValue;
  let source: RuntimeValue;
  if (args.length === 2) {
    keyEnd = undefined;
    source = exec(args[1], stack);
  } else {
    keyEnd = exec(args[1], stack);
    source = exec(args[2], stack);
  }
  return indexInner(source, key, keyEnd)
});

export default index;