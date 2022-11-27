import { Row, Switch, Button } from 'antd';
import { SVGProps, useEffect, useRef, useState } from 'react';
import { Renderer } from '../renderer';
import { InnerNode } from '../tidy';
import { LayoutTypeStr, TidyComponent } from '../TidyComponent';
import { createTree } from '../utils';
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

export function AnimateTransform() {
  const [root, setRoot] = useState(() => createTree(50));
  const [type, setLayoutType] = useState(LayoutTypeStr.Tidy);
  const manualChangedRef = useRef(false);
  useEffect(() => {
    let done = false;
    const run = () => {
      if (done || manualChangedRef.current) {
        return;
      }

      setLayoutType((type) => {
        if (type === LayoutTypeStr.Basic) {
          return LayoutTypeStr.Tidy;
        } else {
          return LayoutTypeStr.Basic;
        }
      });
      setTimeout(run, 5000);
    };

    setTimeout(run, 5000);
    return () => {
      done = true;
    };
  }, []);
  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        flexDirection: 'column',
        border: '1px solid rgba(128,128,128,0.3)',
        padding: 16,
        borderRadius: 8,
        maxWidth: 600,
        boxShadow: '0px 0px 8px rgba(128,128,128,0.3)',
        position: 'relative',
      }}
      onClick={() => {
        manualChangedRef.current = true;
      }}
    >
      <TidyComponent root={root} layoutType={type} style={{ height: 400 }} />
      <Button
        onClick={() => {
          setRoot(createTree(50));
          manualChangedRef.current = true;
        }}
        icon={<MaterialSymbolsRefreshRounded />}
        type="text"
        size="small"
        style={{ position: 'absolute', top: 12, right: 12 }}
      />
      <div
        style={{
          color: 'grey',
          display: 'flex',
          flexDirection: 'row',
          marginTop: 16,
        }}
      >
        <span style={{ marginRight: 8 }}>Naive</span>
        <Switch
          checked={type === LayoutTypeStr.Tidy}
          onChange={(v) => {
            if (v) {
              setLayoutType(LayoutTypeStr.Tidy);
            } else {
              setLayoutType(LayoutTypeStr.Basic);
            }
          }}
        />
        <span style={{ marginLeft: 8 }}>Tidy</span>
      </div>
    </div>
  );
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

function MaterialSymbolsRefreshRounded(props: SVGProps<SVGSVGElement>) {
  return (
    <svg width="1em" height="1em" viewBox="0 0 24 24" {...props}>
      <path
        fill="#575757"
        d="M12 20q-3.35 0-5.675-2.325Q4 15.35 4 12q0-3.35 2.325-5.675Q8.65 4 12 4q1.725 0 3.3.713q1.575.712 2.7 2.037V5q0-.425.288-.713Q18.575 4 19 4t.712.287Q20 4.575 20 5v5q0 .425-.288.712Q19.425 11 19 11h-5q-.425 0-.712-.288Q13 10.425 13 10t.288-.713Q13.575 9 14 9h3.2q-.8-1.4-2.187-2.2Q13.625 6 12 6Q9.5 6 7.75 7.75T6 12q0 2.5 1.75 4.25T12 18q1.725 0 3.188-.913q1.462-.912 2.187-2.437q.125-.275.413-.462q.287-.188.587-.188q.575 0 .863.4q.287.4.062.9q-.95 2.125-2.925 3.412Q14.4 20 12 20Z"
      ></path>
    </svg>
  );
}
