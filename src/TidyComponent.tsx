import { useEffect, useRef } from 'react';

import { Renderer } from './renderer';
import { LayoutType, Node, TidyLayout } from './tidy';

export enum LayoutTypeStr {
  Tidy = 'tidy',
  Basic = 'basic',
}

interface Props {
  root: Node;
  layoutType?: LayoutTypeStr;
  updateTrigger?: number;
}

function getLayoutType(type?: LayoutTypeStr) {
  return type === LayoutTypeStr.Tidy ? LayoutType.Tidy : LayoutType.Basic;
}

export const TidyComponent = ({ root, layoutType, updateTrigger }: Props) => {
  const renderRef = useRef<Renderer>();
  const containerRef = useRef<HTMLDivElement>(null);
  const layoutRef = useRef<TidyLayout>();
  const type = getLayoutType(layoutType);
  useEffect(() => {
    const func = async () => {
      renderRef.current = new Renderer(containerRef.current!);
      layoutRef.current = await TidyLayout.create(type);
      const innerRoot = layoutRef.current.set_root(root);
      layoutRef.current.layout();
      console.log(innerRoot);
      renderRef.current.init(innerRoot);
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

    layoutRef.current.changeLayoutType(type);
    layoutRef.current.layout(true);
    renderRef.current.update();
  }, [updateTrigger, type]);

  return <div ref={containerRef} style={{ width: '100%', minHeight: 500 }} />;
};
