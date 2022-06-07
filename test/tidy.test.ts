import { TidyLayout, initWasm, LayoutType } from '../src/tidy';
import { describe, it } from 'vitest';
import { createTree } from '../src/utils';
import { readFile } from 'fs/promises';
import * as path from 'path';

describe('tidy', () => {
  it('benchmark', async () => {
    const wasm = await readFile(
      path.join(__dirname, '../wasm_dist/wasm_bg.wasm'),
    );
    await initWasm(wasm);
    const tidy = await TidyLayout.create(LayoutType.Tidy);
    const root = createTree(100000);
    tidy.set_root(root);
    const start = performance.now();
    for (let i = 0; i < 20; i++) {
      tidy.layout();
    }
    console.log((performance.now() - start) / 20);
    // const nodes: Node[] = [];
    // visit(root, (node) => {
    //   for (const other of nodes) {
    //     expect(
    //       node.x > other.x + other.width ||
    //         node.x + node.width < other.x ||
    //         node.y > other.y + other.height ||
    //         node.y + node.height < other.y,
    //     ).toBeTruthy();
    //   }

    //   nodes.push(node);
    // });
  });
});
