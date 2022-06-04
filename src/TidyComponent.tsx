import { useEffect, useRef, useState } from 'react';
import { Renderer } from './renderer';
import { InnerNode, LayoutType, Node, TidyLayout } from './tidy';

interface Props {
  root: Node;
  layoutType?: LayoutType;
  updateTrigger?: number;
}

export const TidyComponent = ({ root, layoutType, updateTrigger }: Props) => {
  const renderRef = useRef<Renderer>();
  const containerRef = useRef<HTMLDivElement>(null);
  const layoutRef = useRef<TidyLayout>();
  useEffect(() => {
    const func = async () => {
      renderRef.current = new Renderer(containerRef.current!);
      layoutRef.current = await TidyLayout.create(layoutType);
      const innerRoot = layoutRef.current.set_root(root);
      layoutRef.current.layout();
      renderRef.current.init(innerRoot);
      // TODO: Draw
    };

    func();
    return () => {
      renderRef.current?.dispose();
    };
  }, []);
  useEffect(() => {
    if (!layoutRef.current || !renderRef.current) {
      return;
    }

    layoutRef.current.layout(true);
    renderRef.current.update();
  }, [updateTrigger]);

  return <div ref={containerRef} style={{ width: '100%', minHeight: 500 }} />;
};
