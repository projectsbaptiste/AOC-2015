import logo from '../logo.svg';
import '../App.css';
import Banner from './Banner'
import Todo from './Todo'

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/components/App.js</code> and save to reload.
        </p>
        <div><Banner/></div>
        <div><Todo/></div>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
