import { BuiltinFunction } from "../types";
import filter from './filter';

const find: BuiltinFunction = (args, stack, exec) => {
  return filter(args, stack, exec)[0] ?? null;
};

export default find;