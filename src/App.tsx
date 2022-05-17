import { useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { DefaultApi, ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk'
import Notiflix from 'notiflix'


function App() {
  const [accountAddress, setAccountAddress] = useState<string>()
  const lightgreen = { color: 'lightgreen' }
  const lightblue = { color: 'lightblue' }
  const [packageAddress, setPackageAddress] = useState<string>()
  const [componentAddress, setComponentAddress] = useState<string>()
  const [adminBadge, setadminBadge] = useState<string>()
  const [validatorBadge, setvalidatorBadge] = useState<string>()
  const [userBadge, setuserBadge] = useState<string>()
  const [neura, setNeura] = useState<string>()
  const [validatorAddress, setValidatorAddress] = useState<string>()
  const [stakerBadge, setStakerBadge] = useState<string>()
  const [yourRole, setYourRole] = useState<string>()

  async function data() {

    const url = `https://pte01.radixdlt.com/component/${accountAddress}`;

    const fetchData = async () => {
      try {
        
        setAccountAddress(await getAccountAddress());

        const response = await fetch(url);
  
        const component = await response.json();
  
        const my_resource = component.owned_resources;
        if (my_resource.find((resource) => resource.resource_address === adminBadge)) {
          setYourRole("NeuRacle Admin")
        } else if (my_resource.find((resource) => resource.resource_address === validatorBadge)) {
          setYourRole("NeuRacle Validator")
        } else if (my_resource.find((resource) => resource.resource_address === userBadge)) {
          setYourRole("NeuRacle User")
        } else {
          setYourRole("Visitor")
        }

      } catch (error) {
        fetchData()
      }
    };

    Notiflix.Loading.pulse();
    fetchData();
    Notiflix.Loading.remove()

  }

  async function publish_package() {

    const response = await fetch('./neu_racle.wasm');
    const wasm = new Uint8Array(await response.arrayBuffer());
  
    const manifest = new ManifestBuilder()
      .publishPackage(wasm)
      .build()
      .toString();
  
    const receipt = await signTransaction(manifest);
  
    setPackageAddress(receipt.newPackages[0]);
    success(receipt.status)
  }
  async function become_admin() {

    const manifest = new ManifestBuilder()
      .callFunction(packageAddress!, 'NeuRacle', 'new', ['100u32', '1u64', 'Decimal("1")', 'Decimal("0.3")', '500u64', 'Decimal("0.0015")', 'Decimal("10")'])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString();
  
    const receipt = await signTransaction(manifest);
  
    if (receipt.status == 'Success') {
      setComponentAddress(receipt.newComponents[0]);
      setadminBadge(receipt.newResources[0]);
      setvalidatorBadge(receipt.newResources[3]);
      setuserBadge(receipt.newResources[4]);
      setNeura(receipt.newResources[5]);
      success_big("Done", "You have become NeuRacle Admin, please check your wallet detail in Pouch")
    } else {
      failure_big("Failed", receipt.status)
    }
  }
  async function assign_validators() {

    if (yourRole == "NeuRacle Admin") {
      const validator_account_address = await get_detail("Validator Info", "Account Address");
    if (validator_account_address == '') return
    else {
      const validator_name = await get_detail("Validator Info", "Name");
      if (validator_name == '') return
      else {
        const validator_country = await get_detail("Validator Info", "Country");
        if (validator_country == '') return
        else {
          const validator_website = await get_detail("Validator Info", "Website");
          if (validator_website == '') return
          else {
            const validator_fee = await get_detail("Validator Info", "Staking fee");
            if (validator_fee == '') return
            else {
              const manifest = new ManifestBuilder()
              .callMethod(accountAddress!, 'withdraw_by_amount', ['Decimal("1")', 'ResourceAddress(' + adminBadge + ')'])
              .takeFromWorktop('ResourceAddress(' + adminBadge + ')', 'Bucket("bucket")')
              .createProofFromBucket('Bucket("bucket")', 'Proof("admin_proof")')
              .pushToAuthZone('Proof("admin_proof")')
              .callMethod(componentAddress!, "create_new_validator_node", [validator_name, validator_country, validator_website, validator_fee])
              .takeFromWorktopByAmount(1, 'ResourceAddress(' + validatorBadge + ')', 'Bucket("val1")')
              .callMethod('ComponentAddress(' + validator_account_address + ')', 'deposit', ['Bucket("val1")'])
              .callMethodWithAllResources(accountAddress!, 'deposit_batch')
              .build()
              .toString();
        
            const receipt = await signTransaction(manifest);
          
            // Update UI
            if (receipt.status == 'Success') {
              setValidatorAddress(receipt.newComponents[0]);
              setStakerBadge(receipt.newResources[1]);
              success_big("Done", "The address you provided has been assigned as NeuRacle Validator")
            } else {
              failure_big("Failed", receipt.status);
            }
            }
          }
        }
      }
    }
    }
    else {
      failure_big("Failed", "You are not NeuRacle Admin")
    }
  }
  
  function success(message: string) {Notiflix.Notify.success(message,{
    position: 'right-top',
    borderRadius: '10px',
    showOnlyTheLastOne: true
  })}

  function success_big(title: string, message: string) {Notiflix.Report.success(
    title,
    message,
    'Ok',
    )
  }

  function failure(message: string) {Notiflix.Notify.failure(message,{
    position: 'right-top',
    borderRadius: '10px',
    showOnlyTheLastOne: true
  })}

  function failure_big(title: string, message: string) {Notiflix.Report.failure(
    title,
    message,
    'Ok',
    )
  }

  async function get_detail(head: string, info: string): Promise<string> {
    var answer = '';
    Notiflix.Confirm.prompt(
    head,
    info,
    '',
    'OK',
    'Cancel',
    function okCb(clientAnswer) {
    success(info + ': ' + clientAnswer);
    answer = clientAnswer;
    },
    function cancelCb() {
      answer = '';
      },
    );
    return answer
  }

  useEffect(() => {
    setTimeout(() => {
      data();
    }, 100);
  }, [accountAddress, yourRole]);

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
          Hello <a style={lightblue}>{yourRole}</a> with account: "<a style={lightgreen}>{accountAddress}</a>"
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
          </button> | <button type="button" onClick={assign_validators}>
            Assign a validator
          </button>
        <p>
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
