import { For, createEffect, createSignal, onCleanup, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  ChartOptions,
} from "chart.js";
import { Line } from "solid-chartjs";

import "./App.css";

export const options: ChartOptions<"line"> = {
  responsive: true,
  maintainAspectRatio: false,
  animation: false,
  plugins: {
    legend: {
      position: "top" as const,
    },
    title: {
      display: true,
      text: "Chart.js Line Chart",
    },
  },
};

type PqData = Readonly<{
  p: number;
  q: number;
  min_seq_id: number;
  max_seq_id: number;
  min_color: number;
  max_color: number;
}>;

// type PqPair = Readonly<PqData>;
type PqTriple = Readonly<[number, number, number]>;

type StreamState = {
  bids: ReadonlyArray<PqData>;
  asks: ReadonlyArray<PqData>;
  depthStream: Readonly<
    Record<string, [ReadonlyArray<PqData>, ReadonlyArray<PqData>]>
  >;
  trades: ReadonlyArray<PqTriple>;
};

function App() {
  onMount(() => {
    ChartJS.register(
      CategoryScale,
      LinearScale,
      PointElement,
      LineElement,
      Title,
      Tooltip,
      Legend
    );
  });

  const [state, setState] = createSignal<Readonly<StreamState>>({
    bids: [],
    asks: [],
    depthStream: {},
    trades: [],
  });
  const [states, setStates] = createSignal<ReadonlyArray<StreamState>>([]);
  const [stateIdx, setStateIdx] = createSignal<number>(0);
  const [live, setLive] = createSignal<boolean>(true);
  const [intervalMs, setIntervalMs] = createSignal<Readonly<number>>(250);
  const [count, setCount] = createSignal<Readonly<number>>(0);
  const [pause, setPause] = createSignal<Readonly<boolean>>(false);

  const timer = setInterval(async () => {
    setCount(count() + 1);
  }, 1);
  createEffect(async () => {
    if (!pause() && count() > intervalMs()) {
      const newMsg: Readonly<StreamState> = await invoke("get_latest_tickers", {
        depth: 10,
      });
      setStates((arr) => [...arr, newMsg]);
      setCount(0);
    }
  });
  createEffect(() => {
    if (stateIdx() < states().length) {
      setState(states()[stateIdx()]);
    }
  });
  createEffect(() => {
    if (live()) {
      setStateIdx(Math.max(0, states().length - 1));
    }
  });
  onCleanup(() => clearInterval(timer));

  return (
    <div>
      <div
        style={{
          display: "flex",
          "flex-direction": "row",
          "align-items": "center",
          "column-gap": "5px",
        }}
      >
        <input
          type="number"
          value={intervalMs()}
          onChange={(v) => {
            setIntervalMs(Number(v.target.value));
          }}
        ></input>
        <button onClick={() => setPause((s) => !s)}>
          {pause() ? `Unpause` : `Pause`}
        </button>
        <div>{`counter: ${count()}`}</div>
      </div>
      <div
        style={{
          display: "flex",
          "flex-direction": "row",
          "align-items": "center",
          "column-gap": "5px",
        }}
      >
        <button
          type="button"
          onClick={() => {
            setStateIdx((v) => Math.max(0, v - 1));
          }}
        >
          Previous
        </button>
        <input
          type="range"
          id="state-idx-slider"
          name="state-idx-slider"
          min={0}
          max={states().length}
          onChange={(v) => setStateIdx(Number(v.target.value))}
        />
        <button
          type="button"
          onClick={() => {
            setStateIdx((v) => Math.min(v + 1, states().length));
          }}
        >
          Next
        </button>
        <label>{`Index: ${stateIdx()}`}</label>
        <div>
          <input
            type="checkbox"
            id="state-live"
            name="state-live"
            checked={live()}
            onClick={() => setLive((b) => !b)}
          />
          <label>Live</label>
        </div>
      </div>
      <div
        style={{
          display: "flex",
          "flex-direction": "row",
          "column-gap": "3px",
        }}
      >
        <div>
          <CombinedDepthStream
            count={count()}
            streamName="Combined Stream"
            bids={state().bids}
            asks={state().asks}
          />
        </div>
        {Object.entries(state().depthStream).map(
          ([streamName, [bids, asks]]) => {
            return (
              <div>
                <DepthStream
                  count={count()}
                  streamName={streamName}
                  bids={bids}
                  asks={asks}
                />
              </div>
            );
          }
        )}
        <TradeStream
          count={count()}
          stateIdx={stateIdx()}
          trades={state().trades}
        />
      </div>
    </div>
  );
}

