import builtins from "./builtins";
import { pushRuntimeValueToStack } from "./stackManip";
import {
  ASTApplicationExpression,
  ASTExpression,
  ASTLiteralExpression,
  ASTReferenceExpression,
  Closure,
  ExecutionFunction,
  RuntimeValue,
  Stack,
} from "./types";

const defaultStack: Stack = [builtins];

export const execute = (node: ASTExpression, variables: Closure) => {
  return executeInner(node, pushRuntimeValueToStack(variables, defaultStack));
};

const executeInner: ExecutionFunction = (
  statement: ASTExpression,
  stack: Closure[]
): RuntimeValue => {
  switch (statement.type) {
    case "literal":
      return executeLiteral(statement, stack);
    case "reference":
      return executeReference(statement, stack);
    case "application":
      return executeApplication(statement, stack);
    case "pipeline":
      let last: RuntimeValue = executeInner(statement.stages[0], stack);
      for (let i = 1; i < statement.stages.length; i++) {
        const stage = statement.stages[i];
        let app: ASTApplicationExpression;
        const atRef: ASTExpression = { type: "reference", ref: "@" };
        if (stage.type === "application") {
          app = {
            type: "application",
            function: stage.function,
            arguments: stage.arguments.concat(atRef),
          };
        } else {
          app = {
            type: "application",
            function: stage,
            arguments: [atRef],
          };
        }
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
  }
};

const executeReference = (
  statement: ASTReferenceExpression,
  stack: Closure[]
): RuntimeValue => {
  // first, find the appropriate referenced variable:
  let referencedInStack = undefined;
  for (let i = stack.length - 1; i >= 0; i--) {
    if (stack[i][statement.ref] !== undefined) {
      referencedInStack = stack[i][statement.ref];
      break;
    }
  }
  if (referencedInStack === undefined) {
    throw new Error("Could not find referenced variable " + statement.ref);
  }
  return referencedInStack;
};

const executeApplication = (
  statement: ASTApplicationExpression,
  stack: Closure[]
): RuntimeValue => {
  const fn = executeInner(statement.function, stack);
  if (typeof fn !== "function") {
    throw new Error("Expected a function, got.. something else!");
  }
  return fn(statement.arguments, stack, executeInner);
};
