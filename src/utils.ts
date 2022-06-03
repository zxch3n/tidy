export function visit<T extends { children: T[] }>(
  node: T,
  func: (node: T) => void,
) {
  func(node);
  for (const child of node.children) {
    visit(child, func);
  }
}
