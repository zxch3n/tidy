import React from 'react';
import { LayoutType, Node } from '../tidy';
import { TidyComponent } from '../TidyComponent';
import { ComponentStory, ComponentMeta } from '@storybook/react';
import { createTree } from '../utils';

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
  root: createTree(200) as Node,
};

TidyLayout.addNode = () => {};
