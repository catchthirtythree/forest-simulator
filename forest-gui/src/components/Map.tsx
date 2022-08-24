import { ReactElement, useEffect, useRef, useState } from "react";
import { IForestInfo } from "../types/response";
import './Map.css';

export interface ISelectedCell {
  cell: number;
  x: number;
  y: number;
}

export class SelectedCell implements ISelectedCell {
  cell: number;
  x: number;
  y: number;

  constructor(cell: number, x: number, y: number) {
    this.cell = cell;
    this.x = x;
    this.y = y;
  }
}

export function convert_index_to_xy(index: number, width: number, cellSize: number): [number, number] {
  const adjusted_index = index * cellSize;
  const x = adjusted_index % width;
  const y = Math.floor(adjusted_index / width);
  return [x, y];
}

export function convert_xy_to_index(x: number, y: number, width: number, cellSize: number = 1): number {
  return y * width * cellSize + x;
}

export function calculate_cell_rgba(cell: number): [number, number, number, number] {
  let fill_color = 0x000000;
  let tree_age = 255;

  if (is_jack(cell)) {
    fill_color += 0xff0000;
  }

  if (is_tree(cell)) {
    // tree_age = cell & 0x00ff;
    fill_color += 0x00ff00;
  }

  if (is_bear(cell)) {
    fill_color += 0x0000ff;
  }

  return [
    (fill_color & 0xff0000) >> 8 * 2,
    (fill_color & 0x00ff00) >> 8 * 1,
    (fill_color & 0x0000ff) >> 8 * 0,
    (tree_age),
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
  const [r, g, b, a] = calculate_cell_rgba(cell);
  const adjusted_index = convert_xy_to_index(x, y, width, cellSize);

  for (let dy = 0; dy < cellSize; dy++) {
    for (let dx = 0; dx < cellSize; dx++) {
      const calculated_index = adjusted_index + dx + (dy * width);
      image_data.data[4 * calculated_index + 0] = r;
      image_data.data[4 * calculated_index + 1] = g;
      image_data.data[4 * calculated_index + 2] = b;
      image_data.data[4 * calculated_index + 3] = a;
    }
  }
}

export function show_tree_info(cell: number): string {
  let tree_age = (cell & 0x00ff) >> 4 * 0;
  if (tree_age === 0) {
    return 'None'
  }

  return `age ${tree_age}`;
}

export function show_jack_info(cell: number): string {
  let jack_level = (cell & 0x0f00) >> 4 * 2;
  if (jack_level === 0) {
    return 'None'
  }

  return `level ${jack_level}`;
}

export function show_bear_info(cell: number): string {
  let bear = (cell & 0xf000) >> 4 * 3;
  if (bear === 0) {
    return 'None'
  }

  return 'True';
}

export interface IPosition {
  top?: number;
  left?: number;
  bottom?: number;
  right?: number;
}

export default function Map(props: {
  info: IForestInfo,
  cellSize: number,
}): ReactElement<any, any> {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const width = props.info.width * props.cellSize;
  const height = props.info.height * props.cellSize;

  const [selected, setSelected] = useState<ISelectedCell | null>(null);

  const handleCanvasClick = (event: any) => {
    if (!canvasRef.current) {
      return;
    }

    const canvas = canvasRef.current;
    const rect = canvasRef.current.getBoundingClientRect();

    const [offsetX, offsetY] = [
      event.nativeEvent.offsetX,
      event.nativeEvent.offsetY,
    ];

    const [xRatio, yRatio] = [
      offsetX / rect.width / props.cellSize,
      offsetY / rect.height / props.cellSize,
    ];

    const [canvasX, canvasY] = [
      Math.floor(xRatio * canvas.width),
      Math.floor(yRatio * canvas.height),
    ];

    const index = convert_xy_to_index(
      canvasX,
      canvasY,
      props.info.width,
    );

    setSelected(new SelectedCell(props.info.map[index], offsetX, offsetY));
  }

  useEffect(() => {
    console.log('selected', selected);

    // @TODO(michael): If selected is not null, update the cell.
    // @TODO(michael): If width / height changes, set selected to null.
    if (!canvasRef.current) return;

    let canvas = canvasRef.current;
    let ctx = canvas.getContext('2d');

    if (!ctx) return;

    let context = ctx;
    let image_data = context.getImageData(0, 0, width, height);

    context.clearRect(0, 0, width, height);

    props.info.map.forEach((cell, index) => {
      draw_pixel(image_data, cell, index, width, props.cellSize);
    });

    context.putImageData(image_data, 0, 0);
  }, [props, width, height]);

  return (
    <div id="Map_container">
      <canvas
        ref={canvasRef}
        style={{
          backgroundColor: '#65c399',
          width: '100%'
        }}
        onClick={handleCanvasClick}
        width={width}
        height={height}
      />

      {/* @TODO(michael): Show popup when the user clicks on the canvas. */}
      {/* @TODO(michael): Place the popup properly. */}
      <div id="popup" hidden={selected === null ? true : false}>

        <div className="info-row">
          <span>Tree: {show_tree_info(selected?.cell ?? 0)}</span>
        </div>

        <div className="info-row">
          <span>Jack: {show_jack_info(selected?.cell ?? 0)}</span>
        </div>

        <div className="info-row">
          <span>Bear: {show_bear_info(selected?.cell ?? 0)}</span>
          <span></span>
        </div>

      </div>
    </div>
  );
}
