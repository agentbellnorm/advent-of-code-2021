import { useEffect, useState } from "react";

export type WasmModule = typeof import("../rust/pkg");

const useWasm = () => {
  const [module, setModule] = useState<WasmModule>();

  useEffect(() => {
    import("../rust/pkg")
      .then((m) => {
        m.hello();
        setModule(m);
      })
      .catch((e) => {
        console.error(`could not load wasm module: ${e}`);
      });
  }, []);

  return module;
};

export default useWasm;
