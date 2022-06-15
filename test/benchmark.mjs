/**
 * Run with
 * node --expose-gc test/benchmark.mjs
 */
import {
  initWasm,
  TidyLayout,
  LayoutType,
  createTree,
} from '../dist/tidy.es.mjs';
import { join, dirname } from 'path';
import { writeFileSync, readFileSync } from 'fs';
import { fileURLToPath } from 'node:url';

function gc() {
  global.gc();
}

const filename = fileURLToPath(import.meta.url);
const __dirname = dirname(filename);
initWasm(readFileSync(join(__dirname, '../wasm_dist/wasm_bg.wasm'))).then(main);
async function bench(type) {
  const tidy = await TidyLayout.create(type);
  const root = createTree(1_000);
  tidy.set_root(root);

  const perf = [];
  for (let i = 0; i < 10; i++) {
    tidy.layout();
  }

  tidy.dispose();
  console.log('Benching for type:', LayoutType[type]);
  for (let num = 1000; num < 110_000; num += 1000) {
    const tidy = await TidyLayout.create(type);
    const root = createTree(num);
    tidy.set_root(root);
    gc();
    await new Promise((r) => setTimeout(r, 100));
    const start = performance.now();
    tidy.layout();
    const duration = performance.now() - start;
    perf.push({ duration, num });
    tidy.dispose();
    gc();
    await new Promise((r) => setTimeout(r, 0));
  }

  let out = '';
  for (const line of perf) {
    out += `${line.num} ${line.duration * 1000}\n`;
  }

  if (type === LayoutType.Tidy) {
    writeFileSync(join(__dirname, 'tidy_wasm.log'), out);
  }
  if (type === LayoutType.Basic) {
    writeFileSync(join(__dirname, 'naive_wasm.log'), out);
  }
}

async function main() {
  await bench(LayoutType.Tidy);
  gc();
  await new Promise((r) => setTimeout(r, 100));
  await bench(LayoutType.Basic);
}