type DepthStreamProps = {
  count: number;
  streamName: string;
  bids: ReadonlyArray<PqData>;
  asks: ReadonlyArray<PqData>;
};

const CombinedDepthStream = (props: DepthStreamProps) => {
  return (
    <DepthStream
      count={props.count}
      streamName={`Combined depth`}
      bids={props.bids}
      asks={props.asks}
    />
  );
};

const DepthStream = (props: DepthStreamProps) => {
  return (
    <div>
      <div>{props.streamName}</div>
      <div
        style={{
          display: "flex",
          "flex-direction": "column",
          padding: "5px",
          border: "1px solid black",
          "row-gap": "5px",
        }}
      >
        <div>
          <div>asks</div>
          <div
            style={{
              display: "flex",
              "flex-direction": "column",
              padding: "5px",
              border: "1px solid black",
              "row-gap": "2px",
            }}
          >
            <For each={props.asks}>
              {({ p, q, min_seq_id, max_seq_id, min_color, max_color }) => {
                return (
                  <div
                    style={{
                      display: "flex",
                      "flex-direction": "row",
                      padding: "1px",
                      border: "1px solid black",
                      "column-gap": "3px",
                      "justify-content": "space-between",
                      width: "300px",
                      background: `linear-gradient(to right, pink ${min_color}% ${max_color}%, white ${max_color}%)`,
                    }}
                  >
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {p}
                    </div>
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {q}
                    </div>
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {min_seq_id}
                    </div>
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {max_seq_id}
                    </div>
                  </div>
                );
              }}
            </For>
          </div>
        </div>
        <div>
          <div>bids</div>
          <div
            style={{
              display: "flex",
              "flex-direction": "column",
              padding: "5px",
              border: "1px solid black",
              "row-gap": "2px",
            }}
          >
            <For each={props.bids}>
              {({ p, q, min_seq_id, max_seq_id, min_color, max_color }) => {
                return (
                  <div
                    style={{
                      display: "flex",
                      "flex-direction": "row",
                      padding: "1px",
                      border: "1px solid black",
                      "column-gap": "3px",
                      "justify-content": "space-between",
                      width: "300px",
                      background: `linear-gradient(to right, pink ${min_color}% ${max_color}%, white ${max_color}%)`,
                    }}
                  >
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {p}
                    </div>
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {q}
                    </div>
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {min_seq_id}
                    </div>
                    <div
                      style={{
                        "font-size": "12px",
                        color: "black",
                      }}
                    >
                      {max_seq_id}
                    </div>
                  </div>
                );
              }}
            </For>
          </div>
        </div>
      </div>
    </div>
  );
};

type TradeStreamProps = {
  stateIdx: number;
  count: number;
  trades: ReadonlyArray<PqTriple>;
};

const TradeStream = (props: TradeStreamProps) => {
  return (
    <div>
      <div>Trade Prices</div>
      <div
        style={{
          width: "800px",
          height: "800px",
        }}
      >
        <Line
          datasetIdKey={`trades-${props.stateIdx}-${props.count}`}
          options={options}
          data={{
            datasets: [
              {
                label: `Trade Price`,
                data: props.trades.map(([ts, p, _]) => ({
                  x: new Date(ts).toISOString(),
                  y: p.toString(),
                })),
                borderColor: "rgba(255, 0, 0, 1)",
              },
            ],
          }}
          fallback={fallback}
        />
      </div>
    </div>
  );
};

const fallback = () => {
  return (
    <div>
      <p>Chart is not available</p>
    </div>
  );
};

export default App;
