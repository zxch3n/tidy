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
  let width = 1;
  let height = 1;

  if (Math.random() < 0.1) {
    width = (50 * Math.random() + 50) | 0;
  } else {
    width = (10 * Math.random() + 10) | 0;
  }

  if (Math.random() < 0.1) {
    height = (50 * Math.random() + 50) | 0;
  } else {
    height = (10 * Math.random() + 10) | 0;
  }

  if (Math.random() < 0.03) {
    width = height = (50 * Math.random() + 50) | 0;
  }

  return {
    id: randomId(),
    height,
    width,
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
