import _initWasm, {
  InitInput,
  InitOutput,
  Tidy,
  Tidy as TidyWasm,
} from '../wasm_dist/wasm';
import { Disposable } from './dispose';
import { visit } from './utils';

export enum LayoutType {
  Basic = 'basic',
}

let promise: Promise<InitOutput> | undefined;

export function initWasm(
  module_or_path?: InitInput | Promise<InitInput>,
  maybe_memory?: WebAssembly.Memory,
): Promise<InitOutput> {
  if (!promise) {
    promise = _initWasm(module_or_path, maybe_memory);
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
  static async create(type: LayoutType = LayoutType.Basic) {
    await initWasm();
    return new TidyLayout(type);
  }

  private constructor(type: LayoutType = LayoutType.Basic) {
    super();
    if (type === LayoutType.Basic) {
      this.tidy = TidyWasm.with_basic_layout();
    } else {
      throw new Error('not implemented');
    }
    this._register({
      dispose: () => {
        this.tidy.free();
      },
    });
  }

  layout(updated = false) {
    if (updated) {
      visit(this.root!, (node) => {
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
      for (const child of node.children) {
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
