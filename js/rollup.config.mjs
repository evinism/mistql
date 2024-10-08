import commonjs from "@rollup/plugin-commonjs";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import analyze from "rollup-plugin-analyzer";
import terser from "@rollup/plugin-terser";

export default {
  input: "dist/esm/nodefault.js",
  output: {
    file: "dist/umd/index.js",
    format: "umd",
    name: "mistql",
  },
  plugins: [nodeResolve(), commonjs(), terser(), analyze()],
};
