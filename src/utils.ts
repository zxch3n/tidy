import { Node } from './tidy';

export function visit<T extends { children: T[] }>(
  node: T,
  func: (node: T, depth: number) => void,
  depth = 0,
) {
  func(node, depth);
  for (const child of node.children) {
    visit(child, func, depth + 1);
  }
}

function randomId(): number {
  return (Math.random() * 2147483647) | 0;
}

export function createNode(): Node {
  return {
    id: randomId(),
    height: 10 * Math.random() + 10,
    width: 10 * Math.random() + 10,
    x: 0,
    y: 0,
    children: [],
  };
}

export function createTree(num: number): Node {
  const root = createNode();
  const arr = [root];
  for (let i = 0; i < num; i++) {
    const child = createNode();
    const parentIndex = (arr.length * Math.random()) | 0;
    const parent = arr[parentIndex];
    parent.children.push(child);
    child.parentId = parent.id;
    arr.push(child);
    if (arr.length > 10) {
      arr.shift();
    }
  }

  return root;
}
