import React from 'react'

function Signed({msg, point, res, run, setGuess, guess}) {
  return (
    <div className='d-flex justify-content-center align-items-center flex-column'>
      <img src={res === 'Heads' ? './heads.png' : './tails.png'} alt="Coin" width={500}/>
      <div className=' py-2 text-center'>
        <h3>What do you think is coming next ?</h3>
        <h5><strong>Your guess:</strong><span>  </span>{guess}</h5>
        <button className='btn btn-success mx-5' onClick={() => setGuess('Heads')}>Heads</button>
        <button className='btn btn-primary mx-5' onClick={() => setGuess('Tails')}>Tails</button>
        <br />
        <button className='btn btn-danger mt-3' onClick={() => run(guess)}>Run</button>
      </div>
      <p><strong>Message: </strong> {msg} </p>
      <p><strong>Your point: </strong> {point} </p>
    </div>
  )
}

export default Signed