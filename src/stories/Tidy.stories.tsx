import React from 'react';
import { LayoutType, Node } from '../tidy';
import { TidyComponent } from '../TidyComponent';
import { ComponentStory, ComponentMeta } from '@storybook/react';

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
  return <TidyComponent root={root} layoutType={layoutType} />;
};

TidyLayout.args = {
  root: createTree(100) as Node,
};

function createNode(): Node {
  return {
    id: (Math.random() * 1e9) | 0,
    height: 20 * Math.random() + 10,
    width: 20 * Math.random() + 10,
    x: 0,
    y: 0,
    children: [],
  };
}

function createTree(num: number): Node {
  const root = createNode();
  const arr = [root];
  for (let i = 0; i < num; i++) {
    const child = createNode();
    const parentIndex = (arr.length * Math.random()) | 0;
    const parent = arr[parentIndex];
    parent.children.push(child);
    child.parentId = parent.id;
    arr.push(child);
    if (parent.children.length > 5) {
      arr.splice(parentIndex, 1);
    }
  }

  return root;
}
