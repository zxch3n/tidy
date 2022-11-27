import { useCallback, useRef, useState } from 'react';
import { useDebounce } from 'react-use';
import { Node } from '../tidy';
import { LayoutTypeStr, TidyComponent } from '../TidyComponent';
import {
  createTree,
  insertRandomNodeDepthFirst,
  node,
  nodeNum,
  visit,
} from '../utils';

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
  const [root] = useState(() => {
    return createTree(1);
  });
  const prevNum = useRef(1);
  useDebounce(
    () => {
      const currentNum = nodeNum(root);
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
    <div onClick={addNode} style={{ display: 'flex', height: 800 }}>
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
        node(10, 10, [node(300, 10)]),
        node(10, 10),
        node(10, 10),
        node(10, 10),
        // node(10, 80),
      ])}
      updateTrigger={0}
      layoutType={LayoutTypeStr.Tidy}
      style={{ height: 500 }}
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
