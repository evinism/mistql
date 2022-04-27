import builtins from "./builtins";
import { RuntimeError } from "./errors";
import { getProperties, getType } from "./runtimeValues";
import { pushRuntimeValueToStack } from "./stackManip";
import {
  ASTApplicationExpression,
  ASTExpression,
  ASTLiteralExpression,
  ASTReferenceExpression,
  Closure,
  ExecutionFunction,
  RuntimeValue
} from "./types";


export const inputGardenWall = (data: unknown) => {
  if (typeof data === "number" || data instanceof Number) {
    let number = data instanceof Number ? data.valueOf() : data;
    if (Number.isNaN(number) || !Number.isFinite(number)) {
      return null;
    }
    return number;
  }
  if (["boolean", "string"].indexOf(typeof data) > -1) {
    return data;
  }
  if (data instanceof Boolean || data instanceof String) {
    return data.valueOf();
  }
  if (data instanceof Date) {
    return data.toISOString();
  }
  if (data instanceof Symbol) {
    return data.toString();
  }
  if (data === null || data === undefined) {
    return null;
  }
  if (Array.isArray(data)) {
    return data.map((datum) => inputGardenWall(datum));
  }
  const output: { [str: string]: any } = {};
  for (let i in data as any) {
    if (data.hasOwnProperty(i)) {
      output[i] = inputGardenWall(data[i]);
    }
  }
  return output;
};

export const outputGardenWall = (data: unknown) => {
  const outputType = getType(data)
  if (outputType === "function" || outputType === "regex") {
    throw new RuntimeError(
      `Return value of query is "${outputType}", aborting!`
    );
  } else if (outputType === "array") {
    return (data as any[]).map((datum) => outputGardenWall(datum));
  } else if (outputType === "object") {
    const output: { [str: string]: any } = {};
    for (let i in data as any) {
      if (data.hasOwnProperty(i)) {
        output[i] = outputGardenWall(data[i]);
      }
    }
    return output;
  } else {
    return data;
  }
};

export const execute = (node: ASTExpression, variables: unknown) => {
  const data = inputGardenWall(variables);
  const initialStack = [builtins, {
    $: Object.assign({}, builtins, { "@": data }),
  }];

  const result = executeInner(
    node,
    pushRuntimeValueToStack(data, initialStack)
  );
  return outputGardenWall(result);
};

const executeInner: ExecutionFunction = (
  statement: ASTExpression,
  stack: Closure[]
): RuntimeValue => {
  switch (statement.type) {
    case "parenthetical":
      return executeInner(statement.expression, stack);
    case "literal":
      return executeLiteral(statement, stack);
    case "reference":
      return executeReference(statement, stack);
    case "application":
      return executeApplication(statement, stack);
    case "pipeline":
      let last: RuntimeValue = executeInner(statement.stages[0], stack);
      for (let i = 1; i < statement.stages.length; i++) {
        // We know in a non-typesafe manner that all but the first stage are functions
        const stage = statement.stages[i] as ASTApplicationExpression;
        let app: ASTApplicationExpression;
        const atRef: ASTExpression = { type: "reference", ref: "@" };
        app = {
          type: "application",
          function: stage.function,
          arguments: stage.arguments.concat(atRef),
        };
        last = executeApplication(app, pushRuntimeValueToStack(last, stack));
      }
      return last;
  }
};

const executeLiteral = (
  statement: ASTLiteralExpression,
  stack: Closure[]
): RuntimeValue => {
  switch (statement.valueType) {
    case "string":
    case "number":
    case "boolean":
    case "null":
      return statement.value;
    case "array":
      return statement.value.map((exp: ASTExpression) =>
        executeInner(exp, stack)
      );
    case "object": {
      const result = {};
      getProperties(statement.value).forEach((prop) => {
        result[prop] = executeInner(statement.value[prop], stack);
      })
      return result;
    }
  }
};

const executeReference = (
  statement: ASTReferenceExpression,
  inputStack: Closure[]
): RuntimeValue => {
  const stack = statement.internal ? [builtins] : inputStack;
  // first, find the appropriate referenced variable:
  let referencedInStack = undefined;
  for (let i = stack.length - 1; i >= 0; i--) {
    if (stack[i][statement.ref] !== undefined) {
      referencedInStack = stack[i][statement.ref];
      break;
    }
  }
  if (referencedInStack === undefined) {
    throw new RuntimeError("Could not find referenced variable " + statement.ref);
  }
  return referencedInStack;
};

const executeApplication = (
  statement: ASTApplicationExpression,
  stack: Closure[]
): RuntimeValue => {
  const fn = executeInner(statement.function, stack);
  const fnType = getType(fn);
  if (fnType !== "function") {
    throw new RuntimeError(`Attempted to call a variable of type "${fnType}". Only functions are callable`);
  }
  return fn(statement.arguments, stack, executeInner);
};
