import React, { useEffect, useState } from 'react';
import { LayoutTypeStr, TidyComponent } from '../TidyComponent';
import { Node } from '../tidy';
import { Col, Row, Select, Slider, InputNumber } from 'antd';
import { createNode, createTree, visit } from '../utils';
import { useDebounce } from 'react-use';
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

function deleteRandomNode(root: Node, num: number) {
  while (num > 0 && root.children.length > 0) {
    let candidates: {
      node: Node;
      parent: Node;
      depth: number;
    }[] = [];
    visit(root, (node, depth) => {
      if (num === 0) {
        return;
      }

      for (let i = 0; i < node.children.length; i++) {
        candidates.push({
          node: node.children[i],
          parent: node,
          depth: depth + 1,
        });
      }
    });

    candidates.sort((a, b) => a.depth - b.depth);
    candidates = candidates.slice(-num);
    for (const { parent, node } of candidates) {
      parent.children.splice(parent.children.indexOf(node), 1);
      num -= nodeNum(node);
      if (num <= 0) {
        break;
      }
    }
  }
}

function insertRandomNodeDepthFirst(root: Node, num: number = 1) {
  let nodes: [Node, number][] = [];
  visit(root, (node, depth) => {
    nodes.push([node, depth]);
  });

  function filter() {
    nodes.sort((a, b) => Math.random() * 2 - 1);
    nodes.sort((a, b) => -a[1] + b[1]);
    nodes = nodes.filter(([node, d]) => node.children.length < 4);
    nodes = nodes.slice(0, 20).concat(nodes.filter(([node, d]) => d < 2));
  }

  filter();
  for (let i = 0; i < num; i++) {
    const [node, d] = nodes[(Math.random() * nodes.length) | 0];
    const child = createNode();
    child.parentId = node.id;
    node.children.push(child);
    nodes.push([child, d + 1]);
    if (nodes.length % 40 === 0) {
      filter();
    }
  }
}

function insertRandomNodeBreadthFirst(root: Node, num: number = 1) {
  let nodes: [Node, number][] = [];
  visit(root, (node, depth) => {
    nodes.push([node, depth]);
  });

  nodes.sort((a, b) => a[1] - b[1]);
  nodes = nodes.filter(([node, d]) => node.children.length < 5);
  nodes = nodes.slice(0, 40);
  for (let i = 0; i < num; i++) {
    const [node, d] = nodes[(Math.random() * nodes.length) | 0];
    const child = createNode();
    child.parentId = node.id;
    node.children.push(child);
    nodes.push([child, d + 1]);
    if (nodes.length === 80) {
      nodes.sort((a, b) => a[1] - b[1]);
      nodes = nodes.filter(([node, d]) => node.children.length < 5);
      nodes = nodes.slice(0, 40);
    }
  }
}

function node(width: number, height: number, children: Node[] = []): Node {
  return {
    x: 0,
    y: 0,
    width,
    height,
    children,
  };
}

function nodeNum(root: Node) {
  let count = 0;
  visit(root, () => count++);
  return count;
}
