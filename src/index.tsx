import './styles.css'

/* @refresh reload */
import { QueryClient } from '@tanstack/solid-query'
import { render } from 'solid-js/web'

import App from './App'
import { client, RspcProvider } from './config/rspc'

const queryClient = new QueryClient()

render(() => (
  <RspcProvider client={client} queryClient={queryClient}>
    <App />
  </RspcProvider>
), document.getElementById('root') as HTMLElement)
