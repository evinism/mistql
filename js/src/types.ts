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
export type ExecutionFunction = (
  exp: ASTExpression,
  stack: Stack
) => RuntimeValue;

export type RuntimeArrayValue = Array<RuntimeValue>;
export type RuntimeObjectValue = {
  [key: string]: RuntimeValue;
};
export type RuntimeFunctionValue = (
  args: ASTExpression[],
  stack: Stack,
  executeInner: ExecutionFunction
) => RuntimeValue;

export type RuntimeValue =
  | null
  | boolean
  | number
  | string
  | RuntimeArrayValue
  | RuntimeObjectValue
  | RuntimeFunctionValue;

export type Closure = {
  [varname: string]: RuntimeValue;
};
export type Stack = Closure[];
export type BuiltinFunction = RuntimeFunctionValue;


export type LexToken = {
  token: 'value',
  value: string | number | boolean | null
} | {
  token: 'ref',
  value: string,
} | {
  token: 'special',
  value: string
}
