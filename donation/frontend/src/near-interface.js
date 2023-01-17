/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */
import { CONTRACT_ID } from './const'
import { utils } from 'near-api-js' 
import { near2usd } from './utils'

export class Contract {
    constructor({ contractId, walletToUse }) {
        this.contractId = contractId
        this.wallet = walletToUse
    }

    async get_beneficiary() {
        return await this.wallet.viewMethod({
            contractId: CONTRACT_ID,
            method: "get_beneficiary"
        })
    }

    async get_total_donate() {
        let res = await this.wallet.viewMethod({
            contractId: CONTRACT_ID,
            method: "total_donate"
        })

        res = utils.format.formatNearAmount(res)
        res = Number(res);
        console.log(`number ${res}`)
        res = near2usd(res);
        return res
    }

    async get_lastest_donation() {
        const number_of_donors = await this.wallet.viewMethod({
            contractId: CONTRACT_ID,
            method: "number_of_donors"
        })

        const min = number_of_donors > 10 ? number_of_donors - 10 : 0

        let donations = await this.wallet.viewMethod({
            contractId: CONTRACT_ID,
            method: "get_donations",
            args: {
                from_index: min.toString(),
                limit: number_of_donors 
            }
        })

        donations.forEach(ele => {
            ele.total_amount = utils.format.formatNearAmount(ele.total_amount)
        })

        return donations
    }

    async get_donation_from_transaction(txHash) {
        const donation_amount = await this.wallet.getTransactionResult(txHash)
        return utils.format.formatNearAmount(donation_amount)
    }

    async donate(amount) {
        let deposit = utils.format.parseNearAmount(amount.toString())
        let res = await this.wallet.callMethod({
            contractId: CONTRACT_ID,
            method: "donate",
            deposit
        })
        return res
    }

}
