import { Injectable, OnModuleInit } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Keyring } from '@polkadot/keyring'
import { IKeyringPair } from '@polkadot/types/types'
import { cryptoWaitReady } from '@polkadot/util-crypto'

const defaultAddresses = 10

@Injectable()
export class SubstrateWalletService implements OnModuleInit {
  private keyring: Keyring

  constructor(private configService: ConfigService) { }

  async onModuleInit() {
    await this.generateKeyPairs()
  }

  private async generateKeyPairs(): Promise<void> {
    const ss58Format = this.configService.getOrThrow('chain.ss58Format')
    this.keyring = new Keyring({ type: 'sr25519', ss58Format })
    const mnemonic = this.configService.getOrThrow('MNEMONIC')

    await cryptoWaitReady()

    for (let i = 0; i < defaultAddresses; i++) {
      this.keyring.addFromMnemonic(`${mnemonic}//account//0/${i}`)
    }
  }

  getKeyPair(sender: string): IKeyringPair {
    return this.keyring.getPair(sender)
  }
}
