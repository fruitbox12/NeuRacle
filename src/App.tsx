import { useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { DefaultApi, ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk'
import { ToastContainer, toast } from 'react-toastify'
import 'react-toastify/dist/ReactToastify.css'

function App() {
  var [accountAddress, setAccountAddress] = useState<string>()
  const yellow = { color: 'yellow' }
  const green = { color: 'green' }
  var [packageAddress, setPackageAddress] = useState<string>()
  var [componentAddress, setComponentAddress] = useState<string>()
  var [adminBadge, setadminBadge] = useState<string>()
  var [validatorBadge, setvalidatorBadge] = useState<string>()
  var [userBadge, setuserBadge] = useState<string>()
  var [neura, setNeura] = useState<string>()
  var [status, setStatus] = useState<string>()
  var [validatorName, setValidatorName] = useState<string>()

  const get_account = async function () {
    setAccountAddress((accountAddress) = await getAccountAddress());
  }
  const publish_package = async function () {

    const response = await fetch('./neu_racle.wasm');
    const wasm = new Uint8Array(await response.arrayBuffer());
  
    const manifest = new ManifestBuilder()
      .publishPackage(wasm)
      .build()
      .toString();
  
    const receipt = await signTransaction(manifest);
  
    setPackageAddress((packageAddress) = receipt.newPackages[0]);
    setStatus((status) = receipt.status)
  }
  const become_admin = async function () {

    const manifest = new ManifestBuilder()
      .callFunction(packageAddress!, 'NeuRacle', 'new', ['100u32', '1u64', 'Decimal("1")', 'Decimal("0.3")', '500u64', 'Decimal("0.0015")', 'Decimal("10")'])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString();
  
    const receipt = await signTransaction(manifest);
  
    if (receipt.status == 'Success') {
      setComponentAddress((componentAddress) = receipt.newComponents[0]);
      setadminBadge((adminBadge) = receipt.newResources[0]);
      setvalidatorBadge((validatorBadge) = receipt.newResources[3]);
      setuserBadge((userBadge) = receipt.newResources[4]);
      setNeura((neura) = receipt.newResources[5]);
      setStatus((status) = receipt.status)
    } else {
      setStatus((status) = receipt.status);
    }
  }
  const assign_validators = async function () {

    const manifest = new ManifestBuilder()
      .callMethod(accountAddress!, 'withdraw_by_amount', ['Decimal("1")', 'ResourceAddress(' + adminBadge + ')'])
      .takeFromWorktop('ResourceAddress(' + adminBadge + ')', 'Bucket("bucket")')
      .createProofFromBucket('Bucket("bucket")', 'Proof("admin_proof")')
      .pushToAuthZone('Proof("admin_proof")')
      .callMethod(componentAddress!, "create_new_validator_node", ["val1", "VietNam", "val1.vn", 'Decimal("0")'])
      .takeFromWorktopByAmount(1, 'ResourceAddress("${VALIDATOR_BADGE}")', 'Bucket("val1")')
      .callMethod('ComponentAddress("${VAL1_ACC}")', 'deposit', ['Bucket("val1")'])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString();

    const receipt = await signTransaction(manifest);
  
    // Update UI
    if (receipt.status == 'Success') {
      setComponentAddress((componentAddress) = receipt.newComponents[0]);
      setadminBadge((adminBadge) = receipt.newResources[0]);
      setvalidatorBadge((validatorBadge) = receipt.newResources[3]);
      setuserBadge((userBadge) = receipt.newResources[4]);
      setNeura((neura) = receipt.newResources[5]);
      setStatus((status) = receipt.status)
    } else {
      setStatus((status) = receipt.status);
    }
  }
  get_account()
  
  const notify = (message: String) => toast(message,{
    className: 'black-background',
    bodyClassName: "grow-font-size",
    progressClassName: 'fancy-progress-bar',
    position: "top-left",
    autoClose: 5000,
    hideProgressBar: false,
    closeOnClick: true,
    pauseOnHover: true,
    draggable: true,
    progress: undefined,
    style:{ backgroundColor: "red" }
    })
  get_account()
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
          Hello account: "<a style={green}>{accountAddress}</a>"
        </p>
        <button onClick={() => notify("Done right!")}>Success !</button>
        <ToastContainer />
        <p>
        </p>
        <p>
        Check your balance through <a
            className="App-link"
            href="https://plymth.github.io/pouch/"
            target="_blank"
            rel="noopener noreferrer"
          >Pouch</a>
        </p>
        <p>
        <button type="button" onClick={publish_package}>
            Publish package
          </button> | <button type="button" onClick={become_admin}>
            Become NeuRacle Admin
          </button> |  
        <p>
          Transaction Status: "<a style={yellow}>{status}</a>"
        </p>
        <p>
        </p>
        <p>
        </p>
        </p>
      </header>
    </div>
  )
}

export default App
