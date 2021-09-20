import { compare, getType, RuntimeValueType, truthy } from "./runtimeValues";
import { pushRuntimeValueToStack } from "./stackManip";
import { BuiltinFunction, RuntimeValue } from "./types";
import { seqHelper } from "./util";

const arity =
  (arityCount: number, fn: BuiltinFunction): BuiltinFunction =>
  (args, stack, exec) => {
    if (args.length !== arityCount) {
      throw new Error(
        "Expected " + arityCount + " arguments, got " + args.length
      );
    }
    return fn(args, stack, exec);
  };

const validateType = (
  type: RuntimeValueType,
  value: RuntimeValue
): RuntimeValue => {
  if (getType(value) !== type) {
    throw new Error("Expected type " + type + ", got " + getType(value));
  }
  return value;
};

const map: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const targetExp = args[1];
  const target = exec(targetExp, stack);
  if (!Array.isArray(target)) {
    throw new Error("Expected array");
  }
  const newValue = target.map((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    return exec(fnExp, newStack);
  });
  return newValue;
});

// TODO: Make this work with non-string values
const groupby: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const targetExp = args[1];
  const target = validateType("array", exec(targetExp, stack));
  const groupings: { [key: string]: RuntimeValue } = {};
  target.forEach((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    const group = validateType("string", exec(fnExp, newStack));
    groupings[group] = (groupings[group] || []).concat([innerValue]);
  });
  return groupings;
});

const filter: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const targetExp = args[1];
  const target = validateType("array", exec(targetExp, stack));
  const newValue = target.filter((innerValue) => {
    const newStack = pushRuntimeValueToStack(innerValue, stack);
    return truthy(exec(fnExp, newStack));
  });
  return newValue;
});

const reduce: BuiltinFunction = arity(3, (args, stack, exec) => {
  const fnExp = args[0];
  const startExp = args[1];
  const targetExp = args[2];
  const target = validateType("array", exec(targetExp, stack));
  const newValue = target.reduce((acc: RuntimeValue, cur: RuntimeValue) => {
    const accCurPair: RuntimeValue = [acc, cur];
    const newStack = pushRuntimeValueToStack(accCurPair, stack);
    return exec(fnExp, newStack);
  }, exec(startExp, stack));
  return newValue;
});

const find: BuiltinFunction = (args, stack, exec) => {
  return filter(args, stack, exec)[0] ?? null;
};

const unaryMinus: BuiltinFunction = arity(1, (args, stack, exec) => {
  return -validateType("number", exec(args[0], stack));
});

const notOp: BuiltinFunction = arity(1, (args, stack, exec) => {
  return !truthy(validateType("boolean", exec(args[0], stack)));
});

const plus: BuiltinFunction = arity(2, (args, stack, exec) => {
  const a = exec(args[0], stack);
  const b = exec(args[1], stack);
  const type = getType(a);
  if (type !== getType(b)) {
    throw new Error("Cannot add values of different types");
  }
  if (type !== 'string' && type !== 'number') {
    throw new Error("Cannot add values of type " + type)
  }
  return a + b;
});

const numericBinaryOperator = (
  op: (a: number, b: number) => number
): BuiltinFunction =>
  arity(2, (args, stack, exec) => {
    const a = validateType("number", exec(args[0], stack));
    const b = validateType("number", exec(args[1], stack));
    return op(a, b);
  });

const booleanBinaryOperator = (
  op: (a: boolean, b: boolean) => boolean
): BuiltinFunction =>
  arity(2, (args, stack, exec) => {
    const a = validateType("boolean", exec(args[0], stack));
    const b = validateType("boolean", exec(args[1], stack));
    return op(a, b);
  });

const equal: BuiltinFunction = arity(2, (args, stack, exec) => {
  const a = exec(args[0], stack);
  const b = exec(args[1], stack);
  if (getType(a) !== getType(b)) {
    return false;
  }
  // TODO: Make equality work for arrays.
  return a === b;
});

const notequal: BuiltinFunction = (args, stack, exec) => {
  return !equal(args, stack, exec);
};

const count: BuiltinFunction = arity(1, (args, stack, exec) => {
  const result = validateType("array", exec(args[0], stack));
  return result.length;
});

const keys: BuiltinFunction = arity(1, (args, stack, exec) => {
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(i);
    }
  }
  return results;
});

const values: BuiltinFunction = arity(1, (args, stack, exec) => {
  const evaluated = exec(args[0], stack);
  const results = [];
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results.push(evaluated[i]);
    }
  }
  return results;
});

const mapvalues: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      results[i] = exec(fnExp, pushRuntimeValueToStack(evaluated[i], stack));
    }
  }
  return results;
});

const filtervalues: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (
      evaluated.hasOwnProperty(i) &&
      truthy(exec(fnExp, pushRuntimeValueToStack(evaluated[i], stack)))
    ) {
      results[i] = evaluated[i];
    }
  }
  return results;
});

const mapkeys: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (evaluated.hasOwnProperty(i)) {
      const newKey = exec(fnExp, pushRuntimeValueToStack(i, stack));
      results[newKey] = evaluated[i];
    }
  }
  return results;
});

const filterkeys: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const evaluated = exec(args[1], stack);
  const results = {};
  for (let i in evaluated) {
    if (
      evaluated.hasOwnProperty(i) &&
      truthy(exec(fnExp, pushRuntimeValueToStack(i, stack)))
    ) {
      results[i] = evaluated[i];
    }
  }
  return results;
});

const index: BuiltinFunction = arity(2, (args, stack, exec) => {
  const idx = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr[idx] ?? null;
});

