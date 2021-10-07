import { BuiltinFunction } from "../types";
import { arity, validateType } from "../util";


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
  return _getMean(array.map(function (num) {
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

export default summarize;