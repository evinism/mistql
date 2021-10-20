import { getType } from "../runtimeValues";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";

type Indexer = (source: RuntimeValue, key: RuntimeValue, keyEnd: RuntimeValue) => RuntimeValue;

const objectIndexer: Indexer = (source, key, keyEnd) => {
  if (keyEnd !== undefined) {
    throw new Error("Index ranges not supported for objects")
  }
  validateType("string", key);
  return source[key] ?? null;
}

const arrayOrStringIndexer: Indexer = (source: string | any[], key, keyEnd) => {
  // negative indices!
  if (key < 0) {
    key = key + source.length;
  }
  if (keyEnd < 0) {
    keyEnd = keyEnd + source.length;
  }
  if (keyEnd === undefined) {
    validateType("number", key);
    return source[key] ?? null;
  }
  if (keyEnd === null) {
    validateType("number", key);
    return source.slice(key);
  }
  if (key === null) {
    validateType("number", keyEnd);
    return source.slice(0, keyEnd);
  }
  validateType("number", key);
  validateType("number", keyEnd);
  return source.slice(key, keyEnd);
}

const indexers: { [key: string]: Indexer } = {
  'object': objectIndexer,
  'array': arrayOrStringIndexer,
  'string': arrayOrStringIndexer,
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
  const sourceType = getType(source);
  const indexer = indexers[sourceType];
  if (!indexer) {
    throw new Error("Cannot get index of type " + sourceType);
  }
  return indexer(source, key, keyEnd)
});

export default index;