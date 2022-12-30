import { execute } from "./executor";
import { parseOrThrow } from "./parser";
import { FunctionClosure, FunctionValue } from "./types";
import { jsFunctionToMistQLFunction } from "./util";

type Extras = {
  [key: string]:
    | ((...args: any[]) => any)
    | {
        definition: FunctionValue;
      };
};

export type MistQLOptions = {
  extras?: Extras;
};

export class MistQLInstance {
  _extras: FunctionClosure;

  constructor(options: MistQLOptions = {}) {
    if (options.extras) {
      this._extras = {};
      for (let i in options.extras) {
        if (options.extras.hasOwnProperty(i)) {
          const value = options.extras[i];
          if (typeof value === "function") {
            this._extras[i] = jsFunctionToMistQLFunction(value);
          } else {
            this._extras[i] = value.definition;
          }
        }
      }
    }
  }

  query = (query: string, data: any) => {
    return execute(parseOrThrow(query), data, this._extras);
  };
}

export const defaultInstance = new MistQLInstance();
