import {
  Rect,
  ZRenderType,
  dispose,
  init,
  BezierCurve,
  Group,
  BezierCurveShape,
} from 'zrender';
import { Disposable } from './dispose';
import { InnerNode } from './tidy';
import { visit } from './utils';

export class Renderer extends Disposable {
  private render: ZRenderType;
  private root: InnerNode | undefined;
  private group: Group | undefined;
  private nodeMap: Map<number, InnerNode> = new Map();
  private rectMap: Map<number, Rect> = new Map();
  private lineSourceMap: Map<number, { line: BezierCurve; id: number }[]> =
    new Map();
  private lineTargetMap: Map<number, { line: BezierCurve; id: number }[]> =
    new Map();
  constructor(container: HTMLElement) {
    super();
    this.render = init(container);
    this._register({
      dispose: () => {
        dispose(this.render);
      },
    });
  }

  init(root: InnerNode) {
    this.root = root;
    const g = new Group();
    this.group = g;
    this.render.add(g);
    g.setPosition([this.render.getWidth() / 2, 12]);
    visit(root, (node) => {
      this.addNode(node, g);
    });
    this.rescale();
  }

  private rescale() {
    if (!this.root || !this.group) {
      return;
    }
    const g = this.group;
    const gBox = g.getBoundingRect();
    const scale = Math.min(
      this.render.getWidth() / (gBox.width + 20),
      this.render.getHeight() / (gBox.height + 20),
      5,
    );
    g.animateTo({ scaleX: scale, scaleY: scale });
  }

  private addNode(node: InnerNode, g: Group, createToParentLine = false) {
    const rect = new Rect({
      shape: {
        x: node.x - node.width / 2,
        y: node.y,
        width: node.width,
        height: node.height,
        r: 4,
      },
      style: {
        stroke: '#2b5de9',
        fill: '#a8bbf0',
      },
    });
    this.rectMap.set(node.id, rect);
    this.nodeMap.set(node.id, node);
    g.add(rect);

    for (const child of node.children) {
      this.addLine(node, child, g);
    }

    if (createToParentLine && node.parentId) {
      this.addLine(this.nodeMap.get(node.parentId)!, node, g);
    }
  }

  private addLine(node: InnerNode, child: InnerNode, g: Group) {
    const line = new BezierCurve({
      shape: getBezierCurveShape(node, child),
      style: {
        stroke: '#2b5de9',
      },
    });

    g.add(line);
    get(this.lineSourceMap, node.id, [])!.push({ line, id: child.id });
    get(this.lineTargetMap, child.id, []).push({ line, id: node.id });
  }

  update() {
    if (!this.root || !this.group) {
      return;
    }
    const g = this.group;

    visit(this.root, (node) => {
      if (!this.rectMap.has(node.id)) {
        this.addNode(node, g, true);
        return;
      }

      const rect = this.rectMap.get(node.id)!;
      if (eq(rect.x, node.x - node.width / 2) && eq(rect.y, node.y)) {
        return;
      }

      rect.animateTo({ shape: { x: node.x - node.width / 2, y: node.y } });
      const outLines = get(this.lineSourceMap, node.id, []);
      for (const { id, line } of outLines) {
        const child = this.nodeMap.get(id)!;
        line.animateTo({
          shape: getBezierCurveShape(node, child),
        });
      }
      const inLines = get(this.lineTargetMap, node.id, []);
      for (const { id, line } of inLines) {
        const child = node;
        const parent = this.nodeMap.get(id)!;
        line.animateTo({
          shape: getBezierCurveShape(parent, child),
        });
      }
    });
    this.rescale();
  }
}

function getBezierCurveShape(
  parent: InnerNode,
  child: InnerNode,
): Partial<BezierCurveShape> {
  return {
    x1: parent.x,
    y1: parent.y + parent.height,
    x2: child.x,
    y2: child.y,
    cpx1: parent.x,
    cpy1: (child.y + parent.y + parent.height) / 2,
    cpx2: child.x,
    cpy2: (child.y + parent.y + parent.height) / 2,
  };
}

function eq(a: number, b: number) {
  return Math.abs(a - b) < 1e-6;
}

function get<K, V>(map: Map<K, V>, key: K, defaultValue: V) {
  if (map.has(key)) {
    return map.get(key)!;
  }
  map.set(key, defaultValue);
  return defaultValue;
}
