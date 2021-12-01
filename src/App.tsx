import React, {useEffect, useState} from 'react';
import './App.css';
import one_a from './input/one_a.json';

function App() {
    const [msg, setMsg] = useState<string>();
    useEffect(() => {
        import('./rust/pkg').then((module) => {
            let res = module.one_a(one_a.raw);
            alert(res);
            setMsg(res);
        }).catch(() => {
            setMsg("Error!!");
        })
    }, [])
  return (
    <div className="App">
      <header className="App-header">
          {msg ?? null}
      </header>
    </div>
  );
}

export default App;
