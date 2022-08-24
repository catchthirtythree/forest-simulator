import { info } from 'console';
import { ReactElement, useEffect, useState } from 'react';
import { create_forest, get_forest, update_forest } from '../commands/forest_commands';
import { IForestInfo } from '../types/response';
import './App.css';
import Map from './Map';

const DEFAULT_SEED: number = 123123;
const DEFAULT_WIDTH: number = 120;
const DEFAULT_HEIGHT: number = 80;
const DEFAULT_CELL_DRAW_SIZE: number = 8;

export function get_bear_amount(forestInfo: IForestInfo): number {
  return forestInfo.map.reduce((acc, cell) => {
    if ((cell & 0xf000) > 0) {
      return acc + 1;
    }

    return acc;
  }, 0);
}

export function get_jack_amount(forestInfo: IForestInfo): number {
  return forestInfo.map.reduce((acc, cell) => {
    if ((cell & 0x0f00) > 0) {
      return acc + 1;
    }

    return acc;
  }, 0);
}

export function get_tree_amount(forestInfo: IForestInfo): number {
  return forestInfo.map.reduce((acc, cell) => {
    if ((cell & 0x00ff) > 0) {
      return acc + 1;
    }

    return acc;
  }, 0);
}

export function show_formatted_date(forestInfo: IForestInfo): string {
  const years = Math.floor(forestInfo.months_elapsed / 12);
  const months = Math.floor(forestInfo.months_elapsed % 12);
  return `Year ${years}, Month ${months}`;
}

export default function App(props: {}): ReactElement<any, any> {
  const [timer, setTimer] = useState<NodeJS.Timer | null>(null);

  const [forestInfo, setForestInfo] = useState<IForestInfo>({
    map: [],
    width: DEFAULT_WIDTH,
    height: DEFAULT_HEIGHT,
    months_elapsed: 0,
    yearly_lumber: 0,
    yearly_mauls: 0,
  });
  const [seed, setSeed] = useState<number>(DEFAULT_SEED);

  const [seedInput, setSeedInput] = useState<string>(DEFAULT_SEED.toString());
  const [widthInput, setWidthInput] = useState<string>(DEFAULT_WIDTH.toString());
  const [heightInput, setHeightInput] = useState<string>(DEFAULT_HEIGHT.toString());
  const [cellInput, setCellInput] = useState<string>(DEFAULT_CELL_DRAW_SIZE.toString());

  const getForest = () => {
    get_forest().then(info => setForestInfo(info));
  }

  const handleUpdateSettings = (event: any): void => {
    const seed = Number(seedInput);
    const width = Number(widthInput);
    const height = Number(heightInput);

    create_forest({
      seed: seed,
      width: width,
      height: height,
    }).then(info => {
      if (!info) return;
      setForestInfo(info);
      setSeed(seed);
    });
  }

  const handleRunSimulation = (event: any): void => {
    if (timer === null) {
      update_forest().then(_ => getForest());
      setTimer(setInterval(() => update_forest().then(_ => getForest()), 1000));
    } else {
      clearInterval(timer);
      setTimer(null);
    }
  }

  const handleUpdateMap = (event: any): void => {
    update_forest().then(_ => getForest());
  }

  useEffect(() => {
    getForest();
  }, [props]);

  return (
    <div id="App_container">
      <div id="App_canvas">
        <Map info={forestInfo} cellSize={Number(cellInput)} />
      </div>

      <div id="App_information">
        <div id="App_settings">
          <div id="settings-header">Simulation Settings</div>

          <div className="settings-field">
            <span>Seed:</span>
            <input type="number" value={seedInput}
              onChange={(event) => {
                setSeedInput(event.target.value);
              }}
            />
          </div>

          <div className="settings-field">
            <span>Width:</span>
            <input type="number" value={widthInput}
              onChange={(event) => {
                setWidthInput(event.target.value);
              }} />
          </div>

          <div className="settings-field">
            <span>Height:</span>
            <input type="number" value={heightInput}
              onChange={(event) => {
                setHeightInput(event.target.value);
              }} />
          </div>

          <div className="settings-field">
            <span>Cell Size:</span>
            <input type="range" min="1" max="64" value={cellInput}
              onChange={(event) => {
                setCellInput(event.target.value);
              }}
            />
          </div>

          <button onClick={handleUpdateSettings}>Update Settings</button>
        </div>

        <hr />

        <div id="App_statistics">
          <div id="stats-header">Simulation Statistics</div>

          <div>{show_formatted_date(forestInfo)}</div>

          <div className="stats-field">
            <div>Bears:</div>
            <div>{get_bear_amount(forestInfo)}</div>
          </div>

          <div className="stats-field">
            <div>Jacks:</div>
            <div>{get_jack_amount(forestInfo)}</div>
          </div>

          <div className="stats-field">
            <div>Trees:</div>
            <div>{get_tree_amount(forestInfo)}</div>
          </div>

          <div className="stats-field">
            <div>Yearly Lumber:</div>
            <div>{forestInfo.yearly_lumber}</div>
          </div>

          <div className="stats-field">
            <div>Yearly Mauls:</div>
            <div>{forestInfo.yearly_mauls}</div>
          </div>
        </div>

        <hr />

        <div id="App_buttons">
          <button onClick={handleRunSimulation}>
            {timer === null ? 'Start Simulation' : 'Stop Simulation'}
          </button>

          <button onClick={handleUpdateMap} disabled={timer !== null}>
            Update
          </button>
        </div>
      </div>
    </div>
  );
}
