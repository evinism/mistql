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
    };

export type ASTPipelineExpression = {
  type: "pipeline";
  stages: ASTExpression[];
};

export type ASTReferenceExpression = {
  type: "reference";
  path: string[];
};

export type ASTApplicationExpression = {
  type: "application";
  function: ASTExpression;
  arguments: ASTExpression[];
};

export type ASTExpression =
  | ASTApplicationExpression
  | ASTReferenceExpression
  | ASTPipelineExpression
  | ASTLiteralExpression;

/* Runtime types */
export type RuntimeValue = any;
export type Closure = {
  [varname: string]: RuntimeValue;
};
export type Stack = Closure[];

export type ExecutionFunction = (
  exp: ASTExpression,
  stack: Stack
) => RuntimeValue;
export type BuiltinFunction = (
  args: ASTExpression[],
  stack: Stack,
  executeInner: ExecutionFunction
) => RuntimeValue;
