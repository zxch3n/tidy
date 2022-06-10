import React, { useCallback, useEffect, useRef, useState } from 'react';
import { Renderer } from '../renderer';
import { Node } from '../tidy';
import { LayoutTypeStr } from '../TidyComponent';
import { createNode, createTree, visit } from '../utils';
import { debugStrToTree } from './debugToTree';

export default {
  title: 'DebugTree',
  component: DebugTree,
};

interface Props {
  input: string;
}

function DebugTree({ input }: Props) {
  const root = debugStrToTree(input);
  const renderRef = useRef<Renderer>();
  const containerRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const func = async () => {
      renderRef.current = new Renderer(containerRef.current!);
      renderRef.current.init(root);
    };

    func();
    return () => {
      renderRef.current?.dispose();
    };
  }, []);

  return <div ref={containerRef} style={{ width: '100%', minHeight: 500 }} />;
}

/**
 * Primary UI component for user interaction
 */
export const Debug = () => {
  const partial = `x: 0, y: 0, width: 6, height: 34, rx: 78.6875, mod: -78.6875
    x: -78.6875, y: 44, width: 29, height: 11, rx: 21.25, mod: -21.25
        x: -99.9375, y: 65, width: 24, height: 34, rx: 30, mod: -30
            x: -129.9375, y: 109, width: 6, height: 20, rx: 0, mod: 0
                x: -129.9375, y: 139, width: 22, height: 48, rx: 0, mod: 0
            x: -102.9375, y: 109, width: 12, height: 49, rx: 0, mod: 27
            x: -69.9375, y: 109, width: 34, height: 27, rx: 0, mod: 60
        x: -57.4375, y: 65, width: 41, height: 26, rx: 0, mod: 42.5
    x: 25.40625, y: 44, width: 16, height: 15, rx: 0, mod: 104.09375
    x: 78.6875, y: 44, width: 41, height: 37, rx: 61.875, mod: 95.5
        x: 16.8125, y: 91, width: 45, height: 23, rx: 45.75, mod: -45.75
            x: -28.9375, y: 124, width: 28, height: 39, rx: 0, mod: 0
                x: -28.9375, y: 173, width: 19, height: 14, rx: 0, mod: 0
            x: 19.5625, y: 124, width: 49, height: 42, rx: 0, mod: 48.5
            x: 62.5625, y: 124, width: 17, height: 30, rx: 0, mod: 91.5
        x: 96.5625, y: 91, width: 25, height: 10, rx: 0, mod: 79.75
            x: 96.5625, y: 111, width: 31, height: 15, rx: 0, mod: 0
        x: 140.5625, y: 91, width: 37, height: 15, rx: 0, mod: 123.75`;
  const full = `x: 0, y: 0, width: 6, height: 34, rx: 78.6875, mod: -78.6875
    x: -78.6875, y: 44, width: 29, height: 11, rx: 21.25, mod: -21.25
        x: -99.9375, y: 65, width: 24, height: 34, rx: 30, mod: -30
            x: -129.9375, y: 109, width: 6, height: 20, rx: 0, mod: 0
                x: -129.9375, y: 139, width: 22, height: 48, rx: 0, mod: 0
            x: -102.9375, y: 109, width: 12, height: 49, rx: 0, mod: 27
            x: -69.9375, y: 109, width: 34, height: 27, rx: 0, mod: 60
        x: -57.4375, y: 65, width: 41, height: 26, rx: 0, mod: 42.5
    x: 10.625, y: 44, width: 16, height: 15, rx: 0, mod: 89.3125
    x: 78.6875, y: 44, width: 41, height: 37, rx: 61.875, mod: 95.5
        x: 16.8125, y: 91, width: 45, height: 23, rx: 45.75, mod: -45.75
            x: -28.9375, y: 124, width: 28, height: 39, rx: 0, mod: 0
                x: -28.9375, y: 173, width: 19, height: 14, rx: 0, mod: 0
            x: 19.5625, y: 124, width: 49, height: 42, rx: 0, mod: 48.5
            x: 62.5625, y: 124, width: 17, height: 30, rx: 0, mod: 91.5
        x: 96.5625, y: 91, width: 25, height: 10, rx: 0, mod: 79.75
            x: 96.5625, y: 111, width: 31, height: 15, rx: 0, mod: 0
        x: 140.5625, y: 91, width: 37, height: 15, rx: 0, mod: 123.75`;
  return (
    <div>
      <h2>Partial Layout</h2>
      <DebugTree input={partial} />
      <h2>Full Layout</h2>
      <DebugTree input={full} />
    </div>
  );
};

Debug.args = {};
