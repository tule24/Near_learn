import { Contract } from './near-interface'
import { Wallet } from './near-wallet'
import { CONTRACT_ID } from './const'

const wallet = new Wallet({createAccessKeyFor: CONTRACT_ID})
const contract = new Contract({contractId: CONTRACT_ID, walletToUse: wallet})
const near2usd = async (amount) => {
    let data = await fetch("https://api.coingecko.com/api/v3/simple/price?ids=near&vs_currencies=usd").then(res => res.json())
    const near2usd = data['near']['usd']
    console.log(`near2usd ${near2usd}`)
    const amount_in_near = amount * near2usd
    const rounded_two_decimals = Math.round(amount_in_near * 1000) / 1000
    return rounded_two_decimals
}
export {wallet, contract, near2usd}