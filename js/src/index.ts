import {
  defaultInstance as DI,
  MistQLInstance as MQI,
  MistQLOptions as MQO,
} from "./instance";
export {
  FunctionValue,
  RuntimeValue,
  RuntimeValueType,
  FunctionClosure,
  ASTExpression,
} from "./types";

export const MistQLInstance = MQI;
export const defaultInstance = DI;
export const query = DI.query;

export type MistQLOptions = MQO;

export default { query, defaultInstance, MistQLInstance };
