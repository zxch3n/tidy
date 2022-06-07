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
    height: (10 * Math.random() + 10) | 0,
    width: (10 * Math.random() + 10) | 0,
    x: 0,
    y: 0,
    children: [],
  };
}

export function createTree(num: number): Node {
  const root = createNode();
  let arr = [root];
  for (let i = 0; i < num; i += 10) {
    const parentIndex = (arr.length * Math.random()) | 0;
    for (let j = 0; j < 10; j++) {
      const child = createNode();
      const parent = arr[parentIndex];
      parent.children.push(child);
      child.parentId = parent.id;
      arr.push(child);
    }
    if (arr.length > 200) {
      arr = arr.slice(100);
    }
  }

  return root;
}
