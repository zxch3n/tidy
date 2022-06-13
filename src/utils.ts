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
  const MAX_CHOSEN_SIZE = 16;
  for (let i = 0; i < num; i++) {
    let parentIndex = 0;
    if (arr.length < MAX_CHOSEN_SIZE) {
      parentIndex = (arr.length * Math.random()) | 0;
    } else {
      parentIndex = (arr.length - 1 - MAX_CHOSEN_SIZE * Math.random()) | 0;
    }

    const parent = arr[parentIndex];
    const child = createNode();
    parent.children.push(child);
    child.parentId = parent.id;
    arr.push(child);
  }

  return root;
}
