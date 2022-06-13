import React, {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from 'react';
import { useDebounce } from 'react-use';
import { Node } from '../tidy';
import { LayoutTypeStr, TidyComponent } from '../TidyComponent';
import { createNode, createTree, visit } from '../utils';

export default {
  title: 'Tidy',
  component: TidyComponent,
  argTypes: {
    layoutType: {
      options: [
        LayoutTypeStr.Tidy,
        LayoutTypeStr.Basic,
        LayoutTypeStr.LayeredTidy,
      ],
      defaultValue: LayoutTypeStr.Tidy,
    },
  },
};

interface Props {
  layoutType: LayoutTypeStr;
}
/**
 * Primary UI component for user interaction
 */
export const TidyLayout = ({
  layoutType,
  num,
  ...props
}: Props & { num: number }) => {
  const [updateTrigger, setUpdate] = useState(0);
  const [root, setRoot] = useState(() => {
    return createTree(1);
  });
  const prevNum = useRef(1);
  useDebounce(
    () => {
      let currentNum = nodeNum(root);
      if (num < currentNum) {
        deleteRandomNode(root, currentNum - num);
      } else if (num > currentNum) {
        insertRandomNodeDepthFirst(root, num - currentNum);
      }

      setUpdate((updateTrigger) => updateTrigger + 1);
      prevNum.current = num;
    },
    100,
    [num],
  );
  const addNode = useCallback(() => {
    insertRandomNodeDepthFirst(root, 1);
    setUpdate((updateTrigger) => updateTrigger + 1);
  }, [root]);

  return (
    <div onClick={addNode}>
      <TidyComponent
        root={root}
        updateTrigger={updateTrigger}
        layoutType={layoutType}
      />
    </div>
  );
};

TidyLayout.argTypes = {
  num: {
    control: { type: 'range', min: 0, max: 400 },
    defaultValue: 200,
  },
};

export const Example0 = () => {
  return (
    <TidyComponent
      root={node(10, 10, [
        node(10, 10, [
          node(10, 10, [
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
            node(10, 10),
          ]),
        ]),
        node(10, 10, [node(10, 10), node(10, 10), node(10, 10), node(10, 10)]),
        node(10, 10),
        node(10, 40),
      ])}
      updateTrigger={0}
      layoutType={LayoutTypeStr.Tidy}
    />
  );
};

function deleteRandomNode(root: Node, num: number) {
  while (num > 0 && root.children.length > 0) {
    let candidates: {
      node: Node;
      parent: Node;
      i: number;
      depth: number;
    }[] = [];
    visit(root, (node, depth) => {
      if (num === 0) {
        return;
      }

      for (let i = 0; i < node.children.length; i++) {
        if (node.children[i].children.length === 0) {
          candidates.push({
            node: node.children[i],
            parent: node,
            i,
            depth: depth + 1,
          });
          break;
        }
      }
    });

    candidates.sort((a, b) => a.depth - b.depth);
    candidates = candidates.slice(-num);
    for (const { parent, i } of candidates) {
      parent.children.splice(i, 1);
      num--;
      if (num === 0) {
        break;
      }
    }
  }
}

function insertRandomNodeDepthFirst(root: Node, num: number = 1) {
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

function insertRandomNodeBreadthFirst(root: Node, num: number = 1) {
  let nodes: [Node, number][] = [];
  visit(root, (node, depth) => {
    nodes.push([node, depth]);
  });

  nodes.sort((a, b) => a[1] - b[1]);
  nodes = nodes.filter(([node, d]) => node.children.length < 5);
  nodes = nodes.slice(0, 40);
  for (let i = 0; i < num; i++) {
    const [node, d] = nodes[(Math.random() * nodes.length) | 0];
    const child = createNode();
    child.parentId = node.id;
    node.children.push(child);
    nodes.push([child, d + 1]);
    if (nodes.length === 80) {
      nodes.sort((a, b) => a[1] - b[1]);
      nodes = nodes.filter(([node, d]) => node.children.length < 5);
      nodes = nodes.slice(0, 40);
    }
  }
}

function node(width: number, height: number, children: Node[] = []): Node {
  return {
    x: 0,
    y: 0,
    width,
    height,
    children,
  };
}

function nodeNum(root: Node) {
  let count = 0;
  visit(root, () => count++);
  return count;
}
