import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import "./App.css";
import Day from "./component/Day";
import one_a from "./input/one_a.json";
import two_a from "./input/two_a.json";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <BrowserRouter>
          <Routes>
            <Route
              path="01"
              element={
                <Day
                  runA={(module) => module.one_a(one_a.raw)}
                  runB={(module) => module.one_b(one_a.raw)}
                />
              }
            />
            <Route
              path="02"
              element={
                <Day
                  runA={(module) => module.two_a(two_a.raw)}
                  runB={(module) => module.two_b(two_a.raw)}
                />
              }
            />
            <Route
              path="*"
              element={
                <main style={{ padding: "1rem" }}>
                  <p>There's nothing here!</p>
                </main>
              }
            />
          </Routes>
        </BrowserRouter>
      </header>
    </div>
  );
}

export default App;
