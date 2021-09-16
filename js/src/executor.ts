import { ASTExpression, ASTLiteralExpression, ASTReferenceExpression } from "./types"

type RuntimeValue = any;

type Closure = {
  [varname: string]: RuntimeValue,
}

const defaultStack: Closure[] = [{}];

export const execute = (node: ASTExpression, variables: Closure) => {
  executeInner(node, defaultStack.concat(variables));
}

const executeInner = (statement: ASTExpression, stack: Closure[]): RuntimeValue=> {
  switch (statement.type) {
    case 'literal':
      return executeLiteral(statement, stack);
    case 'reference':
      return executeReference(statement, stack);
      break;
    case 'pipeline': {
      let pipeStack = stack;
      for (let i = 0; i < statement.stages.length; i++) {
        const retval = executeInner(statement.stages[i], pipeStack);
      }
      break;
    }

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
}
