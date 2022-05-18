import { useCallback, useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { DefaultApi, ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction, waitForAction } from 'pte-browser-extension-sdk'
import Notiflix from 'notiflix'
import { ADMINBADGE, PACKAGE, NAR, COMPONENT,  VALIDATOR_BADGE, USER_BADGE, STAKER_BADGE, VALIDATOR_ADDRESS} from './NEURACLE'


function App() {
  const [accountAddress, setAccountAddress] = useState<string>()
  const lightgreen = { color: 'lightgreen' }
  const lightblue = { color: 'lightblue' }
  const [neura, setNeura] = useState<string>()
  const [yourRole, setYourRole] = useState<string>()
  const [memberInfo, setMemberInfo] = useState<Array<string>>()
  const [showInfo, setShowInfo] = useState<string>()
  
  

  async function get_nft_data(nft, resource) {
    

    const nonFungibleId = nft.non_fungible_ids[0];

        try {
          const response = await fetch(
            `https://pte01.radixdlt.com/non-fungible/${resource}${nonFungibleId}`
            );
            
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
        catch {
          get_nft_data(nft, resource)
        }
    
          

  }
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms))
  async function data() {
    
    const fetchData = async () => {
      try {
        
        setAccountAddress(await getAccountAddress());

        if ((accountAddress == undefined) || (accountAddress == 'Loading...')) {
          return
        } else {
          const response = await fetch(`https://pte01.radixdlt.com/component/${accountAddress}`);
  
          const component = await response.json();
    
          const my_resource = component.owned_resources;
  
          const admin = my_resource.find((nft) => nft.resource_address === ADMINBADGE);
          if (admin) {
            setYourRole("NeuRacle Admin")
          } 
          const validator = my_resource.find((nft) => nft.resource_address === VALIDATOR_BADGE);
          if (validator) {

            setYourRole("NeuRacle Validator");
            
            await get_nft_data(validator, VALIDATOR_BADGE);

            if (memberInfo == undefined) {
              return data()
            } else {setShowInfo(`Name: "${memberInfo![0]}" | Country: "${memberInfo![1]}" | Website: "${memberInfo![2]}" | Validator Address: "${memberInfo![3]}"`);
          alert(showInfo)}
          
          } 
          const user = my_resource.find((nft) => nft.resource_address === USER_BADGE) 
          if (user) {
            setYourRole("NeuRacle User")
            await get_nft_data(user, USER_BADGE);
            setShowInfo(`Your data source: "${memberInfo![0]}" | Account can use until epoch: "${memberInfo![1]}"`)
          } else {
            setYourRole("Visitor")
          }
        }
      } catch {
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
    Notiflix.Loading.pulse();
    const newpack: string = receipt.newPackages[0];
    success("Done!");
    info("Change the value", "New package address: " + newpack + ". <br/>Please add this on NEURACLE.tsx");
    Notiflix.Loading.remove()
  }
  async function become_admin() {

    const manifest = new ManifestBuilder()
      .callFunction(PACKAGE, 'NeuRacle', 'new', ['100u32', '1u64', 'Decimal("1")', 'Decimal("0.3")', '500u64', 'Decimal("0.0015")', 'Decimal("10")'])
      .callMethodWithAllResources(accountAddress!, 'deposit_batch')
      .build()
      .toString();
    
    const receipt = await signTransaction(manifest);
    Notiflix.Loading.pulse();
    if (receipt.status == 'Success') {
      success("Done!");
      info("Change the value", 'You have become NeuRacle Admin, please check your wallet detail in Pouch. You must edit the NEURACLE.tsx file with |New component: ' + receipt.newComponents[0]
      + '. <br/>|New Admin Badge: ' + receipt.newResources[0]
      + '. <br/>|New Validator Badge: ' + receipt.newResources[3]
      + '. <br/>|New User Badge: ' + receipt.newResources[4]
      + '. <br/>|New Neura Resource Address: ' + receipt.newResources[5])
    }
    else {
      failure_big("Failed", "Please try again: " + receipt.status)
    }
    Notiflix.Loading.remove()
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
              Notiflix.Loading.pulse();
          
              if (receipt.status == 'Success') {
                success("Done!");
                info("Change the value", "The address you provided has been assigned as NeuRacle Validator. New Validator Address: "
                + receipt.newComponents[0] 
                + ". This validator staker badge: "
                + receipt.newResources[1]
                + ". Please don't close this window until done!")
                    } else {
                failure_big("Failed", "Please try again: " + receipt.status);
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
    Notiflix.Loading.remove()
  }

  async function stake() {

  }
  
  function success(message: string) {Notiflix.Notify.success(message,{
    position: 'right-top',
    borderRadius: '10px',
    showOnlyTheLastOne: true
  })}

  async function info(title: string, message: string) {Notiflix.Report.info(
    title,
    message,
    'Ok',
    function(){
    },
    {
      width: "1000px",
      messageMaxLength: 1000,
    }
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
        Check your balance through <a
            className="App-link"
            href="https://plymth.github.io/pouch/"
            target="_blank"
            rel="noopener noreferrer"
          >Pouch</a>
        </p>
        <p></p>
        <div>
          Hello <a style={lightblue}>{yourRole}</a> with account: "<a style={lightgreen}>{accountAddress}</a>"
        </div>
        <p>
        <button type="button" onClick={() => {setAccountAddress('Loading...'), setYourRole('Loading...'), setShowInfo('Loading...')}}>
            Refresh your data
          </button>
        </p>
        <div>
          {showInfo}
        </div>
        <p>
        <button type="button" onClick={publish_package}>
            Publish package
          </button> | <button type="button" onClick={become_admin}>
            Become NeuRacle Admin
          </button> | <button type="button" onClick={assign_validators}>
            Assign a validator
          </button>
        </p>
      </header>
    </div>
  )
}

export default App
