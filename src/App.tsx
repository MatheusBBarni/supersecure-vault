import { rspc } from './config/rspc'


function App() {
  const version = rspc.createQuery(() => ['version'])

  console.log(version.error)

  return (
    <div>
      <h1>Welcome to Taurii!</h1>
      <p>{version.data}</p>
      <p>{version.isLoading}</p>
    </div>
  )
}

export default App
