// Parser Types
export type ASTLiteralExpression =
  | {
    type: "literal";
    valueType: "string";
    value: string;
  }
  | {
    type: "literal";
    valueType: "number";
    value: number;
  }
  | {
    type: "literal";
    valueType: "boolean";
    value: boolean;
  }
  | {
    type: "literal";
    valueType: "array";
    value: Array<ASTExpression>;
  }
  | {
    type: "literal";
    valueType: "null";
    value: null;
  }
  | {
    type: "literal";
    valueType: "object";
    value: { [key: string]: ASTExpression };
  };

export type ASTPipelineExpression = {
  type: "pipeline";
  stages: ASTExpression[];
};

export type ASTReferenceExpression = {
  type: "reference";
  ref: string;
  internal?: true;
};

export type ASTApplicationExpression = {
  type: "application";
  function: ASTExpression;
  arguments: ASTExpression[];

  // The below is a nasty hack, but it's useful for distinguishing for
  // whether we need to wrap the function in piped expressions
  _shouldntWrapInPipedExpressions?: boolean;
};

export type ASTParentheticalExpression = {
  type: "parenthetical";
  expression: ASTExpression;
};

export type ASTExpression =
  | ASTApplicationExpression
  | ASTReferenceExpression
  | ASTPipelineExpression
  | ASTLiteralExpression
  | ASTParentheticalExpression;

/* Runtime types */
export type RuntimeValue =
  | RuntimeValue[]
  | { [key: string]: RuntimeValue }
  | FunctionValue
  | RegExp
  | number
  | boolean
  | string
  | null
  | any; // TODO: Remove this one.

export type RuntimeValueType =
  | "array"
  | "object"
  | "regex"
  | "number"
  | "boolean"
  | "string"
  | "function"
  | "null";

export type Closure = {
  [varname: string]: RuntimeValue;
};

export type Stack = Closure[];

export type FunctionClosure = {
  [varname: string]: FunctionValue;
};

export type ExecutionFunction = (
  exp: ASTExpression,
  stack: Stack
) => RuntimeValue;

export type FunctionValue = (
  args: ASTExpression[],
  stack: Stack,
  executeInner: ExecutionFunction
) => RuntimeValue;

export type BuiltinFunction = FunctionValue;

export type LexToken =
  | {
    token: "value";
    value: string | number | boolean | null;
    position: number;
  }
  | {
    token: "ref";
    value: string;
    position: number;
  }
  | {
    token: "special";
    value: string;
    position: number;
  };
