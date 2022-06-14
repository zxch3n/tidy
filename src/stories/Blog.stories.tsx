import { Row } from 'antd';
import { useEffect, useRef } from 'react';
import { Renderer } from '../renderer';
import { InnerNode, Node } from '../tidy';
import { LayeredVsNonLayered, Card } from './LayeredVsNonLayered';
import { TidyExample } from './TidyExample';

export default {
  title: 'Blog',
};

export function Layered() {
  return <LayeredVsNonLayered />;
}

export function InteractiveTidy() {
  return <TidyExample />;
}

export function AestheticRule7() {
  const tidyRoot = {
    x: 0,
    y: 0,
    width: 10,
    height: 10,
    children: [
      {
        x: -82.5,
        y: 50,
        width: 10,
        height: 10,
        children: [
          {
            x: -82.5,
            y: 100,
            width: 300,
            height: 10,
            children: [],
            parentId: 2,
            id: 3,
          },
        ],
        parentId: 1,
        id: 2,
      },
      {
        x: -41.25,
        y: 50,
        width: 10,
        height: 10,
        children: [],
        parentId: 1,
        id: 4,
      },
      {
        x: 0,
        y: 50,
        width: 10,
        height: 10,
        children: [],
        parentId: 1,
        id: 5,
      },
      {
        x: 41.25,
        y: 50,
        width: 10,
        height: 10,
        children: [],
        parentId: 1,
        id: 6,
      },
      {
        x: 82.5,
        y: 50,
        width: 10,
        height: 80,
        children: [],
        parentId: 1,
        id: 7,
      },
    ],
    id: 1,
  };
  const errorRoot = {
    x: 0,
    y: 0,
    width: 10,
    height: 10,
    children: [
      {
        x: -80,
        y: 50,
        width: 10,
        height: 10,
        children: [
          {
            x: -80,
            y: 100,
            width: 300,
            height: 10,
            children: [],
            parentId: 2,
            id: 3,
          },
        ],
        parentId: 1,
        id: 2,
      },
      {
        x: -60,
        y: 50,
        width: 10,
        height: 10,
        children: [],
        parentId: 1,
        id: 4,
      },
      {
        x: -40,
        y: 50,
        width: 10,
        height: 10,
        children: [],
        parentId: 1,
        id: 5,
      },
      {
        x: -20,
        y: 50,
        width: 10,
        height: 10,
        children: [],
        parentId: 1,
        id: 6,
      },
      {
        x: 85,
        y: 50,
        width: 10,
        height: 80,
        children: [],
        parentId: 1,
        id: 7,
      },
    ],
    id: 1,
  };
  return (
    <Row style={{ maxWidth: 1000 }}>
      <Card style={{ height: 300 }}>
        <Tree root={errorRoot} />
        <p style={{ color: 'rgb(128, 128, 128)', textAlign: 'center' }}>
          Not Symmetric
        </p>
      </Card>
      <Card style={{ height: 300 }}>
        <Tree root={tidyRoot} />
        <p style={{ color: 'rgb(128, 128, 128)', textAlign: 'center' }}>
          Symmetric
        </p>
      </Card>
    </Row>
  );
}

function Tree({ root }: { root: InnerNode }) {
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

  return (
    <div
      ref={containerRef}
      style={{ width: '100%', flexGrow: 1, padding: 16 }}
    />
  );
}
