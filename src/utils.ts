import { Node } from './tidy';

export function visit<T extends { children: T[] }>(
  node: T,
  func: (node: T) => void,
) {
  func(node);
  for (const child of node.children) {
    visit(child, func);
  }
}

function createNode(): Node {
  return {
    id: (Math.random() * 1e9) | 0,
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
