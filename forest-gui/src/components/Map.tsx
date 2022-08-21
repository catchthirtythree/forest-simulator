import { ReactElement, useEffect, useRef, useState } from "react";
import './Map.css';

const DEFAULT_CELL_DRAW_SIZE: number = 8;
const DEFAULT_ALPHA: number = 255;

export function convert_index_to_xy(index: number, width: number, cellSize: number): [number, number] {
  const adjusted_index = index * cellSize;
  const x = adjusted_index % width;
  const y = Math.floor(adjusted_index / width);
  return [x, y];
}

export function convert_xy_to_index(x: number, y: number, width: number, cellSize: number): number {
  return y * width * cellSize + x;
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
  cellSize: number
): void {
  const [x, y] = convert_index_to_xy(index, width, cellSize);
  const [r, g, b] = calculate_cell_rgb(cell);
  const adjusted_index = convert_xy_to_index(x, y, width, cellSize);

  for (let dy = 0; dy < cellSize; dy++) {
    for (let dx = 0; dx < cellSize; dx++) {
      const calculated_index = adjusted_index + dx + (dy * width);
      image_data.data[4 * calculated_index + 0] = r;
      image_data.data[4 * calculated_index + 1] = g;
      image_data.data[4 * calculated_index + 2] = b;
      image_data.data[4 * calculated_index + 3] = DEFAULT_ALPHA;
    }
  }
}

export default function Map(props: {
  map: number[],
  width: number,
  height: number,
}): ReactElement<any, any> {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  console.log('width', props.width, 'height', props.height);
  const [width, setWidth] = useState<number>(props.width * DEFAULT_CELL_DRAW_SIZE);
  const [height, setHeight] = useState<number>(props.height * DEFAULT_CELL_DRAW_SIZE);

  useEffect(() => {
    console.log('redraw?');
    if (!canvasRef.current) return;
    console.log('canvas ref check');

    let canvas = canvasRef.current;
    let ctx = canvas.getContext('2d');

    if (!ctx) return;
    console.log('context check');

    let context = ctx;
    let image_data = context.getImageData(0, 0, width, height);

    console.log(props.map);
    console.log(width);
    console.log(height);

    props.map.forEach((cell, index) => {
      draw_pixel(image_data, cell, index, width, DEFAULT_CELL_DRAW_SIZE);
    });

    context.putImageData(image_data, 0, 0);
  }, [props, width, height]);

  return (
    <canvas
      id="Map_canvas"
      ref={canvasRef}
      style={{
        backgroundColor: '#65c399',
        height: width >= height ? '100%' : 'unset',
        width: width < height ? '100%' : 'unset'
      }}
      width={width}
      height={height}
    ></canvas>
  );
}
