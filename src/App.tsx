import { useEffect, useState } from "react";
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
import "./App.css";
import { Line } from "react-chartjs-2";

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
};

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

const getLatestInfo = async (): Promise<Readonly<StreamState>> =>
  invoke("get_latest_info", {
    depth: 10,
  });

const getLatestTrade = async (): Promise<ReadonlyArray<PqTriple>> =>
  invoke("get_latest_trade", {});

function App() {
  const [state, setState] = useState<Readonly<StreamState>>({
    bids: [],
    asks: [],
    depthStream: {},
  });
  const [trades, setTrades] = useState<ReadonlyArray<PqTriple>>([]);
  const [intervalMs, setIntervalMs] = useState<Readonly<number>>(50);
  const [count, setCount] = useState<Readonly<number>>(0);

  useEffect(() => {
    const interval = setInterval(async () => {
      setCount(count + 1);
      if (count >= intervalMs) {
        setCount(0);
        getLatestInfo().then((state) => {
          setState(state);
        });
        getLatestTrade().then((trades) => {
          setTrades((t) => [...t, ...trades]);
        });
      }
    }, 1);
    return () => clearInterval(interval);
  });

  return (
    <div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          columnGap: "5px",
        }}
      >
        <input
          type="number"
          value={intervalMs}
          onChange={(v) => {
            setIntervalMs(Number(v.target.value));
          }}
        ></input>
        <div>{`counter: ${count}`}</div>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          columnGap: "3px",
        }}
      >
        <div>
          <CombinedDepthStream
            count={count}
            streamName="Combined Stream"
            bids={state.bids}
            asks={state.asks}
          />
        </div>
        {Object.entries(state.depthStream).map(([streamName, [bids, asks]]) => {
          return (
            <div>
              <DepthStream
                count={count}
                streamName={streamName}
                bids={bids}
                asks={asks}
              />
            </div>
          );
        })}
        <TradeStream count={count} trades={trades} />
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
          flexDirection: "column",
          padding: "5px",
          border: "1px solid black",
          rowGap: "5px",
        }}
      >
        <div>
          <div>asks</div>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              padding: "5px",
              border: "1px solid black",
              rowGap: "2px",
            }}
          >
            {props.asks.map(
              ({ p, q, min_seq_id, max_seq_id, min_color, max_color }) => {
                return (
                  <div
                    style={{
                      display: "flex",
                      flexDirection: "row",
                      padding: "1px",
                      border: "1px solid black",
                      columnGap: "3px",
                      justifyContent: "space-between",
                      width: "300px",
                      background: `linear-gradient(to right, pink ${min_color}% ${max_color}%, white ${max_color}%)`,
                    }}
                  >
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {p}
                    </div>
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {q}
                    </div>
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {min_seq_id}
                    </div>
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {max_seq_id}
                    </div>
                  </div>
                );
              }
            )}
          </div>
        </div>
        <div>
          <div>bids</div>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              padding: "5px",
              border: "1px solid black",
              rowGap: "2px",
            }}
          >
            {props.bids.map(
              ({ p, q, min_seq_id, max_seq_id, min_color, max_color }) => {
                return (
                  <div
                    style={{
                      display: "flex",
                      flexDirection: "row",
                      padding: "1px",
                      border: "1px solid black",
                      columnGap: "3px",
                      justifyContent: "space-between",
                      width: "300px",
                      background: `linear-gradient(to right, pink ${min_color}% ${max_color}%, white ${max_color}%)`,
                    }}
                  >
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {p}
                    </div>
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {q}
                    </div>
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {min_seq_id}
                    </div>
                    <div
                      style={{
                        fontSize: "12px",
                        color: "black",
                      }}
                    >
                      {max_seq_id}
                    </div>
                  </div>
                );
              }
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

type TradeStreamProps = {
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
          datasetIdKey={`trades-${props.count}`}
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
        />
      </div>
    </div>
  );
};

export default App;
