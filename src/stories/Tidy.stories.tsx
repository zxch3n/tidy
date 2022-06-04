import React, { useCallback, useState } from 'react';
import { LayoutType, Node } from '../tidy';
import { TidyComponent } from '../TidyComponent';
import { produce } from 'immer';
import { ComponentStory, ComponentMeta } from '@storybook/react';
import { createNode, createTree, visit } from '../utils';

export default {
  title: 'Tidy',
  component: TidyComponent,
} as ComponentMeta<typeof TidyComponent>;

interface Props {
  layoutType?: LayoutType;
  root: Node;
}

/**
 * Primary UI component for user interaction
 */
export const TidyLayout = ({ root, layoutType, ...props }: Props) => {
  const [updateTrigger, setUpdate] = useState(0);
  const addNode = useCallback(() => {
    let nodes: [Node, number][] = [];
    visit(root, (node, depth) => {
      if (node.children.length < 4) {
        nodes.push([node, depth]);
      }
    });

    nodes.sort((a, b) => -a[1] + b[1]);
    if (nodes.length > 20) {
      const depth = nodes[20][1];
      nodes = nodes.filter(([_, d]) => d >= depth);
    }
    const node = nodes[(Math.random() * nodes.length) | 0][0];
    const child = createNode();
    child.parentId = node.id;
    node.children.push(child);
    setUpdate((updateTrigger) => updateTrigger + 1);
  }, []);

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

TidyLayout.args = {
  root: createTree(2) as Node,
};

TidyLayout.play = () => {};
