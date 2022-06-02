import init, { sum_of_squares, LinkedList } from '../wasm_dist/wasm';

export { init, LinkedList };
export function sumOfSquares(...arr: number[]) {
  return sum_of_squares(new Int32Array(arr));
}
