import { ReactElement, useState } from 'react';
import { get_forest, update_forest } from '../commands/forest_commands';
import './App.css';
import Map from './Map';

const DEFAULT_SEED: number = 123123;
const DEFAULT_WIDTH: number = 120;
const DEFAULT_HEIGHT: number = 80;

export default function App(props: {}): ReactElement<any, any> {
  const [map, setMap] = useState<number[]>([]);
  const [width, setWidth] = useState<number>(DEFAULT_WIDTH);
  const [height, setHeight] = useState<number>(DEFAULT_HEIGHT);

  const handleRedrawMap = (event: any): void => {
    get_forest().then(map => {
      if (!map) return;
      setMap(map);
    });
  }

  const handleUpdateMap = (event: any): void => {
    update_forest().then(_ => get_forest().then(map => {
      if (!map) return;
      setMap(map);
    }));
  }

  return (
    <div id="App_container">
      <div id="App_canvas">
        <Map map={map} width={width} height={height} />
      </div>

      <div id="App_buttons">
        <button onClick={handleRedrawMap}>Get the map</button>
        <button onClick={handleUpdateMap}>Update the map</button>
      </div>
    </div>
  );
}
