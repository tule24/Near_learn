import { useState, useEffect } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import { wallet, contract } from './utils'
import Components from './components'


function App() {
  const [isSignedIn, setIsSignedIn] = useState(false)
  const [beneficiary, setBeneficiary] = useState("")
  const [donations, setDonations] = useState([])
  const [total, setTotal] = useState(0)
  const [noti, setNoti] = useState({ display: false, text: "" })

  useEffect(() => {
    const initData = async () => {
      const isSignIn = await wallet.startUp()
      setIsSignedIn(isSignIn)

      const beneficiary = await contract.get_beneficiary()
      setBeneficiary(beneficiary)

      const donations = await contract.get_lastest_donation()
      setDonations([...donations])

      const totalDonate = await contract.get_total_donate()
      setTotal(totalDonate)
    }
    initData()
  }, [])

  const handleSubmit = async (e) => {
    e.preventDefault();

    const { fieldset, donation } = e.target.elements
    fieldset.disabled = true

    try {
      await contract.donate(donation.value)

      // const urlParams = new URLSearchParams(window.location.search)
      // const txhash = urlParams.get("transactionHashes")

      // if (!txhash) {
      //   let res = await contract.get_donation_from_transaction(txhash)
      //   setNoti({ ...noti, display: true, text: res })

      //   setTimeout(() => {
      //     setNoti({ ...noti, display: false, text: "" })
      //   }, 5000)
      // }
    } catch (err) {
      alert(
        'Something went wrong! ' +
        'Maybe you need to sign out and back in? ' +
        'Check your browser console for more info.'
      )
      throw new Error(err)
    }
    const donations = await contract.get_lastest_donation()
    setDonations([...donations])

    donation.value = '0'
    fieldset.disabled = false;
  }

  return (
    <div className="App">
      <div className='p-5'>
        <div className="row">
          <div className="col-sm-8 pe-2 pe-sm-5">
            <Components.Donations donations={donations}/>
          </div>
          <div className="col-sm-4">
            <div className="donation-box mt-md-4">
              <div className="donation-box-head">
                <i className="logo" />
                <h4> Donate to <label htmlFor="beneficiary" data-behavior="beneficiary" style={{ color: '#0072CE', borderBottom: '2px solid #0072CE' }}>
                  {beneficiary}
                </label> 
                </h4>
                <br />
                <h4>{total} USD</h4>
              </div>
              <div className='d-grid gap-2 col-3 mx-auto my-2'>
              {isSignedIn
                ? <button className='btn btn-secondary' onClick={() => wallet.signOut()}> Log Out </button>
                : <button className='btn btn-secondary' onClick={() => wallet.signIn()}> Log In </button>}
              </div>
              {isSignedIn
                ? <Components.Form onSubmit={handleSubmit} accountId={wallet?.accountId}/>
                : <Components.SignIn />}

            </div>
          </div>
        </div>
        {noti.display &&
          <aside data-behavior="notification" className="bg-success p-2 text-white bg-opacity-75">
            Thank you! You have donated so far:
            <label htmlFor="donation-number" data-behavior="donation-so-far"> {noti.display} </label>Ⓝ
            <footer>
              <div>✔ Succeeded</div>
            </footer>
          </aside>}
      </div>
    </div>
  )
}

export default App
