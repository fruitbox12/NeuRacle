import { useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { DefaultApi, ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk'

function App() {
  var [accountAddress, setAccountAddress] = useState<string>()
  const get_account = async function () {setAccountAddress((accountAddress) = await getAccountAddress())}
  const green = { color: 'green' }
  var [packageAddress, setPackageAddress] = useState<string>()
  const publish_package = async function () {
    // Load the wasm
    const response = await fetch('./neu_racle.wasm');
    const wasm = new Uint8Array(await response.arrayBuffer());
  
    // Construct manifest
    const manifest = new ManifestBuilder()
      .publishPackage(wasm)
      .build()
      .toString();
  
    const receipt = await signTransaction(manifest);
  
    setPackageAddress((packageAddress) = receipt.newPackages[0]);
  }
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Welcome to NeuRacle!</p>
        <p>
          Install <a
            className="App-link"
            href="https://docs.radixdlt.com/main/scrypto/public-test-environment/pte-getting-started.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Radix Babylon PTE
          </a> PTE to getting started.
        </p>
        <p>
        <button type="button" onClick={ get_account }>
            Click here get account
          </button> 
        </p>
        <p>
          Your account: "<a style={green}>{accountAddress}</a>"
        </p>
        <p>
        <button type="button" onClick={publish_package}>
            Publish package
          </button> 
        <p>
          Package Address: "<a style={green}>{packageAddress}</a>
        </p>
        </p>
      </header>
    </div>

  )
}

export default App
