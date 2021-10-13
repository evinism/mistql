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
    valueType: "struct";
    value: { [key: string]: ASTExpression };
  }

export type ASTPipelineExpression = {
  type: "pipeline";
  stages: ASTExpression[];
};

export type ASTReferenceExpression = {
  type: "reference";
  ref: string;
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


export type LexToken = {
  token: 'value',
  value: string | number | boolean | null
  position: number,
} | {
  token: 'ref',
  value: string,
  position: number,
} | {
  token: 'special',
  value: string,
  position: number,
}
