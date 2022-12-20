import { execute } from "./executor";
import { parseOrThrow } from "./parser";
import { FunctionClosure } from "./types";

export type MistQLOptions = {
  extras?: FunctionClosure;
};

export class MistQLInstance {
  extras?: FunctionClosure;

  constructor(options: MistQLOptions = {}) {
    this.extras = options.extras;
  }

  query = (query: string, data: any) => {
    return execute(parseOrThrow(query), data, this.extras);
  };
}

export const defaultInstance = new MistQLInstance();
