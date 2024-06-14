import { Injectable, OnModuleInit } from '@nestjs/common'
import { ApiPromise } from '@polkadot/api'
import { SubmittableExtrinsic } from '@polkadot/api-base/types'
import { Abi } from '@polkadot/api-contract'
import { HttpProvider } from '@polkadot/rpc-provider'
import { ContractExecResult, WeightV2 } from '@polkadot/types/interfaces'
import { BN } from '@polkadot/util'

import { PriceDataPayload } from '@shared/price'
import { SubstrateWalletService } from '@shared/wallet/substrate/wallet.service'

import { SubstrateConfigService } from '@substrate/config/substrate.config'

import ABI from './resources/abi/ink_standard_reference.json'

@Injectable()
export class SubstrateService implements OnModuleInit {
  public network: string
  private api: ApiPromise
  private abi: Abi
  private maxGasLimit: WeightV2

  constructor(
    private configService: SubstrateConfigService,
    private walletService: SubstrateWalletService,
  ) {}

  async onModuleInit() {
    this.network = this.configService.config.network

    const provider = new HttpProvider(this.configService.config.rpcURL)
    this.api = new ApiPromise({ provider })
    await this.api.isReady
    this.abi = new Abi(ABI)
    this.maxGasLimit = this.api.registry.createType('WeightV2', {
      refTime: new BN(this.configService.config.maxRefTime),
      proofSize: new BN(this.configService.config.maxProofSize),
    }) as WeightV2
  }

  async estimateGas(sender: Uint8Array, calldata: Uint8Array): Promise<WeightV2> {
    const { gasRequired } = await this.api.call.contractsApi.call<ContractExecResult>(
      sender,
      this.configService.config.contractAddress,
      0,
      this.maxGasLimit,
      null,
      calldata,
    )
    return gasRequired
  }

  async sign(
    sender: string,
    prices: PriceDataPayload,
    nonce: number,
    tip: number,
  ): Promise<SubmittableExtrinsic<'promise'>> {
    const calldata = this.abi
      .findMessage('relay')
      .toU8a([prices.prices.map(({ symbol, rate }) => [symbol, parseInt(rate)]), prices.resolveTime, prices.requestId])

    const keypair = this.walletService.getKeyPair(sender)

    // Estimate gas
    const estimatedGas = await this.estimateGas(keypair.publicKey, calldata)

    // Build unsigned transaction
    const tx = this.api.tx.contracts.call(this.configService.config.contractAddress, 0, estimatedGas, null, calldata)

    // Sign transaction with keypair
    return tx.signAsync(keypair, { nonce, tip })
  }
}
