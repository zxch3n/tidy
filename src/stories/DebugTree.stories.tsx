import { useEffect, useRef } from 'react';
import { Renderer } from '../renderer';
import { debugStrToTree } from './debugToTree';

export default {
  title: 'DebugTree',
  component: DebugTree,
};

interface Props {
  input: string;
}

function DebugTree({ input }: Props) {
  const root = debugStrToTree(input);
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

  return <div ref={containerRef} style={{ width: '100%', minHeight: 500 }} />;
}

/**
 * Primary UI component for user interaction
 */
export const Debug = () => {
  const partial = `x: 0, y: 0, width: 44, height: 39, rx: 0, id: 4435718616606173925
    x: -111.5, y: 49, width: 27, height: 11, rx: -111.5, id: 3320828880214741665
    x: 18.5, y: 49, width: 45, height: 12, rx: 18.5, id: 7550641343832520649
        x: -64, y: 71, width: 42, height: 35, rx: -82.5, id: 18345003885030480740
            x: -64, y: 116, width: 45, height: 19, rx: 0, id: 12789160440584134121
                x: -64, y: 145, width: 26, height: 19, rx: 0, id: 6812743926685230063
                    x: -64, y: 174, width: 48, height: 10, rx: 0, id: 618833351226656227
        x: -5.5, y: 71, width: 49, height: 41, rx: -24, id: 325291415541072884
        x: 51, y: 71, width: 44, height: 37, rx: 32.5, id: 5575219789196240031
            x: 33.5, y: 118, width: 9, height: 46, rx: -17.5, id: 11440530225019796262
        x: 104, y: 71, width: 42, height: 14, rx: 85.5, id: 11962458253021963155`;
  const full = `x: 0, y: 0, width: 44, height: 39, rx: 0, id: 4435718616606173925
    x: -18.5, y: 49, width: 45, height: 12, rx: -18.5, id: 7550641343832520649
        x: -104, y: 71, width: 42, height: 14, rx: -85.5, id: 11962458253021963155
        x: -51, y: 71, width: 44, height: 37, rx: -32.5, id: 5575219789196240031
            x: -68.5, y: 118, width: 9, height: 46, rx: -17.5, id: 11440530225019796262
        x: 5.5, y: 71, width: 49, height: 41, rx: 24, id: 325291415541072884
        x: 64, y: 71, width: 42, height: 35, rx: 82.5, id: 18345003885030480740
            x: 64, y: 116, width: 45, height: 19, rx: 0, id: 12789160440584134121
                x: 64, y: 145, width: 26, height: 19, rx: 0, id: 6812743926685230063
                    x: 64, y: 174, width: 48, height: 10, rx: 0, id: 618833351226656227
    x: 111.5, y: 49, width: 27, height: 11, rx: 111.5, id: 3320828880214741665`;

  return (
    <div>
      <h2>Partial Layout</h2>
      <DebugTree input={partial} />
      <h2>Full Layout</h2>
      <DebugTree input={full} />
    </div>
  );
};

Debug.args = {};
