import { CONTRACT_ID } from './const'

export class Contract {
    constructor({ contractId, walletToUse }) {
        this.contractId = contractId
        this.wallet = walletToUse
    }

    async get_point() {
        return await this.wallet.viewMethod({
            contractId: CONTRACT_ID,
            method: "get_points_by_account",
            args: {account: this.wallet.accountId}
        })
    }

    async flip_coin(side) {
        return await this.wallet.callMethod({
            contractId: CONTRACT_ID,
            method: "flip_coin",
            args: {player_guess: side}
        })
    }

}
