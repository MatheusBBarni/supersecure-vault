import { rspc } from "./config/rspc";


function App() {
  const { data, isLoading, error } = rspc.createQuery(() => ["version"])

  console.log(error)

  return (
    <div>
      <h1>Welcome to Taurii!</h1>
      <p>{data}</p>
      <p>{isLoading}</p>
    </div>
  );
}

export default App;
