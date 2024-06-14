import { Injectable, OnModuleInit } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'

import { SubstrateConfig } from '@substrate/config/substrate-config.type'

@Injectable()
export class SubstrateConfigService implements OnModuleInit {
  config: SubstrateConfig

  constructor(private configService: ConfigService) { }

  onModuleInit() {
    this.config = {
      network: this.configService.getOrThrow('chain.network'),
      ss58Format: this.configService.getOrThrow('chain.ss58Format'),
      rpcURL: this.configService.getOrThrow('chain.rpcUrl'),
      maxRefTime: this.configService.getOrThrow('chain.maxRefTime'),
      maxProofSize: this.configService.getOrThrow('chain.maxProofSize'),
      account: this.configService.getOrThrow(`chain.account`),
      contractAddress: this.configService.getOrThrow(`chain.contractAddress`),
    }
  }
}
