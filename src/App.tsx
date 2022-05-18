import { useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { DefaultApi, ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction } from 'pte-browser-extension-sdk'
import Notiflix from 'notiflix'
import { ADMINBADGE, PACKAGE, NAR, COMPONENT,  VALIDATOR_BADGE, USER_BADGE, STAKER_BADGE, VALIDATOR_ADDRESS} from './NEURACLE'


function App() {
  const [accountAddress, setAccountAddress] = useState<string>()
  const lightgreen = { color: 'lightgreen' }
  const lightblue = { color: 'lightblue' }
  const [neura, setNeura] = useState<string>()
  const [yourRole, setYourRole] = useState<string>()
  const [memberInfo, setMemberInfo] = useState<Array<string>>()
  const url = 'https://pte01.radixdlt.com/'
  const [showInfo, setShowInfo] = useState<string>()

  async function get_nft_data(nft, resource) {
    

    const nonFungibleId = nft.non_fungible_ids[0];
    
          const response = await fetch(
          `${url}non-fungible/${resource}${nonFungibleId}`
          );

          window.prompt("here");
          
          let info: Array<string> = [];
          const nonFungibleData = await response.json();
          
          const data = JSON.parse(nonFungibleData.immutable_data).fields;

          data.foreach( (x) => {
            info.push(x.value)
          });

          const data2 = JSON.parse(nonFungibleData.mutable_data).fields
          data2.foreach( (x) => {
            info.push(x.value)
          });

          setMemberInfo(info);

  }

  async function data() {

    const fetchData = async () => {
      try {
        
        setAccountAddress(await getAccountAddress());

        const response = await fetch(`${url}component/${accountAddress}`);
  
        const component = await response.json();
  
        const my_resource = component.owned_resources;

        if (my_resource.find((resource) => resource.resource_address === ADMINBADGE)) {
          setYourRole("NeuRacle Admin")
        } else if (my_resource.find((resource) => resource.resource_address === VALIDATOR_BADGE)) {
          setYourRole("NeuRacle Validator");
          
          await get_nft_data(my_resource, VALIDATOR_BADGE);
          
          setShowInfo(`Name: "${memberInfo![0]}" | Country: "${memberInfo![1]}" | Website: "${memberInfo![2]}" | Validator Address: "${memberInfo![3]}"`);
          
        } else if (my_resource.find((resource) => resource.resource_address === USER_BADGE)) {
          setYourRole("NeuRacle User")
          await get_nft_data(my_resource, USER_BADGE);
          setShowInfo(`Your data source: "${memberInfo![0]}" | Account can use until epoch: "${memberInfo![1]}"`)

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

    alert("You must edit the NEURACLE.tsx file with new package: " + receipt.newPackages[0])
  }
  async function become_admin() {

    const manifest = new ManifestBuilder()
      .callFunction(PACKAGE, 'NeuRacle', 'new', ['100u32', '1u64', 'Decimal("1")', 'Decimal("0.3")', '500u64', 'Decimal("0.0015")', 'Decimal("10")'])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString();
  
    const receipt = await signTransaction(manifest);
  
    if (receipt.status == 'Success') {
      alert("You have become NeuRacle Admin, please check your wallet detail in Pouch. You must edit the NEURACLE.tsx file with new component: " + receipt.newComponents[0]
       + ". New Admin Badge: " + receipt.newResources[0] 
       + ". New Validator Badge: " + receipt.newResources[3] 
       + ". New User Badge: " + receipt.newResources[4] 
       + ". New Neura Resource Address: " + receipt.newResources[5]
       + ". Please don't close this window until done!")
    } else {
      alert(receipt.status)
    }
  }


  async function assign_validators() {

    if (yourRole == "NeuRacle Admin") {

      const result = prompt("Validator Account Address");
      if (result == null) {
        return
      } else {
        const validator_account_address: string = result;
        const result2 = prompt("Validator Name");
        if (result2 == null) {
          return
        } else {
          const validator_name: string = result2;
          const result3 = prompt("Validator Country");
          if (result3 == null) {
            return
          } else {
            const validator_country: string = result3;
            const result4 = prompt("Validator Website");
            if (result4 == null) {
              return
            } else {
              const validator_website: string = result4;
              const result5 = prompt("Validator Fee");
              if (result5 == null) {
                return
              } else {
                const validator_fee: string = result5;
                const manifest = new ManifestBuilder()
                  .withdrawFromAccountByAmount(accountAddress!, 1, ADMINBADGE)
                  .takeFromWorktop( ADMINBADGE , 'bucket')
                  .createProofFromBucket('bucket', 'admin_proof')
                  .pushToAuthZone('admin_proof')
                  .callMethod( COMPONENT,  'create_new_validator_node', [`"${validator_name}" "${validator_country}" "${validator_website}" Decimal("${validator_fee}")`])
                  .takeFromWorktopByAmount(1, VALIDATOR_BADGE, 'val1')
                  .callMethod(validator_account_address, 'deposit', ['Bucket("val1")'])
                  .callMethodWithAllResources(accountAddress!, 'deposit_batch')
                  .build()
                  .toString();
        
              const receipt = await signTransaction(manifest);
          
              if (receipt.status == 'Success') {
                alert("The address you provided has been assigned as NeuRacle Validator. New Validator Address: "
                + receipt.newComponents[0] 
                + ". This validator staker badge: "
                + receipt.newResources[1]
                + ". Please don't close this window until done!")
                    } else {
                alert(receipt.status);
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

  useEffect(() => {
    setTimeout(() => {
      data();
    }, 100);
  }, [accountAddress, yourRole, memberInfo, showInfo]);

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
        <button type="button" onClick={data}>
            Refresh your data
          </button>
        </p>
        <p></p>
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
