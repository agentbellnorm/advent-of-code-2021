import React from "react";
import useWasm, { WasmModule } from "../hooks/useWasm";
import styles from "./Day.module.scss";

type Props = {
  runA: (module: WasmModule) => string;
  runB: (module: WasmModule) => string;
};

const Day = ({ runA, runB }: Props) => {
  const module = useWasm();

  if (!module) {
    return null;
  }

  let timeA = Date.now();
  const resultA = runA(module);
  let durA = Date.now() - timeA;

  let timeB = Date.now();
  const resultB = runB(module);
  let durB = Date.now() - timeB;

  return (
    <div className={styles.container}>
      <h2>A</h2>
      <p>{resultA}</p>
      <p>{durA} ms</p>
      <h2>B</h2>
      <p>{resultB}</p>
      <p>{durB} ms</p>
    </div>
  );
};

export default Day;
