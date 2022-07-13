import { useLayoutEffect } from 'react'
import './App.css'
import init from '../out/practice';

function App() {
  useLayoutEffect(() => {
    init();
  }, []);

  return (
    <div className="App">
      
    </div>
  )
}

export default App
