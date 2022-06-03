import { Rect, ZRenderType, dispose, init, BezierCurve, Group } from 'zrender';
import { Disposable } from './dispose';
import { InnerNode } from './tidy';
import { visit } from './utils';

export class Renderer extends Disposable {
  private render: ZRenderType;
  private root: InnerNode | undefined;
  private rectMap: Map<number, Rect> = new Map();
  private lineFromMap: Map<number, BezierCurve> = new Map();
  private lineToMap: Map<number, BezierCurve> = new Map();
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
    this.render.add(g);
    g.setPosition([this.render.getWidth() / 2, 12]);
    g.setScale([0.4, 0.4]);
    visit(root, (node) => {
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
      g.add(rect);

      for (const child of node.children) {
        const line = new BezierCurve({
          shape: {
            x1: node.x,
            y1: node.y + node.height,
            x2: child.x,
            y2: child.y,
            cpx1: node.x,
            cpy1: (child.y + node.y + node.height) / 2,
            cpx2: child.x,
            cpy2: (child.y + node.y + node.height) / 2,
          },
          style: {
            stroke: '#2b5de9',
          },
        });

        g.add(line);
        this.lineFromMap.set(node.id, line);
        this.lineToMap.set(child.id, line);
      }
    });
  }
}
