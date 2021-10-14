import { getType } from "../runtimeValues";
import { BuiltinFunction, RuntimeValue } from "../types";
import { arity, validateType } from "../util";


const structIndexer = (source: RuntimeValue, key: RuntimeValue) => {
  validateType("string", key);
  return source[key] ?? null;
}

const arrayIndexer = (source: RuntimeValue, key: RuntimeValue) => {
  validateType("number", key);
  if (key < 0) {
    key = key + source.length;
  }
  return source[key] ?? null;
}

const index: BuiltinFunction = arity(2, (args, stack, exec) => {
  let key = exec(args[0], stack);
  const source = exec(args[1], stack);
  if (getType(source) === "struct") {
    return structIndexer(source, key);
  } else if (getType(source) === "array") {
    return arrayIndexer(source, key);
  } else {
    throw new Error("Cannot get index of non-array or non-struct");
  }
});

export default index;