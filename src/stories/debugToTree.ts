import { InnerNode } from '../tidy';
export function debugStrToTree(input: string): InnerNode {
  const lines = input.split('\n');
  return linesToNode(lines.map((line) => ({ line, indent: getIndent(line) })));
}

function linesToNode(lines: { line: string; indent: number }[]): InnerNode {
  const currentIndent = lines[0].indent;
  const node = lineToNode(lines[0].line);
  for (let i = 1; i < lines.length; i++) {
    const { line, indent } = lines[i];
    if (indent === currentIndent + 1) {
      const start = i;
      let childIndent = lines[i + 1]?.indent;
      while (childIndent != null && childIndent >= currentIndent + 2) {
        i += 1;
        childIndent = lines[i + 1]?.indent;
      }

      const child = linesToNode(lines.slice(start, i + 1));
      node.children.push(child);
    } else {
      throw new Error();
    }
  }

  return node;
}

function lineToNode(line: string): InnerNode {
  const ans = line.match(
    /x: ([-\d\.]+), y: ([-\d\.]+), width: ([\d\.]+), height: ([\d\.]+)/,
  )!;
  return {
    id: (Math.random() * 1e9) | 0,
    x: parseFloat(ans[1]),
    y: parseFloat(ans[2]),
    width: parseFloat(ans[3]),
    height: parseFloat(ans[4]),
    children: [],
  };
}

function getIndent(line: string) {
  return (line.match(/^ */s)![0].length / 4) | 0;
}
