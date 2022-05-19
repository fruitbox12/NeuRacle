import { useCallback, useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { DefaultApi, ManifestBuilder } from 'pte-sdk'
import { getAccountAddress, signTransaction, waitForAction } from 'pte-browser-extension-sdk'
import Notiflix from 'notiflix'
import { TESTER, ADMINBADGE, PACKAGE, NAR, COMPONENT, VALIDATOR_BADGE, USER_BADGE } from './NEURACLE'


function App() {
  const [accountAddress, setAccountAddress] = useState<string>()
  const lightgreen = { color: 'lightgreen' }
  const lightblue = { color: 'lightblue' }
  const [yourRole, setYourRole] = useState<string>()
  const [memberInfo, setMemberInfo] = useState<Array<string>>()
  const [tokenInfo, setTokenInfo] = useState<string>()
  const [stakerInfo, setStakerInfo] = useState<Array<Array<string>>>()

  function Show_each_staked_info(): JSX.Element {
    if (stakerInfo == undefined) {
      return <div>
        You haven't staked in any validator
      </div>
    }
    else {
      const list = stakerInfo.map((x) => {
        return (<li key={x[0].toString()}><div style={{ border: '3px solid cyan' }}><br />
          Validator Name: {x[1]}
          <br /><br />
          Current staked: {x[5] + " NAR"}
          <br /><br />
          Current unstaking: {x[2].replace(/^\D+|\D+$/g, "")}
          <br /><br />
          Estimated unstaking done in epoch: {x[3]}
          <br /><br />
          Avaiable for withdraw: {x[4].replace(/^\D+|\D+$/g, "")}
          <br /><br /></div></li>)
      });
      return (
        <div><br />
          {list}
        </div>
      )
    }
  }

  function Show_info() {
    if (memberInfo == undefined) return null
    if (yourRole == "NeuRacle Validator") {
      return <div style={{ border: '3px solid lightgreen' }}>
        Name: {memberInfo![0]} <br /> Country: {memberInfo![1]} <br /> Website: {memberInfo![2]} <br /> Address: {memberInfo![3]}
      </div>
    }
    else if (yourRole == "NeuRacle User") {
      return <div style={{ border: '3px solid lightblue' }}>
        Your data source: {memberInfo![0]} <br /> This account have access until epoch {memberInfo![1]}
      </div>
    }
    else return null
  }

  function Role_button() {
    if (yourRole == undefined) { return <div> Loading... </div> }
    else if (yourRole == 'NeuRacle Admin') {
      return <div>
        <button type="button" onClick={assign_validators}>
          Assign a validator
        </button>
      </div>
    }
    else if (yourRole == 'NeuRacle Validator') {
      return <div>
        <button type="button" onClick={function () { }}>
          Change fee
        </button> | <button type="button" onClick={function () { }}>
          Withdraw fee
        </button>
      </div>
    }
    else if (yourRole == 'NeuRacle User') {
      return <div>
        <button type="button" onClick={function () { }}>
          Show your data:
        </button>
      </div>
    }
    else if (yourRole == 'TESTER') {
      return <div><button type="button" onClick={publish_package}>
        Publish package
      </button> | <button type="button" onClick={become_admin}>
          Become NeuRacle Admin
        </button></div>
    }
    else return null
  }

  function Visitor_button() {
    if (tokenInfo == undefined) { return <div> Loading... </div> }
    return <div style={{ textAlign: "center" }}>
      Current NAR on your wallet: {tokenInfo}
      <br /><br />
      <button type="button" onClick={stake}>
        Stake
      </button> | <button type="button" onClick={function () { }}>
        Unstake
      </button> | <button type="button" onClick={function () { }}>
        Stop Unstake
      </button> | <button type="button" onClick={function () { }}>
        Withdraw
      </button>
      <br /><br />
    </div>
  }

  async function get_nft_data(non_fungible_ids: string, resource: string): Promise<Array<string>> {

    const response = await fetch(
      `https://pte01.radixdlt.com/non-fungible/${resource}${non_fungible_ids}`
    );

    let info: Array<string> = [];
    const nonFungibleData = await response.json();

    const data = JSON.parse(nonFungibleData.immutable_data).fields;

    data.forEach((x: { value: string }) => {
      info.push(x.value);
    });

    const data2 = JSON.parse(nonFungibleData.mutable_data).fields
    data2.forEach((x: { value: string }) => {
      info.push(x.value);
    });
    return info

  }

  async function data() {

    async function fetchData(): Promise<any> {
      try {

        setAccountAddress(await getAccountAddress())

        if ((accountAddress == undefined) || (accountAddress == 'Loading...')) {
          return
        } else {
          const response = await fetch(`https://pte01.radixdlt.com/component/${accountAddress}`)

          const component = await response.json()

          const my_resource = component.owned_resources

          const token_nar = my_resource.find((resource: { resource_address: string} ) => {
            return resource.resource_address === NAR
          })

          if (token_nar) {
            setTokenInfo(token_nar.amount)
          } else { setTokenInfo('0')} ;

          const admin = my_resource.find((resource: { resource_address: string; non_fungible_ids: string[]} ) => {
            return resource.resource_address === ADMINBADGE
          })
          const validator = my_resource.find((resource: { resource_address: string; non_fungible_ids: string[]} ) => {
            return resource.resource_address === VALIDATOR_BADGE
          })
          const user = my_resource.find((resource: { resource_address: string; non_fungible_ids: string[]} ) => {
            return resource.resource_address === USER_BADGE
          })
          if (admin) {
            setYourRole("NeuRacle Admin")
          }

          else if (validator) {

            setYourRole("NeuRacle Validator")

            setMemberInfo(await get_nft_data(validator.non_fungible_ids[0], VALIDATOR_BADGE))

          }

          else if (user) {
            setYourRole("NeuRacle User")
            setMemberInfo(await get_nft_data(user.non_fungible_ids[0], USER_BADGE))
     
          }
          else if (accountAddress == TESTER) {
            setYourRole("TESTER")
          } else { setYourRole("Visitor")} 

          const staker = await my_resource.filter((resource: { name: string} ) => resource.name === "NeuRacle staker Badge")

          if (staker.length) {

            const staker_infos: string[][] = []

            staker.forEach(async (x: { resource_address?: any; non_fungible_ids?: string[]} ) => {
              const staker_info_same_address = await get_nft_data(x.non_fungible_ids![0], x.resource_address)
              var total_amount: number = 0;
              x.non_fungible_ids?.forEach(async (y) => {
                const response = await fetch(
                  `https://pte01.radixdlt.com/component/${staker_info_same_address[0]}`
                )
                const parseData = await response.json()
                const parseNonFungibleId = JSON.parse(parseData.state).fields[0].elements
                const idx = parseNonFungibleId.findIndex((nonfgb: { value: string} ) => {
                  return nonfgb.value === `NonFungibleId("${y}")`
                })
                const staked_amount: number = parseFloat(parseNonFungibleId[idx + 1].value.replace(/^\D+|\D+$/g, ""));
                total_amount = total_amount + staked_amount
              }
              )
              staker_info_same_address.push('' + total_amount);
              staker_infos.push(staker_info_same_address);
              console.log("response>>>>>>>>>>", staker_infos);
            })
            setStakerInfo(staker_infos)
          } else {
            setStakerInfo(undefined)
          }

        }
      } catch {
        fetchData()
      }
    }

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
      info("Change the value", 'You have become NeuRacle Admin, please check your wallet detail in Pouch. You must edit the NEURACLE.tsx file with    |New component: ' + receipt.newComponents[0]
        + '.   |New Admin Badge: ' + receipt.newResources[0]
        + '.   |New Validator Badge: ' + receipt.newResources[3]
        + '.   |New User Badge: ' + receipt.newResources[4]
        + '.   |New Neura Resource Address: ' + receipt.newResources[5])
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
                  .takeFromWorktop(ADMINBADGE, 'bucket')
                  .createProofFromBucket('bucket', 'admin_proof')
                  .pushToAuthZone('admin_proof')
                  .callMethod(COMPONENT, 'create_new_validator_node', [`"${validator_name}" "${validator_country}" "${validator_website}" Decimal("${validator_fee}")`])
                  .takeFromWorktopByAmount(1, VALIDATOR_BADGE, 'val1')
                  .callMethod(validator_account_address, 'deposit', ['Bucket("val1")'])
                  .callMethodWithAllResources(accountAddress!, 'deposit_batch')
                  .build()
                  .toString();

                const receipt = await signTransaction(manifest);
                Notiflix.Loading.pulse();

                if (receipt.status == 'Success') {
                  success_big("Done!", "The address you provided has been assigned as NeuRacle Validator.");
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
    const result = prompt("Validator Address");
    if (result == null) {
      return
    } else {
      const validator_address: string = result;
      const result2 = prompt("How much NAR want to stake to this validator?");
      if (result2 == null) {
        return
      } else {
        const amount: number = parseFloat(result2);
        const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, amount, NAR)
          .takeFromWorktop(NAR, 'bucket')
          .callMethod(validator_address, 'stake', ['Bucket("bucket")'])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);
        Notiflix.Loading.pulse();

        if (receipt.status == 'Success') {
          success_big("Done!", "You have staked " + amount + " NAR into validator address " + validator_address);
        } else {
          failure_big("Failed", "Please try again: " + receipt.status);
        }
      }
      Notiflix.Loading.remove()
    }
  }
  async function un_stake() {
    const result = prompt("Validator Address");
    if (result == null) {
      return
    } else {
      const validator_address: string = result;
      const result2 = prompt("How much NAR want to unstake from this validator?");
      if (result2 == null) {
        return
      } else {
        const amount: number = parseFloat(result2);
        const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, 1, NAR)
          .takeFromWorktop(NAR, 'bucket')
          .callMethod(validator_address, 'stake', ['Bucket("bucket")'])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);
        Notiflix.Loading.pulse();

        if (receipt.status == 'Success') {
          success_big("Done!", "You have staked " + amount + " NAR into validator address " + validator_address);
        } else {
          failure_big("Failed", "Please try again: " + receipt.status);
        }
      }
      Notiflix.Loading.remove()
    }
  }
  async function stop_unstake() {
    const result = prompt("Validator Address");
    if (result == null) {
      return
    } else {
      const validator_address: string = result;
      const result2 = prompt("How much NAR want to stake to this validator?");
      if (result2 == null) {
        return
      } else {
        const amount: number = parseFloat(result2);
        const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, amount, NAR)
          .takeFromWorktop(NAR, 'bucket')
          .callMethod(validator_address, 'stake', ['Bucket("bucket")'])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);
        Notiflix.Loading.pulse();

        if (receipt.status == 'Success') {
          success_big("Done!", "You have staked " + amount + " NAR into validator address " + validator_address);
        } else {
          failure_big("Failed", "Please try again: " + receipt.status);
        }
      }
      Notiflix.Loading.remove()
    }
  }
  async function withdraw() {
    const result = prompt("Validator Address");
    if (result == null) {
      return
    } else {
      const validator_address: string = result;
      const result2 = prompt("How much NAR want to stake to this validator?");
      if (result2 == null) {
        return
      } else {
        const amount: number = parseFloat(result2);
        const manifest = new ManifestBuilder()
          .withdrawFromAccountByAmount(accountAddress!, amount, NAR)
          .takeFromWorktop(NAR, 'bucket')
          .callMethod(validator_address, 'stake', ['Bucket("bucket")'])
          .callMethodWithAllResources(accountAddress!, 'deposit_batch')
          .build()
          .toString();

        const receipt = await signTransaction(manifest);
        Notiflix.Loading.pulse();

        if (receipt.status == 'Success') {
          success_big("Done!", "You have staked " + amount + " NAR into validator address " + validator_address);
        } else {
          failure_big("Failed", "Please try again: " + receipt.status);
        }
      }
      Notiflix.Loading.remove()
    }
  }

  function success_big(title: string, message: string) {
    Notiflix.Report.success(
      title,
      message,
      'Ok',
    )
  }

  function success(message: string) {
    Notiflix.Notify.success(message, {
      position: 'right-top',
      borderRadius: '10px',
      showOnlyTheLastOne: true
    })
  }

  async function info(title: string, message: string) {
    Notiflix.Report.info(
      title,
      message,
      'Ok',
      function () {
      },
      {
        width: "1000px",
        messageMaxLength: 1000,
      }
    )
  }

  function failure(message: string) {
    Notiflix.Notify.failure(message, {
      position: 'right-top',
      borderRadius: '10px',
      showOnlyTheLastOne: true
    })
  }

  function failure_big(title: string, message: string) {
    Notiflix.Report.failure(
      title,
      message,
      'Ok',
    )
  }

  useEffect(() => {
    setTimeout(() => {
       data()
      }, 100);
  }, [accountAddress, yourRole, stakerInfo]);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Welcome to NeuRacle!</p>
        <div>
          Install <a
            className="App-link"
            href="https://docs.radixdlt.com/main/scrypto/public-test-environment/pte-getting-started.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Radix Babylon PTE
          </a> PTE to getting started.
        </div>
        <p>
          Check your balance through <a
            className="App-link"
            href="https://plymth.github.io/pouch/"
            target="_blank"
            rel="noopener noreferrer"
          >Pouch</a>
        </p>
        <p>
          Hello <a style={lightblue}>{yourRole}</a> with account: "<a style={lightgreen}>{accountAddress}</a>"
        </p>
        <div >
          <Show_info />
        </div>
        <br />
        <button type="button" onClick={() => { setAccountAddress('Loading...'), setYourRole('Loading...'), setMemberInfo(undefined), setTokenInfo(undefined) }}>
          Refresh your data
        </button>
        <br />
        <div>
          <Role_button />
        </div>
        <br />
        <Visitor_button />
      <Show_each_staked_info />
      </header>
    </div>
  )
}

export default App
