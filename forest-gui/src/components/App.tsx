import { ReactElement, useEffect, useRef, useState } from 'react';
import { get_map, update_map } from '../commands/map_commands';
import './App.css';

const DEFAULT_SEED: number = 123123;
const DEFAULT_WIDTH: number = 120;
const DEFAULT_HEIGHT: number = 80;
const DEFAULT_CELL_SIZE: number = 1;
const DEFAULT_ALPHA: number = 255;

export function convert_index_to_xy(index: number, width: number): [number, number] {
  const x = index % width;
  const y = Math.floor(index / width);
  return [x, y];
}

export function calculate_cell_rgb(cell: number): [number, number, number] {
  let fill_color = 0x000000;

  if (is_tree(cell)) {
    fill_color += 0x0000ff;
  }

  if (is_jack(cell)) {
    fill_color += 0x00ff00;
  }

  if (is_bear(cell)) {
    fill_color += 0xff0000;
  }

  return [
    (fill_color & 0xff0000) >> 8 * 2,
    (fill_color & 0x00ff00) >> 8 * 1,
    (fill_color & 0x0000ff) >> 8 * 0,
  ];
}

export function is_tree(cell: number): boolean {
  return ((cell & 0xff) >> 4 * 0) > 0;
}

export function is_jack(cell: number): boolean {
  return ((cell & 0xff) >> 4 * 2) > 0;
}

export function is_bear(cell: number): boolean {
  return ((cell & 0xff) >> 4 * 3) > 0;
}

export function App(props: {}): ReactElement<any, any> {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [map, setMap] = useState<number[]>([]);
  const [width, setWidth] = useState<number>(DEFAULT_WIDTH);
  const [height, setHeight] = useState<number>(DEFAULT_HEIGHT);

  useEffect(() => {
    if (!canvasRef.current) return;

    let canvas = canvasRef.current;
    let ctx = canvas.getContext('2d');

    if (!ctx) return;

    let context = ctx;

    // @TODO(michael): Draw the map on the canvas.

    let image_data = context.getImageData(0, 0, width, height);

    map.forEach((cell, index) => {
      const [x, y] = convert_index_to_xy(index, width);
      const [r, g, b] = calculate_cell_rgb(cell);

      image_data.data[4 * index + 0] = r;
      image_data.data[4 * index + 1] = g;
      image_data.data[4 * index + 2] = b;
      image_data.data[4 * index + 3] = DEFAULT_ALPHA;
    });

    context.putImageData(image_data, 0, 0);
  }, [map]);

  return (
    <div id="App_container">
      <div id="App_canvas">
        <canvas
          ref={canvasRef}
          style={{
            backgroundColor: '#65c399',
          }}
          width={DEFAULT_WIDTH}
          height={DEFAULT_HEIGHT}
        ></canvas>
      </div>

      <div id="App_buttons">
        <button onClick={(event) => {
          get_map().then(map => {
            if (!map) return;
            setMap(map);
          });
        }}>Get the map</button>

        <button onClick={(event) => {
          update_map().then(_ => get_map().then(map => {
            if (!map) return;
            setMap(map);
          }));
        }}>Update the map</button>
      </div>
    </div>
  );
}
