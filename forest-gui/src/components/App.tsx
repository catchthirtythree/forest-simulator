import { ReactElement, useEffect, useRef, useState } from 'react';
import { get_forest, update_forest } from '../commands/forest_commands';
import './App.css';

const DEFAULT_SEED: number = 123123;
const DEFAULT_CELL_SIZE: number = 8;
const DEFAULT_WIDTH: number = 120 * DEFAULT_CELL_SIZE;
const DEFAULT_HEIGHT: number = 80 * DEFAULT_CELL_SIZE;
const DEFAULT_ALPHA: number = 255;

export function convert_index_to_xy(index: number, width: number, cell_size: number): [number, number] {
  const adjusted_index = index * cell_size;
  const x = adjusted_index % width;
  const y = Math.floor(adjusted_index / width);
  return [x, y];
}

export function convert_xy_to_index(x: number, y: number, width: number, cell_size: number): number {
  return y * width * cell_size + x;
}

export function calculate_cell_rgb(cell: number): [number, number, number] {
  let fill_color = 0x000000;

  if (is_tree(cell)) {
    fill_color += 0x00ff00;
  }

  if (is_jack(cell)) {
    fill_color += 0xff0000;
  }

  if (is_bear(cell)) {
    fill_color += 0x0000ff;
  }

  return [
    (fill_color & 0xff0000) >> 8 * 2,
    (fill_color & 0x00ff00) >> 8 * 1,
    (fill_color & 0x0000ff) >> 8 * 0,
  ];
}

export function is_tree(cell: number): boolean {
  return ((cell & 0x00ff) >> 4 * 0) > 0;
}

export function is_jack(cell: number): boolean {
  return ((cell & 0x0f00) >> 4 * 2) > 0;
}

export function is_bear(cell: number): boolean {
  return ((cell & 0xf000) >> 4 * 3) > 0;
}

export function draw_pixel(
  image_data: ImageData,
  cell: number,
  index: number,
  width: number,
  cell_size: number
): void {
  const [x, y] = convert_index_to_xy(index, width, cell_size);
  const [r, g, b] = calculate_cell_rgb(cell);
  const adjusted_index = convert_xy_to_index(x, y, width, cell_size);

  for (let dy = 0; dy < cell_size; dy++) {
    for (let dx = 0; dx < cell_size; dx++) {
      const calculated_index = adjusted_index + dx + (dy * width);

      image_data.data[4 * calculated_index + 0] = r;
      image_data.data[4 * calculated_index + 1] = g;
      image_data.data[4 * calculated_index + 2] = b;
      image_data.data[4 * calculated_index + 3] = DEFAULT_ALPHA;
    }
  }
}

export function App(props: {}): ReactElement<any, any> {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [map, setMap] = useState<number[]>([]);
  const [cellSize, setCellSize] = useState<number>(DEFAULT_CELL_SIZE);
  const [width, setWidth] = useState<number>(DEFAULT_WIDTH);
  const [height, setHeight] = useState<number>(DEFAULT_HEIGHT);

  useEffect(() => {
    if (!canvasRef.current) return;

    let canvas = canvasRef.current;
    let ctx = canvas.getContext('2d');

    if (!ctx) return;

    let context = ctx;
    let image_data = context.getImageData(0, 0, width, height);

    map.forEach((cell, index) => {
      draw_pixel(image_data, cell, index, width, cellSize);
    });

    context.putImageData(image_data, 0, 0);
  }, [map, width, height, cellSize]);

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
          get_forest().then(map => {
            if (!map) return;
            setMap(map);
          });
        }}>Get the map</button>

        <button onClick={(event) => {
          update_forest().then(_ => get_forest().then(map => {
            if (!map) return;
            setMap(map);
          }));
        }}>Update the map</button>
      </div>
    </div>
  );
}
