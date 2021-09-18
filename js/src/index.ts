import { execute } from "./executor";
import { parseOrThrow } from "./parser";

export const query = (query: string, data: any) => {
  return execute(parseOrThrow(query), data);
};

export default { query };
