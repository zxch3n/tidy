/**
 * Run with
 * node --expose-gc test/benchmark.mjs
 */
import {
  initWasm,
  TidyLayout,
  LayoutType,
  createNode,
} from '../dist/tidy.es.mjs';
import { join, dirname } from 'path';
import { appendFileSync, readFileSync } from 'fs';
import { fileURLToPath } from 'node:url';

function gc() {
  global.gc();
}

const filename = fileURLToPath(import.meta.url);
const __dirname = dirname(filename);
initWasm(readFileSync(join(__dirname, '../wasm_dist/wasm_bg.wasm'))).then(main);
async function bench(type) {
  const [root, arr] = insertNewNodes(0);
  const perf = [];
  console.log('Benching for type:', LayoutType[type]);
  for (let num = 1000; num < 500_000; num += 1000) {
    const tidy = await TidyLayout.create(type);
    insertNewNodes(1000, root, arr);
    tidy.set_root(root);
    gc();
    await new Promise((r) => setTimeout(r, 0));
    const start = performance.now();
    tidy.layout();
    const duration = performance.now() - start;
    perf.push({ duration, num });
    tidy.dispose();
    gc();
    await new Promise((r) => setTimeout(r, 0));
    if (num % 100_000 === 0) {
      console.log(num);
    }
  }

  let out = '';
  for (const line of perf) {
    out += `${line.num} ${line.duration * 1000}\n`;
  }

  if (type === LayoutType.Tidy) {
    appendFileSync(join(__dirname, 'tidy_wasm.log'), out, {});
  }
  if (type === LayoutType.Basic) {
    appendFileSync(join(__dirname, 'naive_wasm.log'), out);
  }
}

function insertNewNodes(num, root = createNode(), arr = [root]) {
  for (let i = 0; i < num; i++) {
    let parentIndex = 0;
    parentIndex = (arr.length * Math.random()) | 0;
    const parent = arr[parentIndex];
    const child = createNode();
    parent.children.push(child);
    child.parentId = parent.id;
    arr.push(child);
  }

  return [root, arr];
}

async function main() {
  for (let i = 0; i < 100; i++) {
    console.log(i);
    await bench(LayoutType.Tidy);
    gc();
    await new Promise((r) => setTimeout(r, 1000));
    await bench(LayoutType.Basic);
    gc();
    await new Promise((r) => setTimeout(r, 1000));
  }
}
