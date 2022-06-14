import { TidyLayout, initWasm, LayoutType } from '../src/tidy';
import { beforeAll, expect, describe, it } from 'vitest';
import { createTree, insertRandomNodeBreadthFirst } from '../src/utils';
import { readFile } from 'fs/promises';
import { debugStrToTree } from '../src/stories/debugToTree';
import * as path from 'path';

describe('tidy', () => {
  beforeAll(async () => {
    const wasm = await readFile(
      path.join(__dirname, '../wasm_dist/wasm_bg.wasm'),
    );
    await initWasm(wasm);
  });

  it('order', async () => {
    const tidy = await TidyLayout.create(LayoutType.Tidy);
    const root = createTree(100);
    tidy.set_root(root);
    tidy.layout();
    for (let i = 1; i < root.children.length; i++) {
      expect(root.children[i].x - root.children[i].width / 2).toBeGreaterThan(
        root.children[i - 1].x - root.children[i - 1].width / 2,
      );
    }
  });

  /**
   * it takes 20ms to layout 100k nodes
   */
  it('benchmark tidy', async () => {
    const tidy = await TidyLayout.create(LayoutType.Tidy);
    const root = createTree(1_000);
    tidy.set_root(root);

    const perf: { duration: number; num: number }[] = [];
    for (let i = 0; i < 10; i++) {
      tidy.layout();
    }

    for (let num = 1000; num < 110_000; num += 1000) {
      tidy.update();
      const start = performance.now();
      tidy.layout();
      const duration = performance.now() - start;
      perf.push({ duration, num });
      insertRandomNodeBreadthFirst(root, 1000);
    }

    console.log(JSON.stringify(perf, undefined, 2));
  });

  it('benchmark naive', async () => {
    const tidy = await TidyLayout.create(LayoutType.Basic);
    const root = createTree(1_000);
    tidy.set_root(root);

    const perf: { duration: number; num: number }[] = [];
    for (let i = 0; i < 10; i++) {
      tidy.layout();
    }

    for (let num = 1000; num < 110_000; num += 1000) {
      tidy.update();
      const start = performance.now();
      tidy.layout();
      const duration = performance.now() - start;
      perf.push({ duration, num });
      insertRandomNodeBreadthFirst(root, 1000);
    }

    console.log(JSON.stringify(perf, undefined, 2));
  });

  it('debug', () => {
    const tree =
      debugStrToTree(`x: 0, y: 0, width: 2, height: 5, rx: 11.158500000000002, mod: -11.158500000000002
    x: -11.1585, y: 5.001, width: 1, height: 5, rx: 13.315000000000001, mod: -13.315000000000001
        x: -24.4735, y: 10.002, width: 9, height: 2, rx: 6.376, mod: -6.376
            x: -30.849500000000003, y: 12.003, width: 7, height: 1, rx: 0, mod: 0
                x: -30.849500000000003, y: 13.004, width: 9, height: 6, rx: 0, mod: 0
            x: -23.4735, y: 12.003, width: 1, height: 6, rx: 0, mod: 7.376
            x: -18.0975, y: 12.003, width: 5, height: 7, rx: 5.751, mod: 7.001
                x: -23.8485, y: 19.004, width: 5, height: 7, rx: 0, mod: 0
                x: -17.3475, y: 19.004, width: 8, height: 9, rx: 0, mod: 6.501
                x: -12.3465, y: 19.004, width: 2, height: 7, rx: 0, mod: 11.502
        x: -7.345500000000001, y: 10.002, width: 5, height: 7, rx: 0, mod: 17.128
            x: -7.345500000000001, y: 17.003000000000004, width: 8, height: 6, rx: 0, mod: 0
        x: -1.3445, y: 10.002, width: 2, height: 7, rx: 0, mod: 23.129
            x: -1.3445, y: 17.003000000000004, width: 4, height: 9, rx: 0, mod: 0
        x: 2.156500000000001, y: 10.002, width: 3, height: 9, rx: 0, mod: 26.630000000000003
    x: 6.157500000000001, y: 5.001, width: 5, height: 6, rx: 0, mod: 17.316000000000003
    x: 11.158500000000002, y: 5.001, width: 5, height: 7, rx: 0, mod: 22.317000000000004`);
  });
});
