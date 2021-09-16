import builtins from "./builtins";
import { ASTApplicationExpression, ASTExpression, ASTLiteralExpression, ASTReferenceExpression, Closure, ExecutionFunction, RuntimeValue, Stack } from "./types";

const defaultStack: Stack = [builtins];

export const execute = (node: ASTExpression, variables: Closure) => {
  return executeInner(node, defaultStack.concat(variables));
}

const executeInner: ExecutionFunction = (statement: ASTExpression, stack: Closure[]): RuntimeValue => {
  switch (statement.type) {
    case 'literal':
      return executeLiteral(statement, stack);
    case 'reference':
      return executeReference(statement, stack);
    case 'application':
      return executeApplication(statement, stack);
  }
}

const executeLiteral = (statement: ASTLiteralExpression, stack: Closure[]): RuntimeValue => {
  switch (statement.valueType) {
    case 'string':
    case 'number':
    case 'boolean':
    case 'null':
      return statement.value
    case 'array':
      return statement.value.map((exp: ASTExpression) => executeInner(exp, stack))
  }
}

const executeReference = (statement: ASTReferenceExpression, stack: Closure[]): RuntimeValue => {
  // first, find the appropriate referenced variable:
  let referencedInStack = undefined;
  for (let i = stack.length - 1; i >= 0; i--) {
    if (stack[i][statement.path[0]]) {
      referencedInStack = stack[i][statement.path[0]];
      break;
    }
  }
  if (!referencedInStack) {
    throw new Error('Could not find referenced variable ' + statement.path[0]);
  }
  let tail = statement.path.slice(1);
  let retval = referencedInStack;
  while (tail.length > 0) {
    const nextProperty = tail.shift();
    if (retval.hasOwnProperty(nextProperty)) {
      retval = retval[nextProperty];
    }
  }
  return retval;
}

const executeApplication = (statement: ASTApplicationExpression, stack: Closure[]): RuntimeValue => {
  const fn = executeInner(statement.function, stack);
  if (typeof fn !== 'function') {
    throw new Error('Expected a function, got.. something else!');
  }
  return fn(statement.arguments, stack, executeInner);
}