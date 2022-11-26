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
  const arr = [root];
  for (let i = 0; i < num; i++) {
    let parentIndex = 0;
    parentIndex = (arr.length * Math.random()) | 0;
    const parent = arr[parentIndex];
    const child = createNode();
    parent.children.push(child);
    child.parentId = parent.id;
    arr.push(child);
  }

  return root;
}

export function deleteRandomNode(root: Node, num: number) {
  while (num > 0 && root.children.length > 0) {
    let candidates: {
      node: Node;
      parent: Node;
      depth: number;
    }[] = [];
    visit(root, (node, depth) => {
      if (num === 0) {
        return;
      }

      for (let i = 0; i < node.children.length; i++) {
        candidates.push({
          node: node.children[i],
          parent: node,
          depth: depth + 1,
        });
      }
    });

    candidates.sort((a, b) => a.depth - b.depth);
    candidates = candidates.slice(-num);
    for (const { parent, node } of candidates) {
      parent.children.splice(parent.children.indexOf(node), 1);
      num -= nodeNum(node);
      if (num <= 0) {
        break;
      }
    }
  }
}

export function insertRandomNodeDepthFirst(root: Node, num: number = 1) {
  let nodes: [Node, number][] = [];
  visit(root, (node, depth) => {
    nodes.push([node, depth]);
  });

  function filter() {
    nodes.sort((a, b) => Math.random() * 2 - 1);
    nodes.sort((a, b) => -a[1] + b[1]);
    nodes = nodes.filter(([node, d]) => node.children.length < 4);
    nodes = nodes.slice(0, 20).concat(nodes.filter(([node, d]) => d < 2));
  }

  filter();
  for (let i = 0; i < num; i++) {
    const [node, d] = nodes[(Math.random() * nodes.length) | 0];
    const child = createNode();
    child.parentId = node.id;
    node.children.push(child);
    nodes.push([child, d + 1]);
    if (nodes.length % 40 === 0) {
      filter();
    }
  }
}

export function insertRandomNodeBreadthFirst(root: Node, num: number = 1) {
  let nodes: [Node, number][] = [];
  visit(root, (node, depth) => {
    nodes.push([node, depth]);
  });

  for (let i = 0; i < num; i++) {
    const [node, d] = nodes[(Math.random() * nodes.length) | 0];
    const child = createNode();
    child.parentId = node.id;
    node.children.push(child);
    nodes.push([child, d + 1]);
  }
}

export function node(
  width: number,
  height: number,
  children: Node[] = [],
): Node {
  return {
    x: 0,
    y: 0,
    width,
    height,
    children,
  };
}

export function nodeNum(root: Node) {
  let count = 0;
  visit(root, () => count++);
  return count;
}
