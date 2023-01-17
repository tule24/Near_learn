import { useState, useEffect } from 'react'
import './App.css'
import { wallet, contract } from './utils'
import View from './components'

function App() {
  const [isSignedIn, setIsSignedIn] = useState(true)
  const [res, setRes] = useState('Heads')
  const [guess, setGuess] = useState('')
  const [point, setPoint] = useState(0)
  const [msg, setMsg] = useState('Start and enjoy the game!')

  const getPoint = async () => {
    const point = await contract.get_point()
    setPoint(point)
  }

  useEffect(() => {
    const initData = async () => {
      const isSignIn = await wallet.startUp()
      setIsSignedIn(isSignIn)
      await getPoint()
    }
    initData()
  }, [])

  const runGame = async () => {
    const res = await contract.flip_coin(guess)
    setRes(res)
    if(res === guess) {
      setMsg('You were right, you win a point!')
    } else {
      setMsg('You were wrong, you lose a point!')
    }
    await getPoint()
  }

  return (
    <div className="App">
      <div className='p-3'>
        {isSignedIn
          ? <div className='d-flex align-items-center'>
            <p><strong>Your account: </strong> {wallet?.accountId}</p>
            <button className='btn btn-secondary btn-sm mx-3'> Log Out </button>
          </div>
          : <button className='btn btn-secondary'> Log In </button>
        }
      </div>
      {isSignedIn
        ? <View.Signed msg={msg} point={point} res={res} run={runGame} setGuess={setGuess} guess={guess} />
        : <View.NotSigned />
      }
    </div>
  )
}

export default App
