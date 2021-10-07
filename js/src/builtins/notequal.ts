import { BuiltinFunction } from "../types";
import equal from './equal';

const notequal: BuiltinFunction = (args, stack, exec) => {
  return !equal(args, stack, exec);
};

export default notequal;