import { useEffect, useLayoutEffect, useRef } from 'react';

import { Renderer } from './renderer';
import { LayoutType, Node, TidyLayout } from './tidy';

export enum LayoutTypeStr {
  Tidy = 'tidy',
  Basic = 'basic',
  LayeredTidy = 'layeredTidy',
}

interface Props {
  root: Node;
  layoutType?: LayoutTypeStr;
  updateTrigger?: number;
}

function getLayoutType(type?: LayoutTypeStr) {
  if (type == null) {
    return LayoutType.Tidy;
  }

  switch (type) {
    case LayoutTypeStr.Basic:
      return LayoutType.Basic;
    case LayoutTypeStr.Tidy:
      return LayoutType.Tidy;
    case LayoutTypeStr.LayeredTidy:
      return LayoutType.LayeredTidy;
    default:
      throw new Error();
  }
}

export const TidyComponent = ({ root, layoutType, updateTrigger }: Props) => {
  const renderRef = useRef<Renderer>();
  const containerRef = useRef<HTMLDivElement>(null);
  const layoutRef = useRef<TidyLayout>();
  const type = getLayoutType(layoutType);
  useLayoutEffect(() => {
    if (!layoutRef.current || !renderRef.current) {
      return;
    }

    layoutRef.current.changeLayoutType(type);
    layoutRef.current.layout(true);
    renderRef.current.update();
  }, [updateTrigger, type]);
  useLayoutEffect(() => {
    let done = false;
    const func = async () => {
      renderRef.current = new Renderer(containerRef.current!);
      layoutRef.current = await TidyLayout.create(type);
      if (done) {
        return;
      }

      const innerRoot = layoutRef.current.set_root(root);
      layoutRef.current.layout();
      renderRef.current.init(innerRoot);
    };

    func();
    return () => {
      done = true;
      layoutRef.current?.dispose();
      layoutRef.current = undefined;
      renderRef.current?.clear();
    };
  }, [root]);
  useEffect(() => {
    return () => {
      renderRef.current?.dispose();
      renderRef.current = undefined;
    };
  }, []);

  return <div ref={containerRef} style={{ width: '100%', minHeight: 500 }} />;
};
