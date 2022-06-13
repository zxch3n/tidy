import _initWasm, {
  InitInput,
  InitOutput,
  Tidy,
  Tidy as TidyWasm,
  WasmLayoutType as LayoutType,
} from '../wasm_dist/wasm';
import { Disposable } from './dispose';
import { visit } from './utils';

export { LayoutType };

let promise: Promise<InitOutput> | undefined;

export function initWasm(
  module_or_path?: InitInput | Promise<InitInput>,
): Promise<InitOutput> {
  if (!promise) {
    promise = _initWasm(module_or_path);
  }

  return promise;
}

export interface Node {
  id?: number;
  width: number;
  height: number;
  parentId?: number;
  x: number;
  y: number;
  children: Node[];
}

export interface InnerNode {
  id: number;
  width: number;
  height: number;
  parentId?: number;
  x: number;
  y: number;
  children: InnerNode[];
}

let nullId = -1;
const NULL_ID = () => {
  if (nullId === -1) {
    nullId = Tidy.null_id();
  }
  return nullId;
};
export class TidyLayout extends Disposable {
  private tidy: TidyWasm;
  private nextId = 1;
  private root: InnerNode | undefined;
  private idToNode: Map<number, InnerNode> = new Map();
  static async create(type: LayoutType = LayoutType.Tidy) {
    await initWasm();
    return new TidyLayout(type);
  }

  private constructor(type: LayoutType = LayoutType.Tidy) {
    super();
    if (type === LayoutType.Basic) {
      this.tidy = TidyWasm.with_basic_layout(40, 10);
    } else if (type === LayoutType.Tidy) {
      this.tidy = TidyWasm.with_tidy_layout(40, 10);
    } else if (type === LayoutType.LayeredTidy) {
      this.tidy = TidyWasm.with_layered_tidy(40, 10);
    } else {
      throw new Error('not implemented');
    }
    this._register({
      dispose: () => {
        this.tidy.free();
      },
    });
  }

  changeLayoutType(type: LayoutType) {
    this.tidy.change_layout(type);
  }

  layout(updated = false) {
    if (updated) {
      const removedNodeId = new Set(this.idToNode.keys());
      visit(this.root!, (node) => {
        removedNodeId.delete(node.id);
        if (this.idToNode.has(node.id)) {
          return;
        }

        this.idToNode.set(node.id, node);
        this.tidy.add_node(
          node.id,
          node.width,
          node.height,
          node.parentId ?? NULL_ID(),
        );
      });

      for (const nodeId of removedNodeId) {
        this.tidy.remove_node(nodeId);
        this.idToNode.delete(nodeId);
      }
    }

    this.tidy.layout();
    const positions = this.tidy.get_pos();
    for (let i = 0; i < positions.length; i += 3) {
      const id = positions[i] | 0;
      const node = this.idToNode.get(id)!;
      node.x = positions[i + 1];
      node.y = positions[i + 2];
    }
  }

  set_root(root: Node): InnerNode {
    //TODO: Free old nodes
    const stack = [root];
    const ids: number[] = [];
    const width: number[] = [];
    const height: number[] = [];
    const parents: number[] = [];
    while (stack.length) {
      const node = stack.pop()!;
      if (node.id == null) {
        node.id = this.nextId++;
      }

      ids.push(node.id!);
      width.push(node.width);
      height.push(node.height);
      parents.push(node.parentId ?? NULL_ID());
      this.idToNode.set(node.id!, node as InnerNode);
      for (const child of node.children.concat().reverse()) {
        if (child.parentId == null) {
          child.parentId = node.id;
        }

        stack.push(child);
      }
    }

    this.root = root as InnerNode;
    this.tidy.data(
      new Uint32Array(ids),
      new Float64Array(width),
      new Float64Array(height),
      new Uint32Array(parents),
    );

    return this.root;
  }
}
