import React from 'react'

function Form({ onSubmit, accountId }) {
    return (
        <div className="signed-in-flow px-2">
            <h5>Your account: {accountId}</h5>
            <form onSubmit={onSubmit}>
                <fieldset id="fieldset">
                    <label htmlFor="donation" className="form-label">
                        Donation amount (in Ⓝ)
                    </label>
                    <div className="input-group">
                        <input type='number' min='0' defaultValue={'0'} id="donation" step='0.01' className="form-control" data-behavior="donation" />
                        <span className="input-group-text">Ⓝ</span>
                        <button className="btn btn-primary" type='submit'>Donate</button>
                    </div>
                </fieldset>
            </form>
            <button className="link signed-in-flow" style={{ display: 'none', float: 'right' }} id="sign-out-button">
                Sign out
            </button>
        </div>
    )
}

export default Form
