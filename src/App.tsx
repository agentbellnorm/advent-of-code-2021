import React from "react";
import { HashRouter, Routes, Route, Link, Navigate } from "react-router-dom";
import "./App.scss";
import Day from "./component/Day";
import one_a from "./input/one_a.json";
import two_a from "./input/two_a.json";
import three from "./input/three.json";
import four from "./input/four.json";
import five from "./input/five.json";
import six from "./input/six.json";
import seven from "./input/seven.json";
import eight from "./input/eight.json";
import nine from "./input/nine.json";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <HashRouter>
          <div className="navLinks">
            <Link to="01">Day 1</Link>
            <Link to="02">Day 2</Link>
            <Link to="03">Day 3</Link>
            <Link to="04">Day 4</Link>
            <Link to="05">Day 5</Link>
            <Link to="06">Day 6</Link>
            <Link to="07">Day 7</Link>
            <Link to="08">Day 8</Link>
            <Link to="09">Day 8</Link>
          </div>
          <Routes>
            <Route path="/" element={<Navigate to="09" />} />
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
              path="03"
              element={
                <Day
                  runA={(module) => module.three_a(three.raw)}
                  runB={(module) => module.three_b(three.raw)}
                />
              }
            />
            <Route
              path="04"
              element={
                <Day
                  runA={(module) => module.four_a(four.raw)}
                  runB={(module) => module.four_b(four.raw)}
                />
              }
            />
            <Route
              path="05"
              element={
                <Day
                  runA={(module) => module.five_a(five.real)}
                  runB={(module) => module.five_b(five.real)}
                />
              }
            />
            <Route
              path="06"
              element={
                <Day
                  runA={(module) => module.six_a(six.real)}
                  runB={(module) => module.six_b(six.real)}
                />
              }
            />
            <Route
              path="07"
              element={
                <Day
                  runA={(module) => module.seven_a(seven.real)}
                  runB={(module) => module.seven_b(seven.real)}
                />
              }
            />
            <Route
              path="08"
              element={
                <Day
                  runA={(module) => module.eight_a(eight.real)}
                  runB={(module) => module.eight_b(eight.real)}
                />
              }
            />
            <Route
              path="09"
              element={
                <Day
                  runA={(module) => module.nine_a(nine.real)}
                  runB={(module) => module.nine_b(nine.real)}
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
        </HashRouter>
      </header>
    </div>
  );
}

export default App;
