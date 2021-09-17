import commonjs from '@rollup/plugin-commonjs';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import analyze from 'rollup-plugin-analyzer';
import { terser } from "rollup-plugin-terser";

export default {
  input: 'dist/index.js',
  output: {
    file: 'dist/bundle.js',
    format: 'umd',
    name: "millieql"
  },
  plugins: [nodeResolve(), commonjs(), terser(), analyze()]
};