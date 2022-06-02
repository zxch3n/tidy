import { describe, it, expect, beforeAll } from 'vitest';
import { sumOfSquares, init, LinkedList } from '../src/wasmEntry';
import { readFile } from 'fs/promises';
import * as path from 'path';

beforeAll(async () => {
  await init(
    await readFile(path.resolve(__dirname, '../wasm_dist/wasm_bg.wasm')),
  );
});

describe('add', () => {
  it('adds two numbers', async () => {
    expect(sumOfSquares(3, 4)).toBe(25);
  });
});

class Node {
  value: number;
  next?: Node;
  constructor(value: number, next?: Node) {
    this.value = value;
    this.next = next;
  }
}

class StupidList {
  head?: Node;
  push(v: number) {
    if (this.head == null) {
      this.head = new Node(v);
    } else {
      let node = this.head;
      while (node.next) {
        node = node.next;
      }

      node.next = new Node(v);
    }
  }

  pop() {
    const node = this.head;
    this.head = this.head?.next;
    return node.value;
  }
}

describe('list', () => {
  it('wasm list', () => {
    const list = new LinkedList();
    const start = performance.now();
    for (let i = 0; i < 1e6; i++) {
      list.push(i);
      list.pop();
    }
    console.log('wasm list ', performance.now() - start);
  });

  it('local list', () => {
    const list = new StupidList();
    const start = performance.now();
    for (let i = 0; i < 1e6; i++) {
      list.push(i);
      list.pop();
    }
    console.log('local list ', performance.now() - start);
  });
});
