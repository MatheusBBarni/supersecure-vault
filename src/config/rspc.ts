import { createClient } from '@rspc/client'
import { createSolidQueryHooks } from '@rspc/solid'
import { TauriTransport } from '@rspc/tauri'

// These were the bindings exported from your Rust code!
import type { Procedures } from '../types/rspc'

// You must provide the generated types as a generic and create a transport (in this example we are using HTTP Fetch) so that the client knows how to communicate with your API.
export const client = createClient<Procedures>({
  // Refer to the integration your using for the correct transport.
  transport: new TauriTransport(),
})

export const rspc = createSolidQueryHooks<Procedures>()

export const RspcProvider = rspc.Provider