const head: BuiltinFunction = arity(2, (args, stack, exec) => {
  const count = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr.slice(0, count);
});

const tail: BuiltinFunction = arity(2, (args, stack, exec) => {
  const count = validateType("number", exec(args[0], stack));
  const arr = validateType("array", exec(args[1], stack));
  return arr.slice(arr.length - count, arr.length);
});

const first: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arr = validateType("array", exec(args[0], stack));
  return arr[0] ?? null;
});

const last: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arr = exec(args[0], stack);
  if (!Array.isArray(arr)) {
    throw new Error("Expected array");
  }
  return arr[arr.length - 1] ?? null;
});

const binaryCompareFunction = (
  truthtable: [boolean, boolean, boolean]
): BuiltinFunction =>
  arity(2, (args, stack, exec) => {
    const a = exec(args[0], stack);
    const b = exec(args[1], stack);
    const comparison = compare(a, b);
    if (comparison < 0) {
      return truthtable[0];
    }
    if (comparison === 0) {
      return truthtable[1];
    }
    if (comparison > 0) {
      return truthtable[2];
    }
  });

const sort: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg = validateType("array", exec(args[0], stack));
  // default to ascending
  return arg.slice().sort((b, a) => compare(a, b));
});

const sortby: BuiltinFunction = arity(2, (args, stack, exec) => {
  const fnExp = args[0];
  const arg = validateType("array", exec(args[1], stack));
  return arg.slice().map((item) => {
    const sortValue = exec(fnExp, pushRuntimeValueToStack(item, stack))
    return ({sortValue, item})
  }).sort(({sortValue: a}, {sortValue: b}) => compare(b, a)).map(({item}) => item) ;
});

const reverse: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg = validateType("array", exec(args[0], stack));
  return arg.reverse();
});

const dotAccessor: BuiltinFunction = arity(2, (args, stack, exec) => {
  const former = exec(args[0], stack);
  if (args[1].type !== "reference") {
    throw new Error("Only references are allowed as rhs to dot access");
  }
  const latter = exec(args[1], pushRuntimeValueToStack(former, []));
  return latter;
});

const sum: BuiltinFunction = arity(1, (args, stack, exec) => {
  const arg: RuntimeValue = validateType("array", exec(args[0], stack));
  return arg.reduce((a, b) => a + b, 0);
});

const ifFunction: BuiltinFunction = arity(3, (args, stack, exec) => {
  return truthy(exec(args[0], stack))
    ? exec(args[1], stack)
    : exec(args[2], stack);
});

const log: BuiltinFunction = arity(1, (args, stack, exec) => {
  const res = exec(args[0], stack);
  console.log("MistQL: " + JSON.stringify(res, null, 2));
  return res;
});

// For Summary
const _getMean = (array: number[]) => {
  return array.reduce((a, b) => a + b, 0) / array.length;
}

const _getMedian = (array: number[]) => {
  const midpoint = array.length / 2;
  const median = midpoint % 1 ? array[midpoint - 0.5] : (array[midpoint - 1] + array[midpoint]) / 2;
  return median;
}

const _getVariance = (array: number[]) => {
  var mean = _getMean(array);
  return _getMean(array.map(function(num) {
    return Math.pow(num - mean, 2);
  }));
};

const _getStandardDeviation = (array: number[]) => Math.sqrt(_getVariance(array));

const summarize: BuiltinFunction = arity(1, (args, stack, exec) => {
  const array = validateType("array", exec(args[0], stack));
  array.sort((a, b) => a - b);
  array.forEach(value => validateType("number", value));

  return {
    max: Math.max.apply(null, array),
    min: Math.min.apply(null, array),
    mean: _getMean(array),
    median: _getMedian(array),
    variance: _getVariance(array),
    stddev: _getStandardDeviation(array),
  };
});

const sequence: BuiltinFunction = (args, stack, exec) => {
  if (args.length < 3 ){
    throw new Error("Expected at least 3 arguments, got " + args.length);
  }
  const target: RuntimeValue[] = validateType("array", exec(args[args.length - 1], stack));
  const fns = args.slice(0, args.length - 1);
  const booleanMap = fns.map((fn) => 
    target.map(value => truthy(exec(fn, pushRuntimeValueToStack(value, stack))))
  );
  const seq = seqHelper(booleanMap);
  return seq.map((inner) => inner.map(idx => target[idx]));
}

export default {
  count,
  filter,
  filterkeys,
  filtervalues,
  find,
  first,
  groupby,
  head,
  if: ifFunction,
  index,
  keys,
  last,
  log,
  map,
  mapkeys,
  mapvalues,
  reduce,
  reverse,
  sequence,
  sort,
  sortby,
  sum,
  summarize,
  tail,
  values,
  "!/unary": notOp,
  "-/unary": unaryMinus,
  ".": dotAccessor,
  "+": plus,
  "-": numericBinaryOperator((a, b) => a - b),
  "*": numericBinaryOperator((a, b) => a * b),
  "/": numericBinaryOperator((a, b) => a / b),
  "%": numericBinaryOperator((a, b) => a % b),
  "||": booleanBinaryOperator((a, b) => a || b),
  "&&": booleanBinaryOperator((a, b) => a && b),
  "==": equal,
  "!=": notequal,
  ">": binaryCompareFunction([true, false, false]),
  "<": binaryCompareFunction([false, false, true]),
  ">=": binaryCompareFunction([true, true, false]),
  "<=": binaryCompareFunction([false, true, true]),
};
