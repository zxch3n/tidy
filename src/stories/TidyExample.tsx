import { InputNumber, Select, Slider } from 'antd';
import React, { useState } from 'react';
import { useDebounce } from 'react-use';

import { LayoutTypeStr, TidyComponent } from '../TidyComponent';
import {
  createTree,
  deleteRandomNode,
  insertRandomNodeDepthFirst,
  nodeNum,
} from '../utils';
import { Card } from './LayeredVsNonLayered';

const { Option } = Select;
const root = createTree(50);

function Label({
  style,
  ...props
}: {
  style?: React.CSSProperties;
  children?: any;
}) {
  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        width: 120,
        padding: 4,
        ...style,
      }}
      {...props}
    />
  );
}

export function TidyExample() {
  const [layoutType, setLayoutType] = useState(LayoutTypeStr.Tidy);
  const [updateTrigger, setUpdate] = useState(0);
  const [num, setNum] = useState(50);
  useDebounce(
    () => {
      let currentNum = nodeNum(root);
      if (num < currentNum) {
        deleteRandomNode(root, currentNum - num);
      } else if (num > currentNum) {
        insertRandomNodeDepthFirst(root, num - currentNum);
      }

      setUpdate((updateTrigger) => updateTrigger + 1);
    },
    100,
    [num],
  );
  return (
    <Card style={{ padding: 16 }}>
      <TidyComponent
        root={root}
        updateTrigger={updateTrigger}
        layoutType={layoutType}
      />
      <div style={{ display: 'flex', marginBottom: 12, width: 400 }}>
        <Label style={{ width: 140 }}>Layout Type:</Label>
        <Select
          value={layoutType}
          onChange={(v) => setLayoutType(v)}
          style={{ width: 150 }}
        >
          <Option value={LayoutTypeStr.Tidy}>Tidy</Option>
          <Option value={LayoutTypeStr.LayeredTidy}>Layered Tidy</Option>
          <Option value={LayoutTypeStr.Basic}>Naive</Option>
        </Select>
      </div>

      <div style={{ display: 'flex', width: 400 }}>
        <InputNumber
          value={num}
          min={10}
          max={1000}
          onChange={(v) => setNum(v)}
          addonAfter="Nodes"
          style={{ width: 140, marginRight: 20 }}
        />
        <Slider
          value={num}
          onChange={(v) => setNum(v)}
          min={10}
          max={1000}
          style={{ width: 300 }}
        />
      </div>
    </Card>
  );
}
