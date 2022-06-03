import { TidyLayout, Node, initWasm } from '../src/tidy';
import { describe, expect, it } from 'vitest';
import { createTree, visit } from '../src/utils';
import { readFile } from 'fs/promises';
import * as path from 'path';

describe('tidy', () => {
  it('test', async () => {
    const wasm = await readFile(
      path.join(__dirname, '../wasm_dist/wasm_bg.wasm'),
    );
    await initWasm(wasm);
    const tidy = await TidyLayout.create();
    const root = createTree(1000);
    tidy.set_root(root);
    const start = performance.now();
    tidy.layout();
    console.log(performance.now() - start);
    const nodes: Node[] = [];
    visit(root, (node) => {
      for (const other of nodes) {
        expect(
          node.x > other.x + other.width ||
            node.x + node.width < other.x ||
            node.y > other.y + other.height ||
            node.y + node.height < other.y,
        ).toBeTruthy();
      }

      nodes.push(node);
    });
  });
});
