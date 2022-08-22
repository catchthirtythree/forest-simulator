import { ReactElement, useEffect, useState } from 'react';
import { create_forest, get_forest, update_forest } from '../commands/forest_commands';
import { IForestInfo } from '../types/response';
import './App.css';
import Map from './Map';

const DEFAULT_SEED: number = 123123;
const DEFAULT_WIDTH: number = 120;
const DEFAULT_HEIGHT: number = 80;

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

  const handleRedrawMap = (event: any): void => {
    getForest();
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
        <Map info={forestInfo} />
      </div>

      <div id="App_information">
        <div id="App_settings">
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

          <button onClick={handleUpdateSettings}>Update Settings</button>
        </div>

        {/* @TODO(michael): Show forest information: months elapsed, number of bears/jacks/trees. */}

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